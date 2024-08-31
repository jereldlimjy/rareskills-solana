use anchor_lang::prelude::*;

declare_id!("Dz9gV5pfVTfqs2L9EYTU22xfcJz6vGwZghwBPzEsjpxN");

#[program]
pub mod day_20 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn increase_account_size(ctx: Context<IncreaseSize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncreaseSize<'info> {
    #[account(
        mut,
        realloc = StorageAccountData::INIT_SPACE + 24,
        realloc::payer = signer,
        realloc::zero = false,
        seeds = [],
        bump
    )]
    storage_account: Account<'info, StorageAccountData>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
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

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct StorageAccountData {
    val: u64,
}
