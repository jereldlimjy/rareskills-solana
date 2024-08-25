use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("HbVE1AX1gQppKEVgCM8L5j19SNi6mP5pKtfEaHik3c6o");

#[program]
pub mod day_17 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_x: u64) -> Result<()> {
        ctx.accounts.my_storage.x = new_x;
        Ok(())
    }

    pub fn print(ctx: Context<Print>) -> Result<()> {
        let x = ctx.accounts.my_storage.x;
        msg!("the value of x is: {}", x);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let x = ctx.accounts.my_storage.x;
        msg!("the current value of x is: {}", x);

        ctx.accounts.my_storage.x = x + 1;
        let x = ctx.accounts.my_storage.x;
        msg!("the new value of x is: {}", x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<MyStorage>() + 8,
        seeds = [],
        bump
    )]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Print<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
