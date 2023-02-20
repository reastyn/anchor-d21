use anchor_lang::prelude::*;

declare_id!("Poo5jhFcGjMjYaz2cpmSNVq4ehvjKJhjU7aCZiS2LMP");

mod errors;
mod instructions;

pub use errors::*;
pub use instructions::*;

#[program]
pub mod d21 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn add_subject(ctx: Context<AddSubject>, _bump: u8, name: String) -> Result<()> {
        ctx.accounts.process(name)
    }

    pub fn add_voter(ctx: Context<AddVoter>, _bump: u8, _voter: Pubkey) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn vote(
        ctx: Context<Vote>,
        _voter_bump: u8,
        _subject_bump: u8,
        _basic_info_bump: u8,
        subject: Pubkey,
        is_positive_vote: bool,
    ) -> Result<()> {
        ctx.accounts.process(subject, is_positive_vote)
    }
}

// #[derive(Accounts)]
// pub struct AddSubject {}
