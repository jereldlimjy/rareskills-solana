use anchor_lang::prelude::*;

declare_id!("3PHg83m4JBpZhEoYY7dggsxe6JQiMqhAwhB2tK6r2n1p");

#[program]
pub mod day_11 {
    use super::*;
    use chrono::*;

    // get day of the week from sysvar's unix timeestamp
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let dt = DateTime::from_timestamp(clock.unix_timestamp, 0).unwrap();
        let day = dt.weekday();

        msg!("the day is {}", day);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
