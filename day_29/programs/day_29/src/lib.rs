use anchor_lang::prelude::*;

declare_id!("fpZ98Uhsq8EZEqrXR3gNzc6YuArM16iiTuYXZYMTCMA");

#[program]
pub mod day_29 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = StorageAccountData::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    storage_account: Account<'info, StorageAccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct StorageAccountData {
    value: u64,
}
