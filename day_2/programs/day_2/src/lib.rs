use anchor_lang::prelude::*;

declare_id!("BC9wDiXYV7AyhFEcUn56v2SAbyH4Vv2svboo1DDmJ6Nu");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:#?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn adding(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x: u64 = a - b;
        msg!("Result: {}", x);
        let result = a.checked_sub(b).unwrap();
        msg!("Result 2: {}", result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
