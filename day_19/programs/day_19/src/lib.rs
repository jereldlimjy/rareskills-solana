use anchor_lang::prelude::*;

declare_id!("DaW3rYpnyEPLB6mEsX2kbYt8BFS6Em2ETojGuNAN3zYM");

#[program]
pub mod day_19 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key: u64, val: u64) -> Result<()> {
        ctx.accounts.my_storage.val = val;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    my_storage: Account<'info, MyStorage>
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {
    #[account(init,
        payer = my_signer,
        space = MyStorage::INIT_SPACE + 8,
        seeds = [&key.to_le_bytes()], 
        bump
    )]
    my_mapping: Account<'info, MyStorage>,

    #[account(mut)]
    my_signer: Signer<'info>,

    system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct MyStorage {
    val: u64
}
