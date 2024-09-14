use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use sha3::{Digest, Keccak256};

use crate::error::RedstoneError;
use crate::instructions::constants::*;
use crate::state::*;

pub type U256 = [u8; 32];

pub fn u256_from_slice(bytes: &[u8]) -> U256 {
    let mut array = [0u8; 32];
    let len = if bytes.len() > 32 { 32 } else { bytes.len() };
    array[..len].copy_from_slice(&bytes[..len]);
    array
}

pub trait Trim<T>
where
    Self: Sized,
{
    fn trim_end(&mut self, len: usize) -> T;
}

impl Trim<Vec<u8>> for Vec<u8> {
    fn trim_end(&mut self, len: usize) -> Self {
        if len >= self.len() {
            std::mem::take(self)
        } else {
            self.split_off(self.len() - len)
        }
    }
}

pub fn verify_redstone_marker(bytes: &[u8]) -> Result<()> {
    if bytes.len() < REDSTONE_MARKER.len()
        || bytes[bytes.len() - REDSTONE_MARKER.len()..] != REDSTONE_MARKER
    {
        return Err(RedstoneError::InvalidRedstoneMarker.into());
    }
    Ok(())
}

pub fn parse_payload(
    bytes: &mut Vec<u8>,
    data_packages_count: usize,
) -> Result<Payload> {
    let mut data_packages = Vec::with_capacity(data_packages_count);
    for _ in 0..data_packages_count {
        let data_package = parse_data_package(bytes)?;
        data_packages.push(data_package);
    }
    Ok(Payload { data_packages })
}

fn parse_data_package(payload: &mut Vec<u8>) -> Result<DataPackage> {
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

pub fn trim_data_point_count(payload: &mut Vec<u8>) -> usize {
    let data_point_count = payload.trim_end(DATA_POINTS_COUNT_BS);
    usize::from_le_bytes(data_point_count.try_into().unwrap())
}

pub fn trim_data_point_value_size(payload: &mut Vec<u8>) -> usize {
    let value_size = payload.trim_end(DATA_POINT_VALUE_BYTE_SIZE_BS);
    usize::from_le_bytes(value_size.try_into().unwrap())
}

pub fn trim_timestamp(payload: &mut Vec<u8>) -> u64 {
    let timestamp = payload.trim_end(TIMESTAMP_BS);
    u64::from_le_bytes(timestamp.try_into().unwrap())
}

pub fn parse_data_points(
    payload: &mut Vec<u8>,
    count: usize,
    value_size: usize,
) -> Vec<DataPoint> {
    let mut data_points = Vec::new();

    for _ in 0..count {
        let data_point = parse_data_point(payload, value_size);
        data_points.push(data_point);
    }

    data_points
}

fn parse_data_point(payload: &mut Vec<u8>, value_size: usize) -> DataPoint {
    let value = payload.trim_end(value_size);
    let feed_id = payload.trim_end(DATA_FEED_ID_BS);
    let feed_id = u256_from_slice(&feed_id);

    DataPoint {
        value: u128::from_le_bytes(value.try_into().unwrap()),
        feed_id,
    }
}

pub fn recover_address(message: &[u8], signature: &[u8]) -> Result<Vec<u8>> {
    let recovery_byte = signature[64];
    let msg_hash = keccak256(message);
    let res = secp256k1_recover(&msg_hash, recovery_byte, &signature[..64]);
    match res {
        Ok(pubkey) => {
            let key_hash = keccak256(&pubkey.to_bytes()[1..]);
            Ok(key_hash[12..].to_vec())
        }
        Err(_) => Err(RedstoneError::InvalidSignature.into()),
    }
}

pub fn verify_data_packages(
    payload: &Payload,
    config: &Config,
) -> Result<()> {
    for package in &payload.data_packages {
        verify_timestamp(package.timestamp, config.block_timestamp)?;
        // Add more verifications as needed
    }
    verify_signer_count(
        &payload.data_packages,
        config.signer_count_threshold,
    )?;
    Ok(())
}

pub fn verify_timestamp(timestamp: u64, block_timestamp: u64) -> Result<()> {
    if timestamp + MAX_TIMESTAMP_DELAY_MS < block_timestamp {
        return Err(RedstoneError::TimestampTooOld.into());
    }
    if timestamp > block_timestamp + MAX_TIMESTAMP_AHEAD_MS {
        return Err(RedstoneError::TimestampTooFuture.into());
    }
    Ok(())
}

pub fn verify_signer_count(
    data_packages: &[DataPackage],
    threshold: u8,
) -> Result<()> {
    let unique_signers: std::collections::HashSet<_> = data_packages
        .iter()
        .map(|dp| dp.signer_address.clone())
        .collect();
    if unique_signers.len() < threshold as usize {
        return Err(RedstoneError::InsufficientSignerCount.into());
    }
    Ok(())
}

pub fn keccak256(data: &[u8]) -> Box<[u8]> {
    Keccak256::new_with_prefix(data)
        .finalize()
        .as_slice()
        .into()
}
