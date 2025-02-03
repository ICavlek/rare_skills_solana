use anchor_lang::prelude::*;

declare_id!("6THYXhwNZ6W9JCRyqyL1m79iV3CwmAqxUM4BYGi7Pn57");

const OWNER: &str = "GGWSVKHQ525GDKwjqbWe3dv6bemYcv6ps5WXCi1TkiHU";

#[program]
pub mod day_14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let signer1: &mut Signer = &mut ctx.accounts.signer1;
        let signer2: &mut Signer = &mut ctx.accounts.signer2;
        msg!("Signer 1: {:?}", *signer1.key);
        msg!("Signer 2: {:?}", *signer2.key);
        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn called_by_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Holla, I'm the owner");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}
