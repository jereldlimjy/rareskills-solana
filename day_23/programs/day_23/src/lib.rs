use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("3g1SZb3QuXi4oNXNAGNKJGTVCfSx9T9i756awVw2NW1R");

#[program]
pub mod day_23 {
    use super::*;

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.receiver.to_account_info(),
            },
        );

        // amount is in lamports
        let res = system_program::transfer(cpi_context, amount);

        if res.is_ok() {
            Ok(())
        } else {
            err!(Errors::TransferFailed)
        }
    }

    // exercise: split sol among 2 receivers
    pub fn split_sol(ctx: Context<SplitSol>, amount: u64) -> Result<()> {
        let split_amount = amount / 2;
        let cpi_context_1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.receiver_1.to_account_info(),
            },
        );
        let cpi_context_2 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.receiver_2.to_account_info(),
            },
        );

        let res_1 = system_program::transfer(cpi_context_1, split_amount);
        let res_2 = system_program::transfer(cpi_context_2, split_amount);

        if res_1.is_ok() && res_2.is_ok() {
            Ok(())
        } else {
            err!(Errors::TransferFailed)
        }
    }

    // arbitrary number of accounts
    pub fn split_sol_arb<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SplitSolArb<'info>>,
        amount: u64,
    ) -> Result<()> {
        let amount_each_gets = amount / ctx.remaining_accounts.len() as u64;
        let system_program = &ctx.accounts.system_program;

        // note the keyword `remaining_accounts`
        for recipient in ctx.remaining_accounts {
            let cpi_accounts = system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: recipient.to_account_info(),
            };
            let cpi_program = system_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

            let res = system_program::transfer(cpi_context, amount_each_gets);
            if !res.is_ok() {
                return err!(Errors::TransferFailed);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    // to account
    /// CHECK:
    #[account(mut)]
    receiver: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    // from account
    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SplitSol<'info> {
    /// CHECK:
    #[account(mut)]
    receiver_1: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    receiver_2: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SplitSolArb<'info> {
    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}
