use anchor_lang::prelude::*;

declare_id!("3JqxV2WvUeraDrBRx3Da4E2RSiJU654tGcCJcY9Cycey");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(_ctx: Context<Empty>) -> Result<()> {
        Ok(())
    }

    pub fn add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }

    pub fn non_empty_account_example(_ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
