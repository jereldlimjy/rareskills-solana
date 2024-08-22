use anchor_lang::prelude::*;

declare_id!("FAqkiSAWQs1EAt8uj1PpmLqM9fxZYSkNxaxxRK7Rfgim");

#[program]
pub mod day_6 {
    use super::*;

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        // if age >= 18 {
        //     msg!("You are 18 years old or above");
        // } else {
        //     msg!("You are below 18 years old");
        // }

        match age {
            1 | 2 => {
                msg!("Age is 1 or 2");
            }
            _ => {
                msg!("Age is not 1 or 2");
            }
        };
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            name: String,
            age: u64,
        };

        let mut person = Person { name, age };

        msg!("before: {} is {} years old", person.name, person.age);

        person.name = "Jereld".to_string();
        person.age = 26;

        msg!("after: {} is {} years old", person.name, person.age);

        Ok(())
    }

    pub fn exercise(ctx: Context<Initialize>, vec: Vec<u64>) -> Result<()> {
        let mut even_nums = Vec::with_capacity(vec.len());

        for num in vec {
            if num % 2 == 0 {
                even_nums.push(num);
            }
        }

        msg!("The even numbers are: {:?}", even_nums);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
