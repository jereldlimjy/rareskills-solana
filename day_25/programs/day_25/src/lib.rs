use anchor_lang::prelude::*;

declare_id!("dLNTyoprpd2qNKbcnJqidZYEhQqiMMmkHAkMhuTWv6t");

#[program]
pub mod day_25 {
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
        space = KeypairAccountData::INIT_SPACE + 8
    )]
    keypair_account: Account<'info, KeypairAccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct KeypairAccountData {
    x: u64,
}
