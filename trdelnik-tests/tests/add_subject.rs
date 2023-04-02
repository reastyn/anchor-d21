mod common;

use common::*;
use d21::{D21ErrorCode, SubjectAccount};
use fehler::throws;
use program_client::d21_instruction::PROGRAM_ID;
use trdelnik_client::{
    anchor_lang::AccountDeserialize, anyhow::Result, solana_sdk::account::ReadableAccount, *,
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

    fixture
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

struct Fixture {
    common: InitialFixture,
    subject: SubjectFixture,
}
impl Fixture {
    fn new(client: Client) -> Self {
        let subject = system_keypair(1);
        let common = InitialFixture::new(client.clone());
        let program_id = common.program.clone();
        Fixture {
            subject: SubjectFixture::new(
                client.clone_with_payer(subject.clone()),
                subject,
                &program_id,
            ),
            common,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.common.deploy().await?;
        self.subject.deploy().await?;
    }
}
