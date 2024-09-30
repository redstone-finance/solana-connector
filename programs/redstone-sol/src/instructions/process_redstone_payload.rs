use crate::constants::SIGNERS;
use crate::state::*;
use crate::util::u256_from_slice;
use anchor_lang::prelude::*;
use redstone::{
    core::{config::Config, processor::process_payload},
    network::{as_str::AsHexStr, from_bytes_repr::FromBytesRepr},
};

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
    let feed_id_u256 =
        redstone::network::specific::U256::from_bytes_repr(feed_id.to_vec());
    let config = Config {
        block_timestamp,
        signer_count_threshold: 3,
        signers: SIGNERS.map(|s| s.to_vec()).into(),
        feed_ids: vec![feed_id_u256],
    };

    let result = process_payload(config, payload);

    ctx.accounts.price_account.value = result.values[0].try_into().unwrap();
    ctx.accounts.price_account.timestamp = result.min_timestamp;
    ctx.accounts.price_account.feed_id = feed_id;

    msg!(
        "Updated price for feed {}: {} at timestamp {}",
        feed_id_u256.as_hex_str(),
        ctx.accounts.price_account.value,
        ctx.accounts.price_account.timestamp
    );

    Ok(())
}
