use anchor_lang::prelude::*;

declare_id!("DaW3rYpnyEPLB6mEsX2kbYt8BFS6Em2ETojGuNAN3zYM");

#[program]
pub mod day_19 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
