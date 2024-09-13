use crate::instructions::redstone;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_redstone_payload(_ctx: Context<ProcessPayload>, payload: Vec<u8>) -> Result<()> {
    let config = Config {
        signer_count_threshold: 2,
        block_timestamp: Clock::get()?.unix_timestamp as u64,
    };
    let mut bytes = payload.to_vec();
    redstone::verify_redstone_marker(&bytes)?;

    let unsigned_metadata_size = redstone::extract_unsigned_metadata_size(&mut bytes)?;
    let data_packages_count = redstone::extract_data_packages_count(&mut bytes)?;
    let payload = redstone::parse_payload(&mut bytes, data_packages_count)?;
    redstone::verify_data_packages(&payload, &config)?;

    // can do something with the payload here
    //
    Ok(())
}
