use anchor_lang::prelude::*;

declare_id!("6SNYWbsEaHE5wRAsKYYmLU5nfPdDEDBeWrpD98mFANYA");

#[program]
pub mod compute_unit {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let mut a = vec![];
        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);
        msg!("Vec value: {:#?}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub singer1: Signer<'info>,
    pub singer2: Signer<'info>,
}
