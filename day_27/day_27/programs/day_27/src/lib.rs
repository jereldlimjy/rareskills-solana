use anchor_lang::prelude::*;

declare_id!("4Qy365FgkLja5mB2kaoFxg4PGDhFp8VbjFExqi7c4JwJ");

#[program]
pub mod day_27 {
    use anchor_lang::system_program;

    use super::*;

    // pub fn increment(ctx: Context<Increment>) -> Result<()> {
    //     ctx.accounts.my_pda.counter += 1;
    //     Ok(())
    // }

    // example to illustrate initialising multiple times
    pub fn initialise(ctx: Context<Initialise>) -> Result<()> {
        Ok(())
    }

    pub fn drain_lamport(ctx: Context<DrainLamports>) -> Result<()> {
        let lamports = ctx.accounts.my_pda.get_lamports();
        ctx.accounts.my_pda.sub_lamports(lamports)?;
        ctx.accounts.signer.add_lamports(lamports)?;
        Ok(())
    }

    pub fn give_to_system_program(ctx: Context<GiveToSystemProgram>) -> Result<()> {
        let my_pda = &mut ctx.accounts.my_pda.to_account_info();
        my_pda.assign(&system_program::ID);
        my_pda.realloc(0, false)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialise<'info> {
    #[account(
        init,
        payer = signer,
        space = MyPdaData::INIT_SPACE + 8,
        seeds = [],
        bump
    )]
    my_pda: Account<'info, MyPdaData>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DrainLamports<'info> {
    #[account(mut)]
    my_pda: Account<'info, MyPdaData>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GiveToSystemProgram<'info> {
    #[account(mut)]
    my_pda: Account<'info, MyPdaData>,
}

#[account]
#[derive(InitSpace)]
pub struct MyPdaData {}

// #[derive(Accounts)]
// pub struct Increment<'info> {
//     #[account(
//         init,
//         payer = signer,
//         space = MyPdaData::INIT_SPACE + 8,
//         seeds = [],
//         bump
//     )]
//     my_pda: Account<'info, MyPdaData>,

//     system_program: Program<'info, System>,

//     #[account(mut)]
//     signer: Signer<'info>,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct MyPdaData {
//     counter: u64,
// }
