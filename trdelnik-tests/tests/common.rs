use d21::D21ErrorCode;
use d21;
use fehler::throws;
use program_client::d21_instruction::{self, PROGRAM_ID};
use trdelnik_client::{
    anchor_client::solana_client::{
        client_error::ClientErrorKind,
        rpc_request::{self, RpcResponseErrorData},
    },
    anyhow::Result,
    solana_sdk::{instruction::InstructionError, transaction::TransactionError},
    *,
};

pub struct InitialFixture {
    pub client: Client,
    pub basic_info: (Pubkey, u8),
    pub program: Pubkey,
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
        // self.owner
        //     .airdrop(self.owner.payer().pubkey(), 5_000_000_000)
        //     .await?;
        // self.client.deploy_by_name(&self.program, "d21").await?;
        // println!("Deployed program: {:?}", self.program.pubkey());
    }

    pub async fn init(&mut self) -> Result<()> {
        d21_instruction::initialize(
            &self.client,
            d21::instruction::Initialize {},
            d21::accounts::Initialize {
                basic_info: self.basic_info.0,
                initializer: self.client.payer().pubkey(),
                system_program: System::id(),
            },
            Some(self.client.payer().clone()),
        )
        .await?;
        Ok(())
    }

    pub fn new(client: Client) -> Self {
        let program_id = PROGRAM_ID;
        InitialFixture {
            client: client,
            program: program_id.clone(),
            basic_info: Pubkey::find_program_address(&[b"basic_info"], &program_id),
        }
    }
}

pub struct SubjectFixture {
    pub client: Client,
    pub subject: (Pubkey, u8),
}

impl SubjectFixture {
    pub fn new(client: Client, subject: Keypair, program_id: &Pubkey) -> Self {
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
        d21::instruction::AddSubject { name: name.clone() },
        d21::accounts::AddSubject {
            basic_info: common_fixture.basic_info.0,
            subject: subject_fixture.subject.0,
            initializer: subject_fixture.client.payer().pubkey(),
            system_program: System::id(),
        },
        Some(subject_fixture.client.payer().clone()),
    )
    .await
}

pub async fn add_voter(
    common_fixture: &InitialFixture,
    voter_fixture: &VoterFixture,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    d21_instruction::add_voter(
        &common_fixture.client,
        d21::instruction::AddVoter {
            _voter: voter_fixture.pubkey.pubkey(),
        },
        d21::accounts::AddVoter {
            basic_info: common_fixture.basic_info.0,
            voter: voter_fixture.account.0,
            initializer: common_fixture.client.payer().pubkey(),
            system_program: System::id(),
        },
        Some(common_fixture.client.payer().clone()),
    )
    .await
}

pub fn initialize_validator() -> Validator {
    let mut validator = Validator::default();
    validator.add_program("d21", PROGRAM_ID);
    validator
}

pub struct VoterFixture {
    pub account: (Pubkey, u8),
    pub pubkey: Keypair,
    pub client: Client,
}

impl VoterFixture {
    pub fn new(client: Client, voter: Keypair, program_id: &Pubkey) -> Self {
        VoterFixture {
            account: Pubkey::find_program_address(
                &[b"voter", voter.pubkey().as_ref()],
                &program_id,
            ),
            pubkey: voter.clone(),
            client,
        }
    }

    #[throws]
    pub async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }
}
