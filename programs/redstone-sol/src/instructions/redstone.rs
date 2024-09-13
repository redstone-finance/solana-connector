use anchor_lang::prelude::*;
use sha3::{Digest, Keccak256};

use crate::error::RedstoneError;
use crate::state::*;

// Constants (moved from processor.rs)
pub const REDSTONE_MARKER: [u8; 9] = [0, 0, 2, 237, 87, 1, 30, 0, 0];
pub const DATA_FEED_ID_BS: usize = 32;
pub const SIGNATURE_BS: usize = 65;
pub const TIMESTAMP_BS: usize = 6;
pub const DATA_POINTS_COUNT_BS: usize = 3;
pub const DATA_POINT_VALUE_BYTE_SIZE_BS: usize = 4;
pub const MAX_TIMESTAMP_DELAY_MS: u64 = 15 * 60 * 1000; // 15 minutes
pub const MAX_TIMESTAMP_AHEAD_MS: u64 = 3 * 60 * 1000; // 3 minutes

pub fn verify_redstone_marker(bytes: &[u8]) -> Result<()> {
    if bytes.len() < REDSTONE_MARKER.len()
        || &bytes[bytes.len() - REDSTONE_MARKER.len()..] != REDSTONE_MARKER
    {
        return Err(RedstoneError::InvalidRedstoneMarker.into());
    }
    Ok(())
}

pub fn extract_unsigned_metadata_size(bytes: &mut Vec<u8>) -> Result<usize> {
    // Implementation for extracting unsigned metadata size
    // This is similar to _extractByteSizeOfUnsignedMetadata in Ethereum version
    unimplemented!()
}

pub fn extract_data_packages_count(bytes: &mut Vec<u8>) -> Result<usize> {
    // Implementation for extracting data packages count
    // This is similar to _extractDataPackagesCountFromCalldata in Ethereum version
    unimplemented!()
}

pub fn parse_payload(bytes: &mut Vec<u8>, data_packages_count: usize) -> Result<Payload> {
    let mut data_packages = Vec::with_capacity(data_packages_count);
    for _ in 0..data_packages_count {
        let data_package = parse_data_package(bytes)?;
        data_packages.push(data_package);
    }
    Ok(Payload { data_packages })
}

pub fn parse_data_package(bytes: &mut Vec<u8>) -> Result<DataPackage> {
    let (data_points_count, value_byte_size) = extract_data_points_details(bytes)?;
    let timestamp = extract_timestamp(bytes)?;
    let signature = extract_signature(bytes)?;
    let data_points = parse_data_points(bytes, data_points_count, value_byte_size)?;
    let signer_address = recover_signer_address(bytes, &signature)?;

    Ok(DataPackage {
        signer_address,
        timestamp,
        data_points,
    })
}

pub fn extract_data_points_details(bytes: &[u8]) -> Result<(usize, usize)> {
    // Implementation for extracting data points details
    // This is similar to _extractDataPointsDetailsForDataPackage in Ethereum version
    unimplemented!()
}

pub fn extract_timestamp(bytes: &mut Vec<u8>) -> Result<u64> {
    // Implementation for extracting timestamp
    unimplemented!()
}

pub fn extract_signature(bytes: &mut Vec<u8>) -> Result<[u8; 65]> {
    // Implementation for extracting signature
    unimplemented!()
}

pub fn parse_data_points(
    bytes: &mut Vec<u8>,
    count: usize,
    value_size: usize,
) -> Result<Vec<DataPoint>> {
    let mut data_points = Vec::with_capacity(count);
    for _ in 0..count {
        let (feed_id, value) = extract_data_point_value_and_feed_id(bytes, value_size)?;
        data_points.push(DataPoint { feed_id, value });
    }
    Ok(data_points)
}

pub fn extract_data_point_value_and_feed_id(
    bytes: &[u8],
    value_size: usize,
) -> Result<([u8; 32], u128)> {
    // Implementation for extracting data point value and feed ID
    // This is similar to _extractDataPointValueAndDataFeedId in Ethereum version
    unimplemented!()
}

pub fn recover_signer_address(message: &[u8], signature: &[u8]) -> Result<[u8; 20]> {
    // Implementation for recovering signer address
    // Note: This might need adaptation for Solana's signature scheme
    unimplemented!()
}

pub fn verify_data_packages(payload: &Payload, config: &Config) -> Result<()> {
    for package in &payload.data_packages {
        verify_timestamp(package.timestamp, config.block_timestamp)?;
        // Add more verifications as needed
    }
    verify_signer_count(&payload.data_packages, config.signer_count_threshold)?;
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

pub fn verify_signer_count(data_packages: &[DataPackage], threshold: u8) -> Result<()> {
    let unique_signers: std::collections::HashSet<_> =
        data_packages.iter().map(|dp| dp.signer_address).collect();
    if unique_signers.len() < threshold as usize {
        return Err(RedstoneError::InsufficientSignerCount.into());
    }
    Ok(())
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}
