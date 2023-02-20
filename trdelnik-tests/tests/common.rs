use d21::D21ErrorCode;
use fehler::throws;
use program_client::d21_instruction;
use trdelnik_client::{
    anchor_client::solana_client::{
        client_error::ClientErrorKind,
        rpc_request::{self, RpcError, RpcResponseErrorData},
    },
    anyhow::Result,
    solana_sdk::{instruction::InstructionError, transaction::TransactionError},
    *,
};

pub struct InitialFixture {
    pub owner: Client,
    pub basic_info: (Pubkey, u8),
    pub program: Keypair,
}

// Can you do it somehow better please?
pub fn check_custom_err(err: &ClientError, custom_err: D21ErrorCode) {
    if let ClientError::SolanaClientError(solana_client_err) = err {
        if let ClientErrorKind::RpcError(rpc_request::RpcError::RpcResponseError { data, .. }) =
            solana_client_err.kind()
        {
            if let RpcResponseErrorData::SendTransactionPreflightFailure(rpc_err) = data {
                let err = rpc_err.clone().err.unwrap();
                if let TransactionError::InstructionError(_, err) = err {
                    if let InstructionError::Custom(err) = err {
                        assert_eq!(err, (custom_err as u32) + 6000);
                        return;
                    }
                }
            }
        }
    }
    panic!("Unexpected error: {:?}", err);
}

impl InitialFixture {
    #[throws]
    pub async fn deploy(&mut self) {
        self.owner
            .airdrop(self.owner.payer().pubkey(), 5_000_000_000)
            .await?;
        self.owner.deploy_by_name(&self.program, "d21").await?;
    }

    pub async fn init(&mut self) -> Result<()> {
        d21_instruction::initialize(
            &self.owner,
            self.basic_info.0,
            self.owner.payer().pubkey(),
            System::id(),
            Some(self.owner.payer().clone()),
        )
        .await?;
        Ok(())
    }

    pub fn new() -> Self {
        let program_id = program_keypair(0);
        InitialFixture {
            owner: Client::new(system_keypair(0)),
            program: program_id.clone(),
            basic_info: Pubkey::find_program_address(&[b"basic_info"], &program_id.pubkey()),
        }
    }
}

pub struct SubjectFixture {
    pub client: Client,
    pub subject: (Pubkey, u8),
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

pub struct VoterFixture {
    pub account: (Pubkey, u8),
    pub pubkey: Keypair,
    pub client: Client,
}

impl VoterFixture {
    pub fn new(voter: Keypair, program_id: &Pubkey) -> Self {
        VoterFixture {
            account: Pubkey::find_program_address(
                &[b"voter", voter.pubkey().as_ref()],
                &program_id,
            ),
            pubkey: voter.clone(),
            client: Client::new(voter),
        }
    }

    #[throws]
    pub async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }
}
