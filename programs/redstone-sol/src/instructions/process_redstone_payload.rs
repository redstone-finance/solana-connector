use crate::error::RedstoneError;
use crate::redstone;
use crate::state::*;
use crate::util::*;
use anchor_lang::prelude::*;
use zkp_u256::U256;

fn make_price_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    seed[0..5].copy_from_slice(b"price");
    seed
}

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
            &make_price_seed(),
            &feed_id
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
                    u256_to_string(&data_point.feed_id),
                    u256_to_num_string(&data_point.value),
                );
            }
        }
    }

    let mut values: Vec<U256> = Vec::new();

    for package in &payload.data_packages {
        for data_point in &package.data_points {
            if feed_id != data_point.feed_id {
                return Err(RedstoneError::UnsupportedFeedId.into());
            }
            values.push(U256::from_bytes_be(&data_point.value));
        }
    }

    let median_value = median(&values);
    if let Some(median_value) = &median_value {
        ctx.accounts.price_account.value = median_value.to_bytes_be();
    } else {
        return Err(RedstoneError::MedianCalculationError.into());
    }

    ctx.accounts.price_account.timestamp = config.block_timestamp;
    ctx.accounts.price_account.feed_id = feed_id;

    msg!(
        "{} {}: {}",
        ctx.accounts.price_account.timestamp,
        u256_to_string(&feed_id),
        u256_to_num_string(&ctx.accounts.price_account.value)
    );

    Ok(())
}
