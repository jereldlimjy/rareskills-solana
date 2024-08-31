use anchor_lang::prelude::*;

declare_id!("Fzv9aGKm22JG65vjj9tWZmmGSdgXcnpQcNindZsCffxz");

#[program]
pub mod storage_recap {
    use super::*;

    // initialises the storage account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // sets the data of the storage account
    pub fn set(ctx: Context<Set>, num: u64, is_happy: bool) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        storage_account.num = num;
        storage_account.is_happy = is_happy;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    storage_account: Account<'info, StorageAccountData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
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
    num: u64,
    is_happy: bool,
}
