use anchor_lang::prelude::*;

declare_id!("9nbQJUGARmyW5ACemJPUZB2j3kz5fhg81BUhz6GxphRg");

#[program]
pub mod day_30 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, value: u64) -> Result<()> {
        ctx.accounts.my_keypair_account.value = value;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = signer
    )]
    my_keypair_account: Account<'info, AccountData>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    my_keypair_account: Account<'info, AccountData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = AccountData::INIT_SPACE + 8
    )]
    my_keypair_account: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountData {
    value: u64,
}
