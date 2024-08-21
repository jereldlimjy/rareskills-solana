use anchor_lang::prelude::*;

declare_id!("BtkZuTucLioHrkK2hn12mUQs4u6B1XDzTjrUGR3qG9jt");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_add(b).unwrap();
        msg!("Result of {} + {} is {}", a, b, result);
        Ok(())
    }

    pub fn subtract(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_sub(b).unwrap();
        msg!("Result of {} - {} is {}", a, b, result);
        Ok(())
    }

    pub fn multiply(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_mul(b).unwrap();
        msg!("Result of {} * {} is {}", a, b, result);
        Ok(())
    }

    pub fn divide(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_div(b).unwrap();
        msg!("Result of {} / {} is {}", a, b, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
