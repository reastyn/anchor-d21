mod common;

pub use common::*;
use d21::SubjectAccount;
use fehler::throws;
use program_client::d21_instruction;
use trdelnik_client::{
    anchor_lang::AccountDeserialize, anyhow::Result, solana_sdk::account::ReadableAccount,
    ClientError, *,
};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut validator = initialize_validator();
    let client = validator.start().await;
    let mut fixture = Fixture::new(client);
    // @todo: here you can call your <program>::initialize instruction
    fixture.deploy().await?;
    fixture.common.init().await?;

    add_subject(
        &fixture.common,
        &fixture.subject,
        &"Mr Blue Sky".to_string(),
    )
    .await?;
    add_subject(
        &fixture.common,
        &fixture.subject2,
        &"Mr Green Sky".to_string(),
    )
    .await?;
    add_subject(
        &fixture.common,
        &fixture.subject3,
        &"Mr Yellow Sky".to_string(),
    )
    .await?;
    add_subject(
        &fixture.common,
        &fixture.subject4,
        &"Mr White Sky".to_string(),  
    )
    .await?;
    add_voter(&fixture.common, &fixture.voter).await?;

    fixture
}

async fn vote(
    common: &InitialFixture,
    voter: &VoterFixture,
    subject: &SubjectFixture,
    positive_vote: bool,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    d21_instruction::vote(
        &voter.client,
        d21::instruction::Vote {
            subject: subject.client.payer().pubkey(),
            is_positive_vote: positive_vote,
        },
        d21::accounts::Vote {
            voter: voter.account.0,
            subject: subject.subject.0,
            basic_info: common.basic_info.0,
            initializer: voter.client.payer().pubkey(),
            system_program: System::id(),
        },
        Some(voter.client.payer().clone()),
    )
    .await
}

async fn vote_and_check_votes(
    fixture: &Fixture,
    subject: &SubjectFixture,
    positive_vote: bool,
    votes: i64,
) -> Result<(), ClientError> {
    vote(&fixture.common, &fixture.voter, subject, positive_vote).await?;
    let subject = subject.client.get_account(subject.subject.0).await?;
    assert_eq!(false, subject.is_none());
    let subject = SubjectAccount::try_deserialize(&mut subject.unwrap().data()).unwrap();
    assert_eq!(subject.votes, votes);
    Ok(())
}

#[trdelnik_test]
async fn test_voting(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    vote_and_check_votes(&fixture, &fixture.subject, true, 1).await?;
}

#[trdelnik_test]
async fn test_voting_twice_for_same_person(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    vote(&fixture.common, &fixture.voter, &fixture.subject, true).await?;
    let result = vote(&fixture.common, &fixture.voter, &fixture.subject, true).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    check_custom_err(&err, d21::D21ErrorCode::VoteForSameSubjectTwice);
}

#[trdelnik_test]
async fn test_voting_second_time(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    vote_and_check_votes(&fixture, &fixture.subject, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject2, true, 1).await?;
}

#[trdelnik_test]
async fn test_voting_positively_three_times(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    vote_and_check_votes(&fixture, &fixture.subject, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject2, true, 1).await?;
    let result = vote(&fixture.common, &fixture.voter, &fixture.subject3, true).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    check_custom_err(&err, d21::D21ErrorCode::NoMorePositiveVotes);
}

#[trdelnik_test]
async fn should_voting_correctly_three_times(
    #[future] init_fixture: Result<Fixture>,
) -> Result<(), ClientError> {
    let fixture = init_fixture.await?;

    vote_and_check_votes(&fixture, &fixture.subject, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject2, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject3, false, -1).await?;
}

#[trdelnik_test]
async fn should_not_negatively_vote_twice(
    #[future] init_fixture: Result<Fixture>,
) -> Result<(), ClientError> {
    let fixture = init_fixture.await?;

    vote_and_check_votes(&fixture, &fixture.subject, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject2, true, 1).await?;
    vote_and_check_votes(&fixture, &fixture.subject3, false, -1).await?;
    let result = vote(&fixture.common, &fixture.voter, &fixture.subject4, false).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    check_custom_err(&err, d21::D21ErrorCode::NoMoreNegativeVotes);
}

#[trdelnik_test]
async fn should_not_negatively_before_positive(
    #[future] init_fixture: Result<Fixture>,
) -> Result<(), ClientError> {
    let fixture = init_fixture.await?;

    let result = vote(&fixture.common, &fixture.voter, &fixture.subject, false).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    check_custom_err(&err, d21::D21ErrorCode::NegativeVotesAfterTwoPositive);
}

struct Fixture {
    common: InitialFixture,
    voter: VoterFixture,
    subject: SubjectFixture,
    subject2: SubjectFixture,
    subject3: SubjectFixture,
    subject4: SubjectFixture,
}
impl Fixture {
    fn new(client: Client) -> Self {
        let voter = system_keypair(2);
        let common = InitialFixture::new(client.clone());
        let program_id = common.program.clone();
        let voter_fixture = VoterFixture::new(
            client.clone_with_payer(voter.clone()),
            voter.clone(),
            &program_id,
        );
        let subject = system_keypair(1);
        let subject2 = system_keypair(3);
        Fixture {
            subject: SubjectFixture::new(
                client.clone_with_payer(subject.clone()),
                subject,
                &program_id,
            ),
            subject2: SubjectFixture::new(
                client.clone_with_payer(subject2.clone()),
                subject2,
                &program_id,
            ),
            subject3: SubjectFixture::new(
                client.clone_with_payer(system_keypair(4)),
                system_keypair(4),
                &program_id,
            ),
            subject4: SubjectFixture::new(
                client.clone_with_payer(system_keypair(5)),
                system_keypair(5),
                &program_id,
            ),
            common,
            voter: voter_fixture,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.common.deploy().await?;
        self.subject.deploy().await?;
        self.subject2.deploy().await?;
        self.subject3.deploy().await?;
        self.subject4.deploy().await?;
        self.voter.deploy().await?;
    }
}
