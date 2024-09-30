pub mod constants;
pub mod instructions;
pub mod state;
pub mod util;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("5cKwbjiexA7SEBEKb1nEvkykuEWEJhKpJLYbYKTBCyXY");

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
            util::u256_to_string(feed_id)
        );
        instructions::process_redstone_payload(ctx, feed_id, payload)
    }
}
