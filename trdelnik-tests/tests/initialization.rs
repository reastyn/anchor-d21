use std::time::{SystemTime, UNIX_EPOCH};

use d21::BasicInfo;
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
    d21_instruction::initialize(
        &fixture.client,
        fixture.basic_info,
        fixture.client.payer().pubkey(),
        System::id(),
        Some(fixture.client.payer().clone()),
    )
    .await?;
    fixture
}

#[trdelnik_test]
async fn test_initialization(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    // TODO: Why does this not work?
    // fixture
    //     .client
    //     .account_data_borsh::<BasicInfo>(fixture.basic_info)
    //     .await?;
    let basic_info = fixture.client.get_account(fixture.basic_info).await?;
    assert_eq!(false, basic_info.is_none());
    let basic_info = BasicInfo::try_deserialize(&mut basic_info.unwrap().data()).unwrap();

    assert_eq!(basic_info.owner, fixture.client.payer().pubkey());
    assert!(basic_info.end_date > 0);
    let now_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 2628000; // month in seconds
    assert!(basic_info.end_date <= i64::try_from(now_unix).unwrap());
}

// @todo: design and implement all the logic you need for your fixture(s)
struct Fixture {
    client: Client,
    program: Keypair,
    basic_info: Pubkey,
}
impl Fixture {
    fn new() -> Self {
        let program_id = program_keypair(0);
        Fixture {
            client: Client::new(system_keypair(0)),
            program: program_id.clone(),
            basic_info: Pubkey::find_program_address(&[b"basic_info"], &program_id.pubkey()).0,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client.deploy_by_name(&self.program, "d21").await?;
    }
}
