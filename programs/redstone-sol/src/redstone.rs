use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

use crate::constants::*;
use crate::error::RedstoneError;
use crate::state::*;
use crate::util::*;

pub fn verify_redstone_marker(bytes: &[u8]) -> Result<()> {
    if bytes.len() < REDSTONE_MARKER.len()
        || bytes[bytes.len() - REDSTONE_MARKER.len()..] != REDSTONE_MARKER
    {
        return Err(RedstoneError::InvalidRedstoneMarker.into());
    }
    Ok(())
}

pub fn parse_raw_payload(bytes: &mut Vec<u8>) -> Result<Payload> {
    // redstone marker is verifed in top-level method, just trimming here
    trim_redstone_marker(bytes);
    trim_payload(bytes)
}

fn trim_data_packages(
    payload: &mut Vec<u8>,
    count: usize,
) -> Result<Vec<DataPackage>> {
    let mut data_packages = Vec::with_capacity(count);
    for _ in 0..count {
        let data_package = trim_data_package(payload)?;
        data_packages.push(data_package);
    }
    Ok(data_packages)
}

fn trim_data_package(payload: &mut Vec<u8>) -> Result<DataPackage> {
    let signature = payload.trim_end(SIGNATURE_BS);
    let mut tmp = payload.clone();

    let data_point_count = trim_data_point_count(payload);
    let value_size = trim_data_point_value_size(payload);
    let timestamp = trim_timestamp(payload);
    let size = data_point_count * (value_size + DATA_FEED_ID_BS)
        + DATA_POINT_VALUE_BYTE_SIZE_BS
        + TIMESTAMP_BS
        + DATA_POINTS_COUNT_BS;

    let signable_bytes = tmp.trim_end(size);
    let signer_address = recover_address(&signable_bytes, &signature)?;

    let data_points =
        parse_data_points(payload, data_point_count, value_size);

    Ok(DataPackage {
        data_points,
        timestamp,
        signer_address,
    })
}

pub fn trim_payload(payload: &mut Vec<u8>) -> Result<Payload> {
    let data_packages_count = trim_metadata(payload);
    let data_packages = trim_data_packages(payload, data_packages_count)?;

    Ok(Payload { data_packages })
}

pub fn trim_metadata(payload: &mut Vec<u8>) -> usize {
    let unsigned_metadata_size =
        payload.trim_end(UNSIGNED_METADATA_BYTE_SIZE_BS);
    let unsigned_metadata_size = usize::from_bytes(&unsigned_metadata_size);
    let _: Vec<u8> = payload.trim_end(unsigned_metadata_size);

    let package_count = payload.trim_end(DATA_PACKAGES_COUNT_BS);
    usize::from_bytes(&package_count)
}

pub fn trim_redstone_marker(payload: &mut Vec<u8>) -> [u8; 9] {
    let redstone_marker = payload.trim_end(REDSTONE_MARKER_BS);
    redstone_marker.try_into().unwrap()
}

pub fn trim_data_point_count(payload: &mut Vec<u8>) -> usize {
    let data_point_count = payload.trim_end(DATA_POINTS_COUNT_BS);
    usize::from_bytes(&data_point_count)
}

pub fn trim_data_point_value_size(payload: &mut Vec<u8>) -> usize {
    let value_size = payload.trim_end(DATA_POINT_VALUE_BYTE_SIZE_BS);
    usize::from_bytes(&value_size)
}

pub fn trim_timestamp(payload: &mut Vec<u8>) -> u64 {
    let timestamp = payload.trim_end(TIMESTAMP_BS);
    u64::from_bytes(&timestamp)
}

pub fn parse_data_points(
    payload: &mut Vec<u8>,
    count: usize,
    value_size: usize,
) -> Vec<DataPoint> {
    let mut data_points = Vec::with_capacity(count);

    for _ in 0..count {
        let data_point = parse_data_point(payload, value_size);
        data_points.push(data_point);
    }

    data_points
}

