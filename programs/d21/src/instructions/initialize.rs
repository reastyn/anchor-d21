use anchor_lang::{prelude::*, solana_program::program_pack::IsInitialized};

#[account]
pub struct BasicInfo {
    pub owner: Pubkey,
    pub end_date: i64,
    pub is_initialized: bool,
}

impl IsInitialized for BasicInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
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
        basic_info.is_initialized = true;

        Ok(())
    }
}
