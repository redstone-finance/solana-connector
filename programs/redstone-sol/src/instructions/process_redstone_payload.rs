use crate::error::RedstoneError;
use crate::redstone;
use crate::state::*;
use crate::util::*;
use anchor_lang::prelude::*;
use std::collections::HashMap;

#[derive(Accounts)]
#[instruction(feed_id: FeedId)]
pub struct ProcessPayload<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [
            &U256::from_bytes("price".as_bytes()),
            &U256::from_bytes(feed_id.as_slice()),
        ],
        bump,
        constraint = price_account.to_account_info().owner == __program_id
    )]
    pub price_account: Account<'info, PriceData>,
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

pub fn process_redstone_payload(
    ctx: Context<ProcessPayload>,
    feed_id: FeedId,
    payload: Vec<u8>,
) -> Result<()> {
    // block_timestamp as milis
    let config = Config {
        block_timestamp: Clock::get()?.unix_timestamp as u64 * 1000,
        config_account: &ctx.accounts.config_account,
    };

    redstone::verify_redstone_marker(&payload)?;

    let mut payload = payload;
    let payload = redstone::parse_raw_payload(&mut payload)?;

    redstone::verify_data_packages(&payload, &config)?;

    #[cfg(feature = "dev")]
    {
        msg!(
            "Payload processed successfully: {}",
            payload.data_packages.len()
        );
        for package in &payload.data_packages {
            msg!(
                "Package signer: 0x{}",
                bytes_to_hex(&package.signer_address)
            );
            for data_point in &package.data_points {
                msg!(
                    "Data point: {} {}",
                    u256_to_string(data_point.feed_id),
                    data_point.value.to_string()
                );
            }
        }
    }

    let mut feed_values: HashMap<FeedId, Vec<U256>> = HashMap::new();

    for package in &payload.data_packages {
        for data_point in &package.data_points {
            if feed_id != data_point.feed_id {
                return Err(RedstoneError::UnsupportedFeedId.into());
            }
            feed_values
                .entry(data_point.feed_id)
                .or_default()
                .push(data_point.value);
        }
    }

    for (feed_id, mut values) in feed_values {
        let median_value = calculate_median(&mut values);
        ctx.accounts.price_account.value = median_value;
        ctx.accounts.price_account.timestamp = config.block_timestamp;
        ctx.accounts.price_account.feed_id = feed_id;

        msg!(
            "Updated price for feed {}: {} at timestamp {}",
            u256_to_string(feed_id),
            u256_to_num_string(ctx.accounts.price_account.value),
            ctx.accounts.price_account.timestamp
        );
    }

    Ok(())
}
