use anchor_lang::{prelude::*};

#[account]
pub struct BasicInfo {
    pub owner: Pubkey,
    pub end_date: i64,
    pub bump: u8,
}

const DAY_IN_SECONDS: u32 = 86400;

#[derive(Accounts)]
#[instruction(election_duration_days: u32)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8 + 1, seeds = [b"basic_info"], bump)]
    pub basic_info: Account<'info, BasicInfo>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'_> {
    pub fn process(&mut self, bump: u8, election_duration_days: u32) -> Result<()> {
        let basic_info = &mut self.basic_info;

        let clock = Clock::get()?;
        let end_date = clock.unix_timestamp + (election_duration_days * DAY_IN_SECONDS) as i64;

        basic_info.bump = bump;
        basic_info.end_date = end_date;
        basic_info.owner = self.initializer.key();

        Ok(())
    }
}
