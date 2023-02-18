use anchor_lang::solana_program::program_pack::IsInitialized;

use crate::*;

#[account]
pub struct VoterAccount {
    pub is_initialized: bool,
    pub voted_negatively_once: bool,
    pub second_vote_address: Option<Pubkey>,
    pub first_vote_address: Option<Pubkey>,
}

impl IsInitialized for VoterAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, voter_address: Pubkey)]
pub struct AddVoter<'info> {
    #[account(init, payer = initializer, space = 8 + 1 + 1 + (32 + 1) + (32 + 1), seeds = [b"voter", voter_address.key().as_ref()], bump)]
    pub voter: Account<'info, VoterAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(seeds = [b"basic_info"], bump=bump)]
    pub basic_info: Account<'info, BasicInfo>,
}

impl<'info> AddVoter<'_> {
    #[access_control(Self::constraints(&self))]
    pub fn process(&mut self) -> Result<()> {
        let voter = &mut self.voter;

        voter.is_initialized = true;
        voter.voted_negatively_once = false;

        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        let basic_info = &self.basic_info;
        only_in_voting_period(basic_info)?;
        only_owner(basic_info, &self.initializer)?;

        Ok(())
    }
}
