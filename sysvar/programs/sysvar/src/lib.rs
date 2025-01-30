use anchor_lang::prelude::*;

declare_id!("ATKvj2DHbdboCjBdHvVvNpix6j9v58vUsFzwJQb95C6f");

#[program]
pub mod sysvar {
    use super::*;
    use anchor_lang::solana_program::sysvar::instructions;
    use chrono::*;

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        let arr = [ctx.accounts.stake_history.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let stake_history = StakeHistory::from_account_info(sh_sysvar_info)?;
        msg!("Stake history: {:#?}", stake_history);

        let arr = [ctx.accounts.instruction_sysvar.clone()];
        let account_info_iter = &mut arr.iter();
        let instructions_sysvar_account = next_account_info(account_info_iter)?;
        let instruction_details =
            instructions::load_instruction_at_checked(0, instructions_sysvar_account)?;
        msg!(
            "Instruction details of this transaction: {:?}",
            instruction_details
        );
        msg!("Number is: {}", number);
        Ok(())
    }

    pub fn get_day_of_the_week(_ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        msg!("Block timestamp: {}", clock.unix_timestamp);
        let time_stamp = clock.unix_timestamp; // current timestamp

        let date_time = chrono::DateTime::from_timestamp(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!("Week day is: {}", day_of_the_week);

        let epoch_schedule = EpochSchedule::get();
        msg!("Epoch schedule: {:#?}", epoch_schedule);

        let rent = Rent::get();
        msg!("Rent: {:#?}", rent);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>,
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
}
