use anchor_lang::prelude::*;

declare_id!("3trupFmUjciD2oetaDnA9MjMgF32Cr1ZMSsaYg9qp6C7");

#[program]
pub mod day_21 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let lamports = ctx.accounts.account_to_check.lamports();
        msg!("this account has {} lamports", lamports);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    account_to_check: UncheckedAccount<'info>,
}
