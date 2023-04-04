use program_client::d21_instruction::PROGRAM_ID;
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use trdelnik_client::*;

fn random_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() {
    let mut validator = Validator::default();
    validator.add_program("d21", PROGRAM_ID);
    let client = validator.start().await;
    client
        .airdrop(client.payer().pubkey(), 5_000_000_000)
        .await
        .unwrap();
    let basic_info = Pubkey::find_program_address(&[b"basic_info"], &PROGRAM_ID).0;
    program_client::d21_instruction::initialize(
        &client,
        d21::instruction::Initialize {},
        d21::accounts::Initialize {
            basic_info,
            initializer: client.payer().pubkey(),
            system_program: System::id(),
        },
        Some(client.payer().clone()),
    )
    .await
    .unwrap();

    loop {
        let name = random_str(72);
        let subject_keypair = Keypair::new();
        client
            .airdrop(subject_keypair.pubkey(), 5_000_000_000)
            .await
            .unwrap();

        let client = client.clone_with_payer(subject_keypair.clone());
        // println!("subject_keypair: {:?}", subject_keypair.pubkey());
        // println!("payer: {:?}", client.payer().pubkey());

        let balance = client.get_balance(client.payer().pubkey()).await.unwrap();
        // println!("balance: {:?}", balance);

        let subject_add_request = program_client::d21_instruction::add_subject(
            &client,
            d21::instruction::AddSubject { name: name.clone() },
            d21::accounts::AddSubject {
                basic_info,
                initializer: client.payer().pubkey(),
                system_program: System::id(),
                subject: Pubkey::find_program_address(
                    &[b"subject", subject_keypair.pubkey().as_ref()],
                    &PROGRAM_ID,
                )
                .0,
            },
            Some(client.payer().clone()),
        )
        .await;

        if name.len() > 72 {
            println!("name: {}", name);
            assert!(subject_add_request.is_err());
        } else {
            println!("else name: {}", name);
            println!("subject_add_request: {:?}", subject_add_request);
            assert!(subject_add_request.is_ok());
        }
    }

    
}