fn parse_data_point(payload: &mut Vec<u8>, value_size: usize) -> DataPoint {
    let value = payload.trim_end(value_size);
    let feed_id = payload.trim_end(DATA_FEED_ID_BS);
    DataPoint {
        value: value.try_into().unwrap(),
        feed_id: feed_id.try_into().unwrap(),
    }
}

pub fn recover_address(
    message: &[u8],
    signature: &[u8],
) -> Result<SignerAddress> {
    let recovery_byte = signature[64];
    let recovery_id =
        recovery_byte - (if recovery_byte >= 27 { 27 } else { 0 });
    let msg_hash = keccak256(message);
    let res = secp256k1_recover(&msg_hash, recovery_id, &signature[..64]);
    match res {
        Ok(pubkey) => {
            let key_hash = keccak256(&pubkey.to_bytes()[1..]);
            Ok(key_hash[12..].try_into().unwrap())
        }
        Err(_e) => {
            #[cfg(feature = "dev")]
            msg!("Invalid signature: {:?}: {:?}", signature, _e);
            Err(RedstoneError::InvalidSignature.into())
        }
    }
}

pub fn verify_data_packages(
    payload: &Payload,
    config: &Config,
) -> Result<()> {
    for package in &payload.data_packages {
        verify_timestamp(package.timestamp, config)?;
    }
    verify_signer_count(
        &payload.data_packages,
        config.config_account.signer_count_threshold,
        &config.config_account.signers,
    )?;
    Ok(())
}

pub fn verify_timestamp(timestamp: u64, config: &Config) -> Result<()> {
    if timestamp + config.config_account.max_timestamp_delay_ms
        < config.block_timestamp
    {
        #[cfg(feature = "dev")]
        msg!(
            "Timestamp: {} + {} < {}",
            timestamp,
            config.config_account.max_timestamp_delay_ms,
            config.block_timestamp
        );
        return Err(RedstoneError::TimestampTooOld.into());
    }
    if timestamp
        > config.block_timestamp
            + config.config_account.max_timestamp_ahead_ms
    {
        #[cfg(feature = "dev")]
        msg!(
            "Timestamp: {} > {} + {}",
            timestamp,
            config.block_timestamp,
            config.config_account.max_timestamp_ahead_ms
        );
        return Err(RedstoneError::TimestampTooFuture.into());
    }
    Ok(())
}

pub fn verify_signer_count(
    data_packages: &[DataPackage],
    threshold: u8,
    signers: &[SignerAddress],
) -> Result<()> {
    let unique_signers: HashSet<SignerAddress> =
        HashSet::from_iter(signers.iter().copied());
    let mut count: u8 = 0;
    for package in data_packages {
        if unique_signers.contains(&package.signer_address) {
            count += 1;
        }
        if count >= threshold {
            return Ok(());
        }
    }
    Err(RedstoneError::InsufficientSignerCount.into())
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    anchor_lang::solana_program::keccak::hash(data).to_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recover_address() {
        let signature = [
            62, 70, 170, 189, 206, 18, 147, 212, 185, 107, 170, 67, 23, 8,
            191, 160, 165, 172, 65, 237, 78, 237, 132, 1, 251, 9, 11, 217,
            135, 193, 97, 192, 9, 179, 221, 33, 49, 97, 126, 103, 59, 54, 25,
            253, 28, 26, 68, 198, 62, 38, 239, 210, 227, 184, 56, 5, 92, 52,
            13, 37, 49, 219, 63, 253, 28,
        ];

        let message = [
            66, 65, 76, 95, 115, 65, 86, 65, 88, 95, 65, 86, 65, 88, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            172, 164, 188, 52, 1, 146, 166, 216, 247, 144, 0, 0, 0, 32, 0, 0,
            1,
        ];

        let address = super::recover_address(&message, &signature).unwrap();
        println!("{:?}", address);
        SIGNERS.iter().find(|&x| *x == address).unwrap();
    }
}
