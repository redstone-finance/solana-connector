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
            util::u256_to_string(feed_id)
        );
        instructions::process_redstone_payload(ctx, feed_id, payload)
    }
}
