use anchor_lang::prelude::*;

declare_id!("3fas8B6KSeHjRYJqrF4DUVivqiR6mBTWWMZAwwUetphs");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
