use std::ops::Deref;

use d21::{accounts, instruction, SubjectAccount};
use program_client::d21_instruction::{self, add_subject, add_voter, vote, PROGRAM_ID};
use rand::seq::SliceRandom;
use rand::Rng;
use trdelnik_client::{
    random_keypair, trdelnik_fuzz, Client, FutureExt, Id, Keypair, Pubkey, Signer, System,
    Validator,
};
use trdelnik_fuzz::{random_string, FuzzTestBuilder, State};

#[derive(Debug)]
struct CloneableKeypair(Keypair);

impl Clone for CloneableKeypair {
    fn clone(&self) -> Self {
        Self(self.0.insecure_clone())
    }
}

impl Deref for CloneableKeypair {
    type Target = Keypair;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
struct VotingInfo {
    voter: CloneableKeypair,
    voter_acc: Pubkey,
    first_positive_vote: Option<Pubkey>,
    second_positive_vote: Option<Pubkey>,
    third_negative_vote: Option<Pubkey>,
}
#[derive(Clone, Debug)]
struct SubjectInfo {
    subject: Pubkey,
    subject_acc: Pubkey,
    name: String,
    votes: i128,
}

#[derive(Clone, Debug)]
struct SubjectState {
    subjects: Vec<SubjectInfo>,
}

#[derive(Clone, Debug)]
struct VoterState {
    owner: Option<CloneableKeypair>,
    voters: Vec<VotingInfo>,
}

async fn flow_add_subject(client: Client, State(mut test_state): State<SubjectState>) {
    let name = random_string(0, 100);
    let subject = random_keypair();
    client
        .airdrop(subject.pubkey(), 5_000_000)
        .await
        .expect("Unable to airdrop to subject");
    let subject_acc =
        Pubkey::find_program_address(&[b"subject", subject.pubkey().as_ref()], &PROGRAM_ID).0;
    let basic_info = Pubkey::find_program_address(&[b"basic_info"], &PROGRAM_ID).0;

    let res = add_subject(
        &client,
        instruction::AddSubject { name: name.clone() },
        accounts::AddSubject {
            subject: subject_acc,
            initializer: subject.pubkey(),
            system_program: System::id(),
            basic_info,
        },
        Some(subject.insecure_clone()),
    )
    .await;
    if name.len() <= 64 {
        test_state.subjects.push(SubjectInfo {
            subject: subject.pubkey(),
            subject_acc: subject_acc,
            name,
            votes: 0,
        });
        res.expect("Unable to add subject even though name is valid");
    }
}

async fn flow_add_voter(client: Client, State(mut voter_state): State<VoterState>) {
    let voter: Keypair = random_keypair();
    let owner = voter_state
        .owner
        .as_ref()
        .expect("The owner of contract was not initialized");
    client
        .airdrop(voter.pubkey(), 5_000_000)
        .await
        .expect("Unable to airdrop to subject");
    let voter_acc =
        Pubkey::find_program_address(&[b"voter", voter.pubkey().as_ref()], &PROGRAM_ID).0;
    let basic_info = Pubkey::find_program_address(&[b"basic_info"], &PROGRAM_ID).0;

    add_voter(
        &client,
        instruction::AddVoter {
            _voter: voter.pubkey(),
        },
        accounts::AddVoter {
            voter: voter_acc,
            initializer: owner.pubkey(),
            system_program: System::id(),
            basic_info,
        },
        Some(owner.insecure_clone()),
    )
    .await
    .expect("Unable to add voter");

    voter_state.voters.push(VotingInfo {
        voter: CloneableKeypair(voter.insecure_clone()),
        voter_acc,
        first_positive_vote: None,
        second_positive_vote: None,
        third_negative_vote: None,
    });
}

fn random_voter(voters: &mut VoterState) -> Option<&mut VotingInfo> {
    let mut rng = rand::thread_rng();
    voters.voters.choose_mut(&mut rng)
}

fn random_subject<'a, F>(subjects: &'a mut Vec<SubjectInfo>, check_subject: F) -> Option<&mut SubjectInfo>
where
    F: Fn(&SubjectInfo) -> bool,
{
    let mut rng = rand::thread_rng();
    let len = subjects.len();
    for _i in 0..subjects.len() {
        let i = rng.gen_range(0..len);
        let subject = &subjects[i];
        if check_subject(&subject) {
            return Some(&mut subjects[i]);
        }
    }
    return None;
}

