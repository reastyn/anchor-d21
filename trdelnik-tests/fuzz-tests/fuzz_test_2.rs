use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use program_client::d21_instruction::PROGRAM_ID;
use trdelnik_client::*;

type AnyState = HashMap<TypeId, Box<dyn Any>>;

struct FuzzTestBuilder<Args> {
    validator: Validator,
    client: Option<Client>,
    flows: Vec<Box<dyn Handler<Args>>>,
    state: AnyState,
}

impl<Args> FuzzTestBuilder<Args> {
    fn new(validator: Validator) -> Self {
        FuzzTestBuilder {
            validator,
            client: None,
            flows: vec![],
            state: HashMap::new(),
        }
    }

    fn client(&self) -> Client {
        self.client
            .as_ref()
            .expect("You probably forgot to call the `start` method before accessing the client.")
            .clone()
    }

    fn add_flow<F>(&mut self, flow: F) -> &mut Self
    where
        F: Handler<Args> + 'static,
    {
        let boxed_flow = Box::new(flow);
        self.flows.push(boxed_flow);
        self
    }

    async fn start(&mut self) {
        let client = self.validator.start().await;
        self.client = Some(client);
        for flow in self.flows.iter() {
            flow.call(self);
        }
    }

    fn with_state<S: 'static>(&mut self, state: S) -> &mut Self {
        self.state.insert(TypeId::of::<S>(), Box::new(State(state)));
        self
    }
}

trait Handler<Args> {
    fn call(&self, builder: &FuzzTestBuilder<Args>);
}

trait FromFuzzBuilder<Args> {
    fn from_fuzz_builder(builder: &FuzzTestBuilder<Args>) -> Self;
}
pub struct State<S>(pub S);

impl<Args, T> FromFuzzBuilder<Args> for State<T> {
    fn from_fuzz_builder(builder: &FuzzTestBuilder<Args>) -> State<T> {
        let state = builder
            .state
            .get(&TypeId::of::<State<T>>())
            .expect("State not found")
            .downcast_ref::<State<T>>()
            .expect("State type mismatch");
        state.clone()
    }
}

impl<F, A> Handler<A> for F
where
    F: Fn(A),
    A: FromFuzzBuilder<A>,
{
    fn call(&self, fuzz_test_builder: &FuzzTestBuilder<A>) {
        let a = A::from_fuzz_builder(&fuzz_test_builder);
        (*self)(a)
    }
}

impl<F, A, B> Handler<(A, B)> for F
where
    F: Fn(A, B),
    A: FromFuzzBuilder<(A, B)>,
    B: FromFuzzBuilder<(A, B)>,
{
    fn call(&self, fuzz_test_builder: &FuzzTestBuilder<(A, B)>) {
        let a = A::from_fuzz_builder(&fuzz_test_builder);
        let b = B::from_fuzz_builder(&fuzz_test_builder);
        (*self)(a, b)
    }
}

impl<Args> FromFuzzBuilder<Args> for Client {
    fn from_fuzz_builder(builder: &FuzzTestBuilder<Args>) -> Self {
        builder.client()
    }
}

struct TestState {
    owner: Pubkey,
}

#[tokio::main]
async fn main() {
    let mut validator = Validator::default();
    validator.add_program("d21", PROGRAM_ID);

    fn flow_add_subject(client: Client) {
        println!("{}", client.payer().pubkey());
    }

    // fn invariant_add_subject(validator: &mut Validator, client: &mut Client) {}

    FuzzTestBuilder::new(validator)
        .add_flow(flow_add_subject)
        .add_flow(|client: Client| {
            println!("{}", client.payer().pubkey());
        })
        .with_state(TestState {
            owner: Pubkey::new_unique(),
        })
        .start()
        .await;
    // .invariant(invariant_add_subject);
}
