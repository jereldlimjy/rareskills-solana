use anchor_lang::prelude::*;

declare_id!("J8piSK4GgQD4LQRkejw4cRby784MTeYpNuJtirvDz2qm");

#[program]
pub mod mapping_recap {
    use super::*;

    // initialises a map account with a particular key
    pub fn initialize(ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }

    // sets data of a map account depending on the key
    pub fn set(ctx: Context<Set>, key: u64, word: String) -> Result<()> {
        ctx.accounts.map_account.word = word;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    map_account: Account<'info, MapAccountData>,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = MapAccountData::INIT_SPACE + 8,
        seeds = [key.to_le_bytes().as_ref()],
        bump
    )]
    map_account: Account<'info, MapAccountData>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MapAccountData {
    #[max_len(50)]
    word: String,
}
