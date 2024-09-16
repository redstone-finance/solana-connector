use crate::error::RedstoneError;
use crate::instructions::redstone;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn process_redstone_payload(
    ctx: Context<ProcessPayload>,
    payload: Vec<u8>,
) -> Result<()> {
    // block_timestamp as milis
    let block_timestamp = Clock::get()?.unix_timestamp as u64 * 1000;
    let config = Config {
        block_timestamp,
        signer_count_threshold: 3,
        signers: vec![
            "109B4A318A4F5DDCBCA6349B45F881B4137DEAFB".into(),
            "12470F7ABA85C8B81D63137DD5925D6EE114952B".into(),
            "1EA62D73EDF8AC05DFCEA1A34B9796E937A29EFF".into(),
            "2C59617248994D12816EE1FA77CE0A64EEB456BF".into(),
            "83CBA8C619FB629B81A65C2E67FE15CF3E3C9747".into(),
        ],
        feed_ids: vec![
            redstone::u256_from_slice("ETH".as_bytes()),
            redstone::u256_from_slice("BTC".as_bytes()),
        ],
    };
    redstone::verify_redstone_marker(&payload)?;

    let mut payload = payload;
    let payload = redstone::parse_raw_payload(&mut payload)?;

    redstone::verify_data_packages(&payload, &config)?;

    msg!(
        "Payload processed successfully: {}",
        payload.data_packages.len()
    );
    for package in &payload.data_packages {
        msg!(
            "Package signer: 0x{}",
            redstone::bytes_to_hex(&package.signer_address)
        );
        for data_point in &package.data_points {
            msg!(
                "Data point: {} {}",
                redstone::u256_to_string(data_point.feed_id),
                data_point.value.to_string()
            );
        }
    }
    for package in &payload.data_packages {
        for data_point in &package.data_points {
            let price_account = if data_point.feed_id
                == redstone::u256_from_slice("ETH".as_bytes())
            {
                &mut ctx.accounts.eth_price_account
            } else if data_point.feed_id
                == redstone::u256_from_slice("BTC".as_bytes())
            {
                &mut ctx.accounts.btc_price_account
            } else {
                return Err(RedstoneError::InvalidPriceAccount.into());
            };

            price_account.value = data_point.value;
            price_account.timestamp = config.block_timestamp;
            price_account.feed_id = data_point.feed_id;

            let feed_id_str = redstone::u256_to_string(data_point.feed_id);
            msg!(
                "Updated price for feed {}: {} at timestamp {}",
                feed_id_str,
                price_account.value,
                price_account.timestamp
            );
        }
    }

    Ok(())
}
