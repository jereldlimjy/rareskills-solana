use anchor_lang::prelude::*;

declare_id!("4Z4uBsf1LKDLjFT4m7HByvCYXNXDhqqNX7YRCjrhGj2W");

#[program]
pub mod data_holder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, x: u64) -> Result<()> {
        ctx.accounts.other_account.x = x;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = Storage::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    other_account: Account<'info, Storage>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Storage {
    x: u64,
}
