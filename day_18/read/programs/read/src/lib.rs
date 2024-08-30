use anchor_lang::prelude::*;

declare_id!("2JKAKYam1B8epZmHdP6VkxDwnMxAFc8mjRHm71UBDZ77");

#[program]
pub mod read {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
