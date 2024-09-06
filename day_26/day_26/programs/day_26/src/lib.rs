use anchor_lang::prelude::*;

declare_id!("6WD6K3QCijfUVepr5HH2eG5LesKZMoHB4kVbQeCdeTJy");

#[program]
pub mod day_26 {
    use super::*;

    // initialise PDA account
    pub fn initialize_pda(ctx: Context<InitializePDA>) -> Result<()> {
        Ok(())
    }

    // initialise keypair account
    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        Ok(())
    }
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
    my_account: Account<'info, AccountData>,

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
    my_account: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountData {
    value: u64,
}
