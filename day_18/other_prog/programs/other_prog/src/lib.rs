use anchor_lang::prelude::*;

declare_id!("4h8gf2dJLxgGdhK82HmnAZPS1ixyYpfLmJetPmWfWMEK");

#[program]
pub mod other_prog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<SetFlag>, flag: bool) -> Result<()> {
        ctx.accounts.my_storage.flag = flag;
        msg!("flag set to: {}", ctx.accounts.my_storage.flag);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = std::mem::size_of::<MyStorage>() + 8,
        seeds = [],
        bump
    )]
    my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFlag<'info> {
    #[account(mut, seeds = [], bump)]
    my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    flag: bool,
}
