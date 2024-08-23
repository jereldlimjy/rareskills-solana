use anchor_lang::prelude::*;

declare_id!("7Me9JXYvjLM6jtMymDRPa3BVVs6XcAENJJWcxfMoJxRs");

#[program]
pub mod day_15 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
