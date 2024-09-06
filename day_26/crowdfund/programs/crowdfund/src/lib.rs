use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("26Rgqh9w1PN3Hs8jsxy7wZBDUsLCRsY3EjMQ3hU874YU");

#[program]
pub mod crowdfund {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize_pda(ctx: Context<InitializePDA>) -> Result<()> {
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.pda.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.pda.sub_lamports(amount)?;
        ctx.accounts.signer.add_lamports(amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pda: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pda: Account<'info, AccountData>,

    #[account(
        mut,
        address = Pubkey::from_str("Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB").unwrap()
    )]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializePDA<'info> {
    #[account(
        init,
        payer = signer,
        space = 8,
        seeds = [],
        bump
    )]
    pda: Account<'info, AccountData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
pub struct AccountData {}