async fn call_vote(
    voter_info: &VotingInfo,
    client: &Client,
    is_positive_vote: bool,
    subject_info: &SubjectInfo,
) {
    let basic_info = Pubkey::find_program_address(&[b"basic_info"], &PROGRAM_ID).0;
    vote(
        &client,
        instruction::Vote {
            is_positive_vote,
            subject: subject_info.subject,
        },
        accounts::Vote {
            basic_info,
            subject: subject_info.subject_acc,
            initializer: voter_info.voter.pubkey(),
            system_program: System::id(),
            voter: voter_info.voter_acc,
        },
        Some(voter_info.voter.insecure_clone()),
    )
    .await
    .expect("Unable to vote");
}

async fn flow_vote(
    client: Client,
    State(mut test_state): State<SubjectState>,
    State(mut voter_state): State<VoterState>,
) {
    let voting_info = if let Some(voting_info) = random_voter(&mut voter_state) {
        voting_info
    } else {
        return;
    };

    let subject = random_subject(&mut test_state.subjects, |subject| {
        Some(subject.subject) != voting_info.first_positive_vote
            && Some(subject.subject) != voting_info.second_positive_vote
            && Some(subject.subject) != voting_info.third_negative_vote
    });

    let subject_info = if let Some(subject) = subject {
        subject
    } else {
        return;
    };

    match voting_info {
        VotingInfo {
            first_positive_vote: None,
            ..
        } => {
            println!("first positive vote");
            call_vote(voting_info, &client, true, subject_info).await;
            subject_info.votes += 1;
            voting_info.first_positive_vote = Some(subject_info.subject);
        }
        VotingInfo {
            second_positive_vote: None,
            ..
        } => {
            println!("second positive vote");
            call_vote(voting_info, &client, true, subject_info).await;
            subject_info.votes += 1;
            voting_info.second_positive_vote = Some(subject_info.subject);
        }
        VotingInfo {
            third_negative_vote: None,
            ..
        } => {
            println!("third negative vote");
            call_vote(voting_info, &client, false, subject_info).await;
            subject_info.votes -= 1;
            voting_info.third_negative_vote = Some(subject_info.subject);
        }
        _ => {}
    }
}

async fn invariant(client: Client, State(test_state): State<SubjectState>) {
    let program = client.program(PROGRAM_ID);
    let subjects = program
        .accounts::<SubjectAccount>(vec![])
        .expect("Unable to get subjects");
    assert!(subjects.len() == test_state.subjects.len());
    for subject in test_state.subjects.iter() {
        let subject_acc = program
            .account::<SubjectAccount>(subject.subject_acc)
            .expect("Unable to get subject");
        assert!(subject_acc.name == subject.name);
        assert!(subject_acc.votes as i128 == subject.votes);
    }
}

async fn init_contract(client: Client, State(mut test_state): State<VoterState>) {
    let initializer = client.payer();
    client
        .airdrop(initializer.pubkey(), 5_000_000)
        .await
        .expect("Error airdropping the owner");
    let system_program = System::id();
    let basic_info = Pubkey::find_program_address(&[b"basic_info"], &PROGRAM_ID).0;
    let rng = rand::thread_rng().gen_range(1..500);

    d21_instruction::initialize(
        &client,
        instruction::Initialize {
            election_duration_days: rng,
        },
        accounts::Initialize {
            basic_info,
            initializer: initializer.pubkey(),
            system_program,
        },
        Some(initializer.insecure_clone()),
    )
    .await
    .expect("Unable to initialize contract");

    test_state.owner = Some(CloneableKeypair(initializer.insecure_clone()));
}

fn initialize_validator() -> Validator {
    let mut validator = Validator::default();
    validator.add_program("d21", PROGRAM_ID);
    validator
}

#[trdelnik_fuzz]
async fn main() {

    FuzzTestBuilder::new()
        .initialize_validator(initialize_validator)
        .with_state(SubjectState { subjects: vec![] })
        .with_state(VoterState {
            owner: None,
            voters: vec![],
        })
        .add_init_handler(init_contract)
        .add_invariant(invariant)
        .add_flow(flow_add_subject)
        .add_flow(flow_add_voter)
        .add_flow(flow_vote)
        .start(10, 200)
        .await;
}
