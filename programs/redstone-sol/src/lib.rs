pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("rstK87maWhB9ywBwNEzV96rTqLvoUU6oXG3LGoTgyc4");

#[program]
pub mod redstone_sol {
    use super::*;

    pub fn process_redstone_payload(ctx: Context<ProcessPayload>, payload: Vec<u8>) -> Result<()> {
        process_redstone_payload::handler(ctx, payload)
    }
}
