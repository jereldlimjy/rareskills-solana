use anchor_lang::prelude::*;

declare_id!("EzCTsqfQDu4wDgHHkgSPm1qE9pwhXPc2F9FjrbZj3gHL");

#[program]
pub mod erase {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn erase(ctx: Context<Erase>) -> Result<()> {
        let pda = &mut ctx.accounts.my_pda.to_account_info();
        pda.realloc(0, false)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = MyPdaData::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    my_pda: Account<'info, MyPdaData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Erase<'info> {
    // / CHECK:
    #[account(mut)]
    my_pda: Account<'info, MyPdaData>,
}

#[account]
#[derive(InitSpace)]
pub struct MyPdaData {
    value: u64,
}
