pub mod constants;
pub mod error;
pub mod instructions;
pub mod redstone;
pub mod state;
pub mod util;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("redumH9C5NCb4bMUcf5SjE3ANkLSLMTx8L1WPmuHbAR");

#[program]
pub mod redstone_sol {
    use super::*;

    pub fn process_redstone_payload(
        ctx: Context<ProcessPayload>,
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::process_redstone_payload(ctx, payload)
    }
}
