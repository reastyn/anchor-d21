use std::{
    any::{Any, TypeId},
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

use program_client::d21_instruction::PROGRAM_ID;
use trdelnik_client::*;

type AnyState = HashMap<TypeId, Box<dyn Any>>;

struct FuzzTestBuilder {
    validator: Validator,
    flows: Vec<Box<dyn Fn(&PassableState)>>,
    passable_state: PassableState,
}

struct PassableState {
    state: AnyState,
    client: Option<Client>,
}

impl PassableState {
    fn client(&self) -> Client {
        self.client
            .as_ref()
            .expect("You probably forgot to call the `start` method before accessing the client.")
            .clone()
    }
}

impl FuzzTestBuilder {
    fn new(validator: Validator) -> Self {
        FuzzTestBuilder {
            validator,
            flows: vec![],
            passable_state: PassableState {
                state: HashMap::new(),
                client: None,
            },
        }
    }

    fn add_flow<F, Args>(&mut self, flow: F) -> &mut Self
    where
        F: Handler<Args> + 'static,
    {
        let boxed_flow = Box::new(move |passable_state: &PassableState| {
            println!("Called");
            flow.call(passable_state);
            println!("Called2");
        });
        self.flows.push(boxed_flow);
        self
    }

    async fn start(&mut self) {
        let client = self.validator.start().await;
        self.passable_state.client = Some(client);
        for flow in self.flows.iter() {
            (*flow)(&self.passable_state);
        }
    }

    fn with_state<S: 'static>(&mut self, state: S) -> &mut Self {
        println!("{:?}", TypeId::of::<S>());
        self.passable_state
            .state
            .insert(TypeId::of::<S>(), Box::new(Rc::new(RefCell::new(state))));
        println!("{:?}", self.passable_state.state.get(&TypeId::of::<S>()));
        self
    }
}

trait Handler<T> {
    fn call(&self, builder: &PassableState);
}

trait FromPassable {
    fn from_passable(builder: &PassableState) -> Self;
}
pub struct State<S: 'static>(pub Rc<RefCell<S>>);

impl<T: 'static> FromPassable for State<T> {
    fn from_passable(builder: &PassableState) -> State<T> {
        let state = builder
            .state
            .get(&TypeId::of::<T>())
            .expect("State not found")
            .downcast_ref::<Rc<RefCell<T>>>()
            .expect("State type mismatch");
        State(state.clone())
    }
}

impl<F, A> Handler<A> for F
where
    F: Fn(A),
    A: FromPassable,
{
    fn call(&self, fuzz_test_builder: &PassableState) {
        let a = A::from_passable(&fuzz_test_builder);
        (*self)(a)
    }
}

impl<F, A, B> Handler<(A, B)> for F
where
    F: Fn(A, B),
    A: FromPassable,
    B: FromPassable,
{
    fn call(&self, fuzz_test_builder: &PassableState) {
        let a = A::from_passable(&fuzz_test_builder);
        let b = B::from_passable(&fuzz_test_builder);
        (*self)(a, b)
    }
}

impl FromPassable for Client {
    fn from_passable(builder: &PassableState) -> Self {
        builder.client()
    }
}

struct TestState {
    owner: Pubkey,
}

struct TestState2 {
    counter: u64,
}

#[tokio::main]
async fn main() {
    let mut validator = Validator::default();
    validator.add_program("d21", PROGRAM_ID);

    fn flow_add_subject(State(state2): State<TestState2>) {
        // println!("{}", client.payer().pubkey());
        // println!("{}", state.borrow_mut().owner);
        state2.borrow_mut().counter += 1
    }

    // fn invariant_add_subject(validator: &mut Validator, client: &mut Client) {}

    FuzzTestBuilder::new(validator)
        .add_flow(flow_add_subject)
        .add_flow(
            |State(_state): State<TestState>, State(state2): State<TestState2>| {
                // println!("{}", state.owner);
                println!("{}", state2.borrow().counter);
            },
        )
        .with_state(TestState {
            owner: Pubkey::new_unique(),
        })
        .with_state(TestState2 { counter: 5 })
        .start().await;
    // tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    // .invariant(invariant_add_subject);
}
