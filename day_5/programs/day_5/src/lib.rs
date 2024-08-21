use anchor_lang::prelude::*;

declare_id!("6D3RAvCUs8Z487SuNEfZkMEHN9q9E7SrhaLUwwfN43Lw");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings fro {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
