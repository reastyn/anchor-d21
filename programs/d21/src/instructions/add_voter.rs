
use crate::*;

#[account]
pub struct VoterAccount {
    pub voted_negatively_once: bool,
    pub second_vote_address: Option<Pubkey>,
    pub first_vote_address: Option<Pubkey>,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(voter_address: Pubkey)]
pub struct AddVoter<'info> {
    #[account(init, payer = initializer, space = 8 + 1 + (32 + 1) + (32 + 1) + 1, seeds = [b"voter", voter_address.key().as_ref()], bump)]
    pub voter: Account<'info, VoterAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(seeds = [b"basic_info"], bump=basic_info.bump)]
    pub basic_info: Account<'info, BasicInfo>,
}

impl<'info> AddVoter<'_> {
    #[access_control(Self::constraints(&self))]
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let voter = &mut self.voter;
        
        voter.bump = bump;
        voter.voted_negatively_once = false;
        voter.second_vote_address = None;
        voter.first_vote_address = None;

        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        let basic_info = &self.basic_info;
        only_in_voting_period(basic_info)?;
        only_owner(basic_info, &self.initializer)?;

        Ok(())
    }
}
