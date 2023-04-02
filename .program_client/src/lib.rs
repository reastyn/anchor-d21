// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod d21_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        123u8, 42u8, 235u8, 70u8, 255u8, 185u8, 241u8, 198u8, 26u8, 138u8, 23u8, 141u8, 180u8,
        182u8, 235u8, 46u8, 55u8, 31u8, 149u8, 95u8, 167u8, 210u8, 0u8, 32u8, 215u8, 238u8, 88u8,
        141u8, 111u8, 188u8, 87u8, 131u8,
    ]);
    pub async fn initialize(
        client: &Client,
        parameters: d21::instruction::Initialize,
        accounts: d21::accounts::Initialize,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn initialize_ix(
        parameters: d21::instruction::Initialize,
        accounts: d21::accounts::Initialize,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
    pub async fn add_subject(
        client: &Client,
        parameters: d21::instruction::AddSubject,
        accounts: d21::accounts::AddSubject,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn add_subject_ix(
        parameters: d21::instruction::AddSubject,
        accounts: d21::accounts::AddSubject,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
    pub async fn add_voter(
        client: &Client,
        parameters: d21::instruction::AddVoter,
        accounts: d21::accounts::AddVoter,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn add_voter_ix(
        parameters: d21::instruction::AddVoter,
        accounts: d21::accounts::AddVoter,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
    pub async fn vote(
        client: &Client,
        parameters: d21::instruction::Vote,
        accounts: d21::accounts::Vote,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(PROGRAM_ID, parameters, accounts, signers)
            .await?)
    }
    pub fn vote_ix(
        parameters: d21::instruction::Vote,
        accounts: d21::accounts::Vote,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: parameters.data(),
            accounts: accounts.to_account_metas(None),
        }
    }
}
