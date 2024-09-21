use crate::constants::FEED_IDS_STR;
use crate::constants::SIGNERS;
use crate::error::RedstoneError;
use crate::redstone;
use crate::state::*;
use crate::util::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(feed_id: FeedId)]
pub struct ProcessPayload<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [&u256_from_slice("price".as_bytes()), &u256_from_slice(feed_id.as_slice())],
        bump,
        constraint = price_account.to_account_info().owner == __program_id
    )]
    pub price_account: Account<'info, PriceData>,
    pub system_program: Program<'info, System>,
}

pub fn process_redstone_payload(
    ctx: Context<ProcessPayload>,
    feed_id: FeedId,
    payload: Vec<u8>,
) -> Result<()> {
    // block_timestamp as milis
    let block_timestamp = Clock::get()?.unix_timestamp as u64 * 1000;
    let config = Config {
        block_timestamp,
        signer_count_threshold: 3,
        signers: SIGNERS,
        feed_ids: FEED_IDS_STR
            .iter()
            .map(|&x| u256_from_slice(x.as_bytes()))
            .collect::<Vec<FeedId>>()
            .try_into()
            .unwrap(),
    };

    if !config.feed_ids.contains(&feed_id) {
        return Err(RedstoneError::UnsupportedFeedId.into());
    }

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

    for package in &payload.data_packages {
        if let Some(data_point) = &package.data_points.first() {
            if !config.feed_ids.contains(&data_point.feed_id) {
                return Err(RedstoneError::UnsupportedFeedId.into());
            }
            ctx.accounts.price_account.value = data_point.value;
            ctx.accounts.price_account.timestamp = config.block_timestamp;
            ctx.accounts.price_account.feed_id = data_point.feed_id;

            msg!(
                "Updated price for feed {}: {} at timestamp {}",
                u256_to_string(data_point.feed_id),
                ctx.accounts.price_account.value,
                ctx.accounts.price_account.timestamp
            );
        }
    }

    Ok(())
}
