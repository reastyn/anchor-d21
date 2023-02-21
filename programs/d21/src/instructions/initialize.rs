use anchor_lang::{prelude::*};

#[account]
pub struct BasicInfo {
    pub owner: Pubkey,
    pub end_date: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8 + 1, seeds = [b"basic_info"], bump)]
    pub basic_info: Account<'info, BasicInfo>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'_> {
    pub fn process(&mut self) -> Result<()> {
        let basic_info = &mut self.basic_info;

        let clock = Clock::get()?;
        let end_date = clock.unix_timestamp + 2628000;

        basic_info.end_date = end_date;
        basic_info.owner = self.initializer.key();

        Ok(())
    }
}
