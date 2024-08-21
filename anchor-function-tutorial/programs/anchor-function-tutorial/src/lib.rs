use anchor_lang::prelude::*;

declare_id!("BsHEqRbo3rjsQJc8WsV5X5yWn2KcxvsnN2sK8yNxs5XQ");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn function_a(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(ctx: Context<Empty>) -> Result<()> {
        Ok(())
    }

    // pub fn boaty_mc_boatface(ctx: Context<Initialize>, a: u64) -> Result<()> {
    //     msg!("boaty mc boatface receives {} as argument", a);
    //     Ok(())
    // }

    // pub fn add(ctx: Context<NonEmptyAccountExample>, a: u64, b: u64) -> Result<()> {
    //     let sum = a + b;
    //     msg!("Sum is {}", sum);
    //     Ok(())
    // }

    // pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
    //     let difference = a - b;
    //     msg!("Difference is {}", difference);
    //     Ok(())
    // }

    // pub fn mul(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
    //     let result = a * b;
    //     msg!("Result is {}", result);
    //     Ok(())
    // }

    // pub fn div(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
    //     let result = a / b;
    //     msg!("Result is {}", result);
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}
