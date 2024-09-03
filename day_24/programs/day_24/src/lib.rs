use anchor_lang::prelude::*;

declare_id!("EwU1RoERYFwC1Pf2XqFaBXY3ZhXen1RSkSiTDXxm2d4z");

const STARTING_POINTS: u64 = 10;

#[program]
pub mod day_24 {
    use super::*;

    // for alice and bob exercises
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn update_value(ctx: Context<UpdateValue>, value: u64) -> Result<()> {
    //     ctx.accounts.my_storage.x = value;
    //     Ok(())
    // }

    // for proto-ERC20
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.token_account.amount = STARTING_POINTS;
        ctx.accounts.token_account.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.token_account.authority,
            Errors::NotAuthorized
        );
        require!(
            ctx.accounts.token_account.amount >= amount,
            Errors::InsufficientBalance
        );

        let signer_account = &mut ctx.accounts.token_account;
        let receiver_account = &mut ctx.accounts.receiver_token_account;

        signer_account.amount -= amount;
        receiver_account.amount += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = TokenAccountData::INIT_SPACE + 8,
        seeds = [&authority.key().to_bytes()],
        bump
    )]
    token_account: Account<'info, TokenAccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    token_account: Account<'info, TokenAccountData>,

    #[account(mut)]
    receiver_token_account: Account<'info, TokenAccountData>,

    #[account(mut)]
    authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenAccountData {
    amount: u64,
    authority: Pubkey,
}

#[error_code]
pub enum Errors {
    #[msg("insufficient balance")]
    InsufficientBalance,
    #[msg("not authorized")]
    NotAuthorized,
}

// for alice and bob exercises
// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(
//         init,
//         payer = signer,
//         space = MyStorageData::INIT_SPACE + 8,
//         seeds = [],
//         bump
//     )]
//     my_storage: Account<'info, MyStorageData>,

//     #[account(mut)]
//     signer: Signer<'info>,

//     system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct UpdateValue<'info> {
//     #[account(
//         mut,
//         seeds = [],
//         bump
//     )]
//     my_storage: Account<'info, MyStorageData>,

//     #[account(mut)]
//     signer: Signer<'info>,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct MyStorageData {
//     x: u64,
// }
