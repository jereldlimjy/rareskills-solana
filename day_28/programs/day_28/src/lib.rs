use anchor_lang::prelude::*;

declare_id!("9SKD6VinSfo5rmAvN1mRrGJQ76QNWMoj2tTEKSL98x5J");

#[program]
pub mod day_28 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, value: u32) -> Result<()> {
        ctx.accounts.my_account.value = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = AccountData::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    my_account: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    my_account: Account<'info, AccountData>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountData {
    value: u32,
}
