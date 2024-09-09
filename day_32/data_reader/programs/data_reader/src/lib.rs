use anchor_lang::prelude::*;

declare_id!("EhTmPNj3YaNbWSisBhyaKDNcfv9vZWVwGMAGMoBEge4k");

#[program]
pub mod data_reader {
    use super::*;

    pub fn read_other_account(ctx: Context<ReadOtherAccount>) -> Result<()> {
        let data_account = &ctx.accounts.other_account;

        if data_account.data_is_empty() {
            return err!(MyError::NoData);
        }

        let mut data_slice: &[u8] = &data_account.data.borrow();
        let data_struct: Storage = AccountDeserialize::try_deserialize(&mut data_slice)?;
        msg!("the value of x is {}", data_struct.x);
        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("No data")]
    NoData,
}

#[derive(Accounts)]
pub struct ReadOtherAccount<'info> {
    /// CHECK:
    other_account: UncheckedAccount<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Storage {
    x: u64,
}
