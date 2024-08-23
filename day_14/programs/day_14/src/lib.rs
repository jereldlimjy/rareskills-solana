use anchor_lang::prelude::*;

declare_id!("9XGp1YDYv3DJjJqqH5WVm1c3Z4mj5bdTYmL37LgLaur9");

const OWNER: &str = "Hj68YqdDaaTPw5hffSMV3hAXxGpijWswEMEcaR3bQmRB";

#[program]
pub mod day_14 {
    use super::*;

    // single signer
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     let signer_1 = &mut ctx.accounts.signer1;

    //     msg!("signer 1's key is {}", signer_1.key);

    //     Ok(())
    // }

    // multiple signers
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let signer_1 = &mut ctx.accounts.signer1;
        let signer_2 = &mut ctx.accounts.signer2;

        msg!("signer 1's key is {}", signer_1.key);
        msg!("signer 2's key is {}", signer_2.key);

        Ok(())
    }

    // exercise (3 signers)
    pub fn three_signers(ctx: Context<Initialize>) -> Result<()> {
        let signer_1 = &mut ctx.accounts.signer1;
        let signer_2 = &mut ctx.accounts.signer2;
        let signer_3 = &mut ctx.accounts.signer3;

        msg!("signer 1's key is {}", signer_1.key);
        msg!("signer 2's key is {}", signer_2.key);
        msg!("signer 3's key is {}", signer_3.key);

        Ok(())
    }

    // only owner
    #[access_control(check(&ctx))]
    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Holla, I'm the owner.");
        Ok(())
    }
}

pub fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
