use crate::*;

#[account]
pub struct SubjectAccount {
    pub votes: i64,
    pub name: String,
}

#[derive(Accounts)]
#[instruction(bump: u8, name: String)]
pub struct AddSubject<'info> {
    #[account(init, payer = initializer, space = 8 + 1 + 8 + (4 + 64), seeds = [b"subject", initializer.key().as_ref()], bump)]
    pub subject: Account<'info, SubjectAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(seeds = [b"basic_info"], bump=bump)]
    pub basic_info: Account<'info, BasicInfo>,
}

impl<'info> AddSubject<'_> {
    #[access_control(Self::constraints(&self, &name))]
    pub fn process(&mut self, name: String) -> Result<()> {
        let subject = &mut self.subject;

        subject.votes = 0;
        subject.name = name;

        Ok(())
    }

    pub fn constraints(&self, name: &String) -> Result<()> {
        if name.len() > 64 {
            return Err(D21ErrorCode::NameTooLong.into());
        }
        let basic_info = &self.basic_info;
        only_in_voting_period(basic_info)?;

        Ok(())
    }
}
