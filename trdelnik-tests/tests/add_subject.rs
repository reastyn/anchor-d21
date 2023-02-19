mod common;

use common::*;
use d21::{D21ErrorCode, SubjectAccount};
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

    fixture
}

pub async fn add_subject(
    common_fixture: &InitialFixture,
    subject_fixture: &SubjectFixture,
    name: &String,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    d21_instruction::add_subject(
        &subject_fixture.client,
        common_fixture.basic_info.1,
        name.clone(),
        subject_fixture.subject.0,
        subject_fixture.client.payer().pubkey(),
        System::id(),
        common_fixture.basic_info.0,
        Some(subject_fixture.client.payer().clone()),
    )
    .await
}

#[trdelnik_test]
async fn test_add_subject(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    let name = "Subjectus maximus".to_string();

    add_subject(&fixture.common, &fixture.subject, &name).await?;

    let subject = fixture
        .subject
        .client
        .get_account(fixture.subject.subject.0)
        .await?;
    assert_eq!(false, subject.is_none());
    let subject = SubjectAccount::try_deserialize(&mut subject.unwrap().data()).unwrap();

    assert_eq!(subject.name, name);
    assert_eq!(subject.votes, 0);
    assert_eq!(subject.is_initialized, true);
}

#[trdelnik_test]
async fn test_add_subject_twice(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    let name = "Subjectus maximus".to_string();
    add_subject(&fixture.common, &fixture.subject, &name).await?;
    let result = add_subject(&fixture.common, &fixture.subject, &name).await;
    assert!(result.is_err());
}

#[trdelnik_test]
async fn test_add_subject_longer_name(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    let name = "A name that is longer than 64 characters should not be accepted :)".to_string();
    let result = add_subject(&fixture.common, &fixture.subject, &name).await;
    assert!(result.is_err());
    let err = result.err().unwrap();

    check_custom_err(&err, D21ErrorCode::NameTooLong);
}

pub struct SubjectFixture {
    client: Client,
    subject: (Pubkey, u8),
}

impl SubjectFixture {
    pub fn new(subject: Keypair, program_id: &Pubkey) -> Self {
        let client = Client::new(subject.clone());
        let program_id = program_id;
        let subject =
            Pubkey::find_program_address(&[b"subject", subject.pubkey().as_ref()], &program_id);
        Self { client, subject }
    }

    #[throws]
    pub async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }
}

struct Fixture {
    common: InitialFixture,
    subject: SubjectFixture,
}
impl Fixture {
    fn new() -> Self {
        let subject = system_keypair(1);
        let common = InitialFixture::new();
        let program_id = common.program.pubkey().clone();
        Fixture {
            subject: SubjectFixture::new(subject, &program_id),
            common,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.common.deploy().await?;
        self.subject.deploy().await?;
    }
}
