use anchor_lang::prelude::*;

declare_id!("5SEQFGAYRAAGLjgMWEv9JfatG7SGb2wfxiJdpV1hjk3");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
