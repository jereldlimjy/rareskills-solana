use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("19YoCePxeoF2mFaegjFYfugkKXZ724qxVzPLcsz8Mks");

#[program]
pub mod change_owner {
    use super::*;

    // initialise pda account
    pub fn initialize_pda(ctx: Context<InitializePDA>) -> Result<()> {
        Ok(())
    }

    // initialise keypair account
    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        Ok(())
    }

    // change owner
    pub fn change_owner(ctx: Context<ChangeOwner>) -> Result<()> {
        let keypair_account = &mut ctx.accounts.keypair.to_account_info();

        // transfer ownership
        keypair_account.assign(&system_program::ID);

        // must erase all data in the account
        let res = keypair_account.realloc(0, false);

        if !res.is_ok() {
            return err!(Err::ReallocFailed);
        }

        Ok(())
    }
}

#[error_code]
pub enum Err {
    #[msg("realloc failed")]
    ReallocFailed,
}

#[derive(Accounts)]
pub struct InitializePDA<'info> {
    #[account(
        init,
        payer = signer,
        space = AccountData::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    pda: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeKeypair<'info> {
    #[account(
        init,
        payer = signer,
        space = AccountData::INIT_SPACE + 8,
    )]
    keypair: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut)]
    keypair: Account<'info, AccountData>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountData {
    value: u64,
}
