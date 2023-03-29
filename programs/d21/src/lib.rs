use anchor_lang::prelude::*;

declare_id!("GutqpKKVVovBTRdcPaLxyq1ZFji5hYA6cma1YZ5sQfah");

mod errors;
mod instructions;

pub use errors::*;
pub use instructions::*;

#[program]
pub mod d21 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx
            .bumps
            .get("basic_info")
            .ok_or(D21ErrorCode::InvalidBump)?;
        ctx.accounts.process(bump)
    }

    pub fn add_subject(ctx: Context<AddSubject>, name: String) -> Result<()> {
        let bump = *ctx.bumps.get("subject").ok_or(D21ErrorCode::InvalidBump)?;
        ctx.accounts.process(name, bump)
    }

    pub fn add_voter(ctx: Context<AddVoter>, _voter: Pubkey) -> Result<()> {
        let bump = *ctx.bumps.get("voter").ok_or(D21ErrorCode::InvalidBump)?;
        ctx.accounts.process(bump)
    }

    pub fn vote(
        ctx: Context<Vote>,
        subject: Pubkey,
        is_positive_vote: bool,
    ) -> Result<()> {
        ctx.accounts.process(subject, is_positive_vote)
    }
}

// #[derive(Accounts)]
// pub struct AddSubject {}
