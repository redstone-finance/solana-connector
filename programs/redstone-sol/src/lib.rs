use anchor_lang::prelude::*;

declare_id!("rstK87maWhB9ywBwNEzV96rTqLvoUU6oXG3LGoTgyc4");

#[program]
pub mod redstone_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
