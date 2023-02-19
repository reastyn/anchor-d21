mod common;

use common::*;
use d21::VoterAccount;
use fehler::throws;
use program_client::d21_instruction;
use trdelnik_client::{
    anchor_lang::AccountDeserialize, anyhow::Result, solana_sdk::account::ReadableAccount, *,
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

pub async fn add_voter(
    common_fixture: &InitialFixture,
    voter_fixture: &VoterFixture,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    d21_instruction::add_voter(
        &common_fixture.owner,
        common_fixture.basic_info.1,
        voter_fixture.pubkey.pubkey(),
        voter_fixture.account.0,
        common_fixture.owner.payer().pubkey(),
        System::id(),
        common_fixture.basic_info.0,
        Some(common_fixture.owner.payer().clone()),
    )
    .await
}

#[trdelnik_test]
async fn test_add_voter(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    add_voter(&fixture.common, &fixture.voter).await?;

    let voter = fixture
        .common
        .owner
        .get_account(fixture.voter.account.0)
        .await?;
    assert_eq!(false, voter.is_none());
    let voter = VoterAccount::try_deserialize(&mut voter.unwrap().data()).unwrap();

    assert_eq!(voter.voted_negatively_once, false);
    assert_eq!(voter.second_vote_address, None);
    assert_eq!(voter.first_vote_address, None);
    assert_eq!(voter.is_initialized, true);
}

#[trdelnik_test]
async fn test_add_voter_twice(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    add_voter(&fixture.common, &fixture.voter).await?;
    let result = add_voter(&fixture.common, &fixture.voter).await;
    assert!(!result.is_ok());
}

pub struct VoterFixture {
    account: (Pubkey, u8),
    pubkey: Keypair,
}

impl VoterFixture {
    fn new(voter: Keypair, program_id: &Pubkey) -> Self {
        VoterFixture {
            account: Pubkey::find_program_address(
                &[b"voter", voter.pubkey().as_ref()],
                &program_id,
            ),
            pubkey: voter,
        }
    }
}

struct Fixture {
    common: InitialFixture,
    voter: VoterFixture,
}
impl Fixture {
    fn new() -> Self {
        let voter = system_keypair(2);
        let common = InitialFixture::new();
        let program_id = common.program.pubkey().clone();
        let voter_fixture = VoterFixture::new(voter, &program_id);
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
