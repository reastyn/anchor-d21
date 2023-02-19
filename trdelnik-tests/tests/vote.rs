// mod common;
// mod add_voter;
// mod add_subject;

// use common::*;
// use add_subject::{add_subject, SubjectFixture};
// use add_voter::{add_voter, VoterFixture};
// use add_subject::*;
// use add_voter::*;

use trdelnik_client::trdelnik_test;

// #[trdelnik_test]
// async fn test_voting(#[future] init_fixture: Result<Fixture>) {
    
// }

// struct Fixture {
//     common: InitialFixture,
//     voter: VoterFixture,
// }
// impl Fixture {
//     fn new() -> Self {
//         let voter = system_keypair(2);
//         let common = InitialFixture::new();
//         let program_id = common.program.pubkey().clone();
//         let voter_fixture = VoterFixture::new(voter, &program_id);
//         Fixture {
//             common,
//             voter: voter_fixture,
//         }
//     }

//     #[throws]
//     async fn deploy(&mut self) {
//         self.common.deploy().await?;
//     }
// }
