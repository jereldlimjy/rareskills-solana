use anchor_lang::prelude::*;

pub mod calculate;

declare_id!("DtdPfNTixQEoU8jdnFBa5kLut59tFQhgofo29qLuzeph");

#[program]
pub mod day_10 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // Call the internal_function from within its parent module
        some_internal_function::internal_function();

        Ok(())
    }

    // internal function - accesible inside the program and other programs that import it
    pub mod some_internal_function {
        pub fn internal_function() {
            // Internal function logic...
        }
    }

    pub fn add_two_numbers(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = calculate::add(a, b);
        msg!("Result of adding {} and {} is: {}", a, b, result);
        Ok(())
    }
}

mod do_something {
    // Import day_10 module
    use crate::day_10;

    pub fn some_func_here() {
        // Call the internal_function from outside its parent module
        day_10::some_internal_function::internal_function();

        // Do something else...
    }
}

// #[program]
// pub mod day_10 {
//     use super::*;

//     pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
//         // Call the private_function from within its parent module
//         some_function_function::private_function();

//         Ok(())
//     }

//     pub mod some_function_function {
//         pub(in crate::day_10) fn private_function() {
//             // Private function logic...
//         }
//     }
// }

#[derive(Accounts)]
pub struct Initialize {}
