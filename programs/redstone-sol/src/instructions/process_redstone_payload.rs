use crate::error::RedstoneError;
use crate::state::{Config, DataPackage, Payload, ProcessPayload};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use sha3::{Digest, Keccak256};
use std::collections::HashSet;
use std::convert::TryInto;

// Constants
const REDSTONE_MARKER: [u8; 9] = [0, 0, 2, 237, 87, 1, 30, 0, 0];
const DATA_FEED_ID_BS: usize = 32;
const SIGNATURE_BS: usize = 65;
const TIMESTAMP_BS: usize = 6;
const DATA_POINTS_COUNT_BS: usize = 3;
const DATA_POINT_VALUE_BYTE_SIZE_BS: usize = 4;
const MAX_TIMESTAMP_DELAY_MS: u64 = 15 * 60 * 1000; // 15 minutes
const MAX_TIMESTAMP_AHEAD_MS: u64 = 3 * 60 * 1000; // 3 minutes

pub fn handler(_ctx: Context<ProcessPayload>, payload: Vec<u8>) -> Result<()> {
    let config = Config {
        signer_count_threshold: 2,
        block_timestamp: Clock::get()?.unix_timestamp as u64,
    };

    let _processed_payload = process_payload(&payload, &config)?;

    // Here you can do something with the processed payload
    // For example, you could store it in an account or use it for further processing

    Ok(())
}

// Helper functions
fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn recover_address(message: &[u8], signature: &[u8]) -> Result<[u8; 20]> {
    let recovery_byte = signature[64];
    let msg_hash = keccak256(message);
    let public_key = secp256k1_recover(&msg_hash, recovery_byte, &signature[..64])
        .map_err(|_| error!(RedstoneError::InvalidSignature))?;
    let public_key_hash = keccak256(&public_key.to_bytes()[1..]);
    Ok(public_key_hash[12..].try_into().unwrap())
}

// Main processing function
fn process_payload(payload_bytes: &[u8], config: &Config) -> Result<Payload> {
    let mut bytes = payload_bytes.to_vec();
    verify_redstone_marker(&bytes)?;

    let payload = parse_payload(&mut bytes)?;
    verify_data_packages(&payload, config)?;

    Ok(payload)
}

fn verify_redstone_marker(bytes: &[u8]) -> Result<()> {
    if bytes.len() < REDSTONE_MARKER.len()
        || &bytes[bytes.len() - REDSTONE_MARKER.len()..] != REDSTONE_MARKER
    {
        return Err(error!(RedstoneError::InvalidRedstoneMarker));
    }
    Ok(())
}

fn parse_payload(_bytes: &mut Vec<u8>) -> Result<Payload> {
    // need to adapt the impl here to work with Solana's program env
    Err(error!(RedstoneError::PayloadParseError))
}

fn verify_data_packages(payload: &Payload, config: &Config) -> Result<()> {
    for package in &payload.data_packages {
        verify_timestamp(package.timestamp, config.block_timestamp)?;
        verify_signature(package)?;
    }

    verify_signer_count(&payload.data_packages, config.signer_count_threshold)?;

    Ok(())
}

fn verify_timestamp(timestamp: u64, block_timestamp: u64) -> Result<()> {
    if timestamp + MAX_TIMESTAMP_DELAY_MS < block_timestamp {
        return Err(error!(RedstoneError::TimestampTooOld));
    }
    if timestamp > block_timestamp + MAX_TIMESTAMP_AHEAD_MS {
        return Err(error!(RedstoneError::TimestampTooFuture));
    }
    Ok(())
}

fn verify_signature(_package: &DataPackage) -> Result<()> {
    // Implementation for signature verification
    // You'll need to adapt the original Rust implementation to work with Solana's crypto primitives
    Err(error!(RedstoneError::InvalidSignature))
}

fn verify_signer_count(data_packages: &[DataPackage], threshold: u8) -> Result<()> {
    let unique_signers: HashSet<_> = data_packages.iter().map(|dp| dp.signer_address).collect();
    if unique_signers.len() < threshold as usize {
        return Err(error!(RedstoneError::InsufficientSignerCount));
    }
    Ok(())
}
