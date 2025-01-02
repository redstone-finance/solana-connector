use crate::{
    error::RedstoneError, state::PriceData, util::debug_msg, ConfigAccount,
    FeedIdBs,
};
use anchor_lang::prelude::*;
use redstone::{solana::SolanaRedStoneConfig,
    core::{config::Config, processor::process_payload}, network::as_str::AsHexStr, FeedId
};

fn make_price_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    seed[0..5].copy_from_slice(b"price");
    seed
}

#[derive(Accounts)]
#[instruction(feed_id: FeedIdBs)]
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
    feed_id: FeedIdBs,
    payload: Vec<u8>,
) -> Result<()> {
    let feed_id = FeedId(feed_id);
    let signers = ctx
        .accounts
        .config_account
        .signers
        .iter()
        .map(|s| s.to_vec().into())
        .collect();
    // block_timestamp as milis
    let config: SolanaRedStoneConfig = Config {
        block_timestamp: (Clock::get()?.unix_timestamp as u64 * 1000).into(),
        signer_count_threshold: ctx
            .accounts
            .config_account
            .signer_count_threshold,
        signers,
        feed_ids: vec![feed_id],
    }.into();

    let processed_payload = process_payload(&config, payload)?;

    if processed_payload.min_timestamp.is_before(ctx.accounts.price_account.timestamp.into())
    {
        return Err(RedstoneError::TimestampTooOld.into());
    }

    let price = processed_payload.values[0];
    ctx.accounts.price_account.value = price.0;
    ctx.accounts.price_account.timestamp = processed_payload.min_timestamp.as_millis();
    ctx.accounts.price_account.feed_id = feed_id.0;

    debug_msg(|| {
        format!(
            "{} {}: {:?}",
            ctx.accounts.price_account.timestamp,
            feed_id.as_hex_str(),
            price,
        )
    });

    Ok(())
}
