pub mod constants;
pub mod error;
pub mod instructions;
pub mod redstone;
pub mod state;
pub mod util;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T");

#[program]
pub mod redstone_sol {
    use super::*;

    pub fn process_redstone_payload(
        ctx: Context<ProcessPayload>,
        feed_id: FeedId,
        payload: Vec<u8>,
    ) -> Result<()> {
        msg!(
            "Processing redstone payload of size {} for {}",
            payload.len(),
            zkp_u256::U256::from_bytes_be(&feed_id).to_string()
        );
        instructions::process_redstone_payload(ctx, feed_id, payload)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        signers: Vec<SignerAddress>,
        signer_count_threshold: u8,
        max_timestamp_delay_ms: u64,
        max_timestamp_ahead_ms: u64,
    ) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;
        config_account.owner = ctx.accounts.owner.key();
        config_account.signers = signers;
        config_account.signer_count_threshold = signer_count_threshold;
        config_account.max_timestamp_delay_ms = max_timestamp_delay_ms;
        config_account.max_timestamp_ahead_ms = max_timestamp_ahead_ms;
        Ok(())
    }
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        signers: Option<Vec<SignerAddress>>,
        signer_count_threshold: Option<u8>,
        max_timestamp_delay_ms: Option<u64>,
        max_timestamp_ahead_ms: Option<u64>,
    ) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;
        if let Some(signers) = signers {
            config_account.signers = signers;
        }
        if let Some(threshold) = signer_count_threshold {
            config_account.signer_count_threshold = threshold;
        }
        if let Some(delay) = max_timestamp_delay_ms {
            config_account.max_timestamp_delay_ms = delay;
        }
        if let Some(ahead) = max_timestamp_ahead_ms {
            config_account.max_timestamp_ahead_ms = ahead;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        // leaving some excess space for future use
        space = 8 + std::mem::size_of::<ConfigAccount>() + 32 * 10,
        seeds = [b"config"],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config"],
        bump,
        has_one = owner
    )]
    pub config_account: Account<'info, ConfigAccount>,
}
