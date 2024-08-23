use anchor_lang::prelude::*;

declare_id!("DmQ263WRE8sdnh5jUJozNAM5LUGmXSBnPsLwGnsvD7oH");

#[program]
pub mod day_13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, value: u64, input_string: String) -> Result<()> {
        emit!(MyEvent { value });
        emit!(StringEvent {
            value: input_string
        });

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct StringEvent {
    pub value: String,
}

#[derive(Accounts)]
pub struct Initialize {}
