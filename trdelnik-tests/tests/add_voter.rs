mod common;

pub use common::*;
use d21::VoterAccount;
use fehler::throws;
use program_client::d21_instruction;
use trdelnik_client::{anchor_lang::AccountDeserialize, anyhow::Result, *};

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
async fn test_add_voter(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    add_voter(&fixture.common, &fixture.voter).await?;

    let voter = fixture
        .common
        .client
        .get_account(fixture.voter.account.0)
        // .account_data_borsh::<VoterAccount>(fixture.voter.account.0)
        .await?;
    assert_eq!(false, voter.is_none());
    let mut account_data = &voter.unwrap().data[..];
    let voter = VoterAccount::try_deserialize(&mut account_data).unwrap();

    assert_eq!(voter.voted_negatively_once, false);
    assert_eq!(voter.second_vote_address, None);
    assert_eq!(voter.first_vote_address, None);
}

#[trdelnik_test]
async fn test_add_voter_twice(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    add_voter(&fixture.common, &fixture.voter).await?;
    let result = add_voter(&fixture.common, &fixture.voter).await;
    assert!(!result.is_ok());
}

#[trdelnik_test]
async fn test_add_voter_not_owner(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    let payer = Keypair::new();
    fixture.common.client.airdrop(payer.pubkey(), 5_000_000).await?;
    let result = d21_instruction::add_voter(
        &fixture.common.client,
        d21::instruction::AddVoter {
            _voter: fixture.voter.pubkey.pubkey(),
        },
        d21::accounts::AddVoter {
            basic_info: fixture.common.basic_info.0,
            voter: fixture.voter.account.0,
            initializer: payer.pubkey(),
            system_program: System::id(),
        },
        Some(payer),
    )
    .await;
    let err = result.err().unwrap();
    check_custom_err(&err, d21::D21ErrorCode::NotOwner);
}

struct Fixture {
    common: InitialFixture,
    voter: VoterFixture,
}
impl Fixture {
    fn new(client: Client) -> Self {
        let voter = system_keypair(2);
        let common = InitialFixture::new(client.clone());
        let program_id = common.program.clone();
        let voter_fixture =
            VoterFixture::new(client.clone_with_payer(voter.clone()), voter, &program_id);

        Fixture {
            common,
            voter: voter_fixture,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.common.deploy().await?;
    }
}
