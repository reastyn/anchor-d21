// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod d21_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn initialize(
        client: &Client,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                d21::instruction::Initialize {},
                d21::accounts::Initialize {
                    basic_info: a_basic_info,
                    initializer: a_initializer,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_ix(
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: d21::instruction::Initialize {}.data(),
            accounts: d21::accounts::Initialize {
                basic_info: a_basic_info,
                initializer: a_initializer,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn add_subject(
        client: &Client,
        i_name: String,
        a_subject: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                d21::instruction::AddSubject { name: i_name },
                d21::accounts::AddSubject {
                    subject: a_subject,
                    initializer: a_initializer,
                    system_program: a_system_program,
                    basic_info: a_basic_info,
                },
                signers,
            )
            .await?)
    }
    pub fn add_subject_ix(
        i_name: String,
        a_subject: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: d21::instruction::AddSubject { name: i_name }.data(),
            accounts: d21::accounts::AddSubject {
                subject: a_subject,
                initializer: a_initializer,
                system_program: a_system_program,
                basic_info: a_basic_info,
            }
            .to_account_metas(None),
        }
    }
    pub async fn add_voter(
        client: &Client,
        i__voter: Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                d21::instruction::AddVoter { _voter: i__voter },
                d21::accounts::AddVoter {
                    voter: a_voter,
                    initializer: a_initializer,
                    system_program: a_system_program,
                    basic_info: a_basic_info,
                },
                signers,
            )
            .await?)
    }
    pub fn add_voter_ix(
        i__voter: Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: d21::instruction::AddVoter { _voter: i__voter }.data(),
            accounts: d21::accounts::AddVoter {
                voter: a_voter,
                initializer: a_initializer,
                system_program: a_system_program,
                basic_info: a_basic_info,
            }
            .to_account_metas(None),
        }
    }
    pub async fn vote(
        client: &Client,
        i_subject: Pubkey,
        i_is_positive_vote: bool,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_subject: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                d21::instruction::Vote {
                    subject: i_subject,
                    is_positive_vote: i_is_positive_vote,
                },
                d21::accounts::Vote {
                    voter: a_voter,
                    subject: a_subject,
                    basic_info: a_basic_info,
                    initializer: a_initializer,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn vote_ix(
        i_subject: Pubkey,
        i_is_positive_vote: bool,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_subject: anchor_lang::solana_program::pubkey::Pubkey,
        a_basic_info: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: d21::instruction::Vote {
                subject: i_subject,
                is_positive_vote: i_is_positive_vote,
            }
            .data(),
            accounts: d21::accounts::Vote {
                voter: a_voter,
                subject: a_subject,
                basic_info: a_basic_info,
                initializer: a_initializer,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
