use crate::U256;
use crate::{
    constants::SIGNERS, error::RedstoneError, ConfigAccount, FeedId,
    PriceData,
};
use anchor_lang::prelude::*;
use redstone::{
    core::{config::Config, processor::process_payload},
    network::{as_str::AsHexStr, from_bytes_repr::FromBytesRepr},
};

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
    let feed_id_u256 = U256::from_bytes_repr(feed_id.to_vec());
    // block_timestamp as milis
    let config = Config {
        block_timestamp: Clock::get()?.unix_timestamp as u64 * 1000,

        signer_count_threshold: 3,
        signers: SIGNERS.map(|s| s.to_vec()).into(),
        feed_ids: vec![feed_id_u256],
    };

    let processed_payload = process_payload(config, payload);

    if ctx.accounts.price_account.timestamp >= processed_payload.min_timestamp
    {
        return Err(RedstoneError::TimestampTooOld.into());
    }

    let price = processed_payload.values[0];
    ctx.accounts.price_account.value = price.to_big_endian();
    ctx.accounts.price_account.timestamp = processed_payload.min_timestamp;
    ctx.accounts.price_account.feed_id = feed_id;

    msg!(
        "Updated price for feed {}: {} at timestamp {}",
        feed_id_u256.as_hex_str(),
        price,
        ctx.accounts.price_account.timestamp
    );

    Ok(())
}
