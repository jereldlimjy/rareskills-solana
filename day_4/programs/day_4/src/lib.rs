use anchor_lang::prelude::*;

declare_id!("7En6BADmdTWebiB4nw35CUSNpFgxFC9SwoGE1NQvpnaZ");

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        // if a < 10 {
        //     return err!(RangeError::TooSmall);
        // }

        // if a > 100 {
        //     return err!(RangeError::TooLarge);
        // }
        require!(a >= 10, RangeError::TooSmall);
        require!(a <= 100, RangeError::TooLarge);

        msg!("Your number {} is within the acceptable range!", a);
        Ok(())
    }

    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        // return err!(RangeError::AlwaysErrors);
        Ok(())
    }
}

#[error_code]
pub enum RangeError {
    #[msg("Number is too large")]
    TooLarge,
    #[msg("Number is too small")]
    TooSmall,
    #[msg("Always errors")]
    AlwaysErrors,
}

#[derive(Accounts)]
pub struct LimitRange {}
