use crate::*;

#[error_code]
pub enum D21ErrorCode {
    #[msg("Specified name for subject is too long.")]
    NameTooLong,
    #[msg("Election has already ended.")]
    ElectionEnded,
    #[msg("You are not the owner of this election.")]
    NotOwner,
    #[msg("You have already voted for this subject.")]
    VoteForSameSubjectTwice,
    #[msg("You have no more positive votes left.")]
    NoMorePositiveVotes,
    #[msg("You have already voted negatively once.")]
    NoMoreNegativeVotes,
    #[msg("You can only vote negatively after voting positively twice.")]
    NegativeVotesAfterTwoPositive,
}

pub fn only_in_voting_period(basic_info: &Account<BasicInfo>) -> Result<()> {
    if basic_info.end_date < Clock::get()?.unix_timestamp {
        return Err(D21ErrorCode::ElectionEnded.into());
    }
    Ok(())
}

pub fn only_owner(basic_info: &Account<BasicInfo>, initializer: &Signer) -> Result<()> {
    if basic_info.owner != *initializer.key {
        return Err(D21ErrorCode::NotOwner.into());
    }
    Ok(())
}
