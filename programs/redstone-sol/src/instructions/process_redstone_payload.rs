use crate::instructions::redstone;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_redstone_payload(_ctx: Context<ProcessPayload>, payload: Vec<u8>) -> Result<()> {
    let config = Config {
        signer_count_threshold: 2,
        block_timestamp: Clock::get()?.unix_timestamp as u64,
        signers: vec![
            "1ea62d73edF8ac05dfcea1a34b9796e937a29eFF".into(),
            "109b4a318a4f5ddcbca6349b45f881b4137deafb".into(),
        ],
        feed_ids: vec![
            redstone::u256_from_slice("ETH".as_bytes()),
            redstone::u256_from_slice("BTC".as_bytes()),
        ],
    };
    let mut bytes = payload.to_vec();
    redstone::verify_redstone_marker(&bytes)?;

    let _unsigned_metadata_size = redstone::extract_unsigned_metadata_size(&mut bytes)?;
    let data_packages_count = redstone::extract_data_packages_count(&mut bytes)?;
    let payload = redstone::parse_payload(&mut bytes, data_packages_count)?;
    // TODO verify should happen at earlier stage
    redstone::verify_data_packages(&payload, &config)?;

    // can do something with the payload here
    //
    Ok(())
}
