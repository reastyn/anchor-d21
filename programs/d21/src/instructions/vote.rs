use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(voter_bump: u8, subject_bump: u8, basic_info_bump: u8, subject_public_key: Pubkey, is_positive_vote: bool)]
pub struct Vote<'info> {
    #[account(mut, seeds = [b"voter", initializer.key().as_ref()], bump=voter_bump)]
    pub voter: Account<'info, VoterAccount>,
    #[account(mut, seeds = [b"subject", subject_public_key.key().as_ref()], bump=subject_bump)]
    pub subject: Account<'info, SubjectAccount>,
    #[account(seeds = [b"basic_info"], bump=basic_info_bump)]
    pub basic_info: Account<'info, BasicInfo>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Vote<'_> {
    pub fn process(&mut self, subject: Pubkey, is_positive_vote: bool) -> Result<()> {
        if is_positive_vote {
            self.vote_positively(subject)
        } else {
            self.vote_negatively(subject)
        }
    }

    #[access_control(Self::constraints(&self))]
    pub fn vote_positively(&mut self, subject: Pubkey) -> Result<()> {
        let voter = &mut self.voter;
        match (voter.first_vote_address, voter.second_vote_address) {
            (None, None) => {
                voter.first_vote_address = Some(subject);
            }
            (Some(first_vote_address), None) => {
                if first_vote_address == subject {
                    return Err(error!(D21ErrorCode::VoteForSameSubjectTwice));
                }
                voter.second_vote_address = Some(subject);
            }
            _ => {
                return Err(error!(D21ErrorCode::NoMorePositiveVotes));
            }
        }

        let subject = &mut self.subject;
        subject.votes += 1;

        Ok(())
    }

    #[access_control(Self::constraints(&self))]
    pub fn vote_negatively(&mut self, subject: Pubkey) -> Result<()> {
        let voter = &mut self.voter;
        if voter.voted_negatively_once {
            return Err(error!(D21ErrorCode::NoMoreNegativeVotes));
        }
        if voter.first_vote_address.is_none() || voter.second_vote_address.is_none() {
            return Err(error!(D21ErrorCode::NegativeVotesAfterTwoPositive));
        }
        if voter.first_vote_address.unwrap() == subject.key()
            || voter.second_vote_address.unwrap() == subject.key()
        {
            return Err(error!(D21ErrorCode::VoteForSameSubjectTwice));
        }

        voter.voted_negatively_once = true;
        let subject_account = &mut self.subject;
        subject_account.votes -= 1;

        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        let basic_info = &self.basic_info;
        only_in_voting_period(basic_info)?;

        Ok(())
    }
}
