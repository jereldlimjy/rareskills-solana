use anchor_lang::prelude::*;

declare_id!("Yv7pCyzTVYVQAjs4KYcM1mYDKMT9sJLDBbVqvDCwQKH");

#[program]
pub mod day_12 {
    use super::*;
    use anchor_lang::solana_program::last_restart_slot::LastRestartSlot;
    use anchor_lang::solana_program::sysvar::instructions;

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        let clock = Clock::get();
        msg!("clock: {:?}", clock);

        let epoch_schedule = EpochSchedule::get();
        msg!("epoch schedule: {:?}", epoch_schedule);

        // Accessing the StakeHistory sysvar
        // Create an array to store the StakeHistory account
        // let arr = [ctx.accounts.stake_history.clone()];

        // // Create an iterator for the array
        // let accounts_iter = &mut arr.iter();

        // // Get the next account info from the iterator (still StakeHistory)
        // let sh_sysvar_info = accounts_iter.next().unwrap();

        // Create a StakeHistory instance from the account info
        let stake_history = StakeHistory::from_account_info(&ctx.accounts.stake_history)?;

        msg!("stake history: {:?}", ctx.accounts.stake_history);
        msg!("stake_history: {:?}", stake_history);

        // // Get Instruction sysvar
        // let arr = [ctx.accounts.instruction_sysvar.clone()];

        // let account_info_iter = &mut arr.iter();

        // let instructions_sysvar_account = next_account_info(account_info_iter)?;

        // // Load the instruction details from the instruction sysvar account
        // let instruction_details =
        //     instructions::load_instruction_at_checked(0, instructions_sysvar_account)?;

        // msg!(
        //     "Instruction details of this transaction: {:?}",
        //     instruction_details
        // );
        // msg!("Number is: {}", number);

        // // Get LastRestartSlot sysvar
        // let arr = [ctx.accounts.last_restart_slot.clone()];

        // let account_info_iter = &mut arr.iter();

        // let last_restart_slot_account = next_account_info(account_info_iter)?;

        // let last_restart_slot = LastRestartSlot::from_account_info(last_restart_slot_account)?;

        // msg!("Last restart slot: {:?}", last_restart_slot);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>, // We create an account for the StakeHistory sysvar
                                           // /// CHECK:
                                           // pub recent_blockhashes: AccountInfo<'info>,
                                           // /// CHECK:
                                           // pub instruction_sysvar: AccountInfo<'info>,
                                           // /// CHECK:
                                           // pub last_restart_slot: AccountInfo<'info>,
}
