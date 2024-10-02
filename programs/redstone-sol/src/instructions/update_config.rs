use crate::state::ConfigAccount;
use anchor_lang::prelude::*;

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
