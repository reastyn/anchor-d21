mod common;

use common::*;
use d21::SubjectAccount;
use fehler::throws;
use program_client::d21_instruction;
use trdelnik_client::{
    anchor_client::solana_client::client_error::ClientErrorKind,
    anchor_lang::AccountDeserialize,
    anyhow::Result,
    solana_sdk::{account::ReadableAccount, transaction::TransactionError},
    ClientError, *,
};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    // @todo: here you can call your <program>::initialize instruction
    fixture.deploy().await?;
    fixture.common.init().await?;

    add_subject(
        &fixture.common,
        &fixture.subject,
        &"Mr Blue Sky".to_string(),
    )
    .await?;
    // add_subject(
    //     &fixture.common,
    //     &fixture.subject2,
    //     &"Mr Green Sky".to_string(),
    // )
    // .await?;
    add_voter(&fixture.common, &fixture.voter).await?;

    fixture
}

async fn vote(
    common: &InitialFixture,
    voter: &VoterFixture,
    subject: &SubjectFixture,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    d21_instruction::vote(
        &voter.client,
        subject.client.payer().pubkey(),
        true,
        voter.account.0,
        subject.subject.0,
        common.basic_info.0,
        voter.client.payer().pubkey(),
        System::id(),
        Some(voter.client.payer().clone()),
    )
    .await
}

#[trdelnik_test]
async fn test_voting(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    vote(&fixture.common, &fixture.voter, &fixture.subject).await?;

    let subject = fixture
        .subject
        .client
        .get_account(fixture.subject.subject.0)
        .await?;
    assert_eq!(false, subject.is_none());
    let subject = SubjectAccount::try_deserialize(&mut subject.unwrap().data()).unwrap();
    assert_eq!(subject.votes, 1);
}

// #[trdelnik_test]
// async fn test_voting_twice_for_same_person(#[future] init_fixture: Result<Fixture>) {
//     let fixture = init_fixture.await?;
//     vote(&fixture.common, &fixture.voter, &fixture.subject).await?;
//     let result = vote(&fixture.common, &fixture.voter, &fixture.subject).await;
//     assert!(result.is_err());
//     let err = result.err().unwrap();
//     check_custom_err(&err, d21::D21ErrorCode::VoteForSameSubjectTwice);
// }

struct Fixture {
    common: InitialFixture,
    voter: VoterFixture,
    subject: SubjectFixture,
    subject2: SubjectFixture,
}
impl Fixture {
    fn new() -> Self {
        let voter = system_keypair(2);
        let common = InitialFixture::new();
        let program_id = common.program.pubkey().clone();
        let voter_fixture = VoterFixture::new(voter.clone(), &program_id);
        let subject = system_keypair(1);
        Fixture {
            subject: SubjectFixture::new(subject, &program_id),
            subject2: SubjectFixture::new(system_keypair(3), &program_id),
            common,
            voter: voter_fixture,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.common.deploy().await?;
        self.subject.deploy().await?;
        self.subject2.deploy().await?;
        self.voter.deploy().await?;
    }
}
