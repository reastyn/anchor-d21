use std::{
    any::{Any, TypeId},
    cell::{RefCell, RefMut},
    collections::HashMap,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use program_client::d21_instruction::PROGRAM_ID;
use trdelnik_client::{futures::future::BoxFuture, *};

type MyBoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type SimpleHandler = Box<dyn Fn(Arc<Mutex<PassableState>>) -> MyBoxFuture<()>>;

type AnyState = HashMap<TypeId, Box<dyn Any + Send>>;

struct FuzzTestBuilder {
    validator: Validator,
    flows: Vec<SimpleHandler>,
    passable_state: Arc<Mutex<PassableState>>,
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
            passable_state: Arc::new(Mutex::new(PassableState {
                state: HashMap::new(),
                client: None,
            })),
        }
    }

    fn add_flow<F, Args>(&mut self, flow: F) -> &mut Self
    where
        F: Handler<Args> + 'static,
    {
        // let vec = vec![|test: Arc<PassableState>| Box::pin(flow.call(test.clone()))];
        let boxed_flow: SimpleHandler =
            Box::new(move |passable_state: Arc<Mutex<PassableState>>| {
                let f = flow.clone();
                Box::pin(async move {
                    f.call(passable_state.clone()).await;
                })
            });
        self.flows.push(boxed_flow);
        self
    }

    async fn start(&mut self) {
        // let client = self.validator.start().await;
        // self.passable_state.client = Some(client);
        for flow in self.flows.iter() {
            // (*flow)(self.passable_state.clone());
            println!("Starting flow");
            flow(self.passable_state.clone()).await;
            println!("Stopped flow");
        }
    }

    fn with_state<S: Send + 'static>(&mut self, state: S) -> &mut Self {
        println!("{:?}", TypeId::of::<S>());
        {
            let mut passable_state = self.passable_state.lock().unwrap();
            passable_state
                .state
                .insert(TypeId::of::<S>(), Box::new(Arc::new(Mutex::new(state))));
            println!("{:?}", passable_state.state.get(&TypeId::of::<S>()));
        }
        self
    }
}

trait Handler<T>: Clone + Send + Sized + 'static {
    type Future: Future<Output = ()> + Send + 'static;

    fn call(self, builder: Arc<Mutex<PassableState>>) -> Self::Future;
}

trait FromPassable {
    fn from_passable(builder: Arc<Mutex<PassableState>>) -> Self;
}
pub struct State<S: 'static>(pub Arc<Mutex<S>>);

impl<T: 'static> FromPassable for State<T> {
    fn from_passable(builder: Arc<Mutex<PassableState>>) -> State<T> {
        let builder = builder.lock().unwrap();
        let state = builder
            .state
            .get(&TypeId::of::<T>())
            .expect("State not found")
            .downcast_ref::<Arc<Mutex<T>>>()
            .expect("State type mismatch");
        State(state.clone())
    }
}

impl<F, A, Fut> Handler<A> for F
where
    F: FnOnce(A) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    A: FromPassable,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn call(self, fuzz_test_builder: Arc<Mutex<PassableState>>) -> Self::Future {
        let a = A::from_passable(fuzz_test_builder);
        (self)(a).boxed()
    }
}

// impl<F, A, B> Handler<(A, B)> for F
// where
//     F: Fn(A, B),
//     A: FromPassable,
//     B: FromPassable,
// {
//     fn call(&self, fuzz_test_builder: &PassableState) {
//         let a = A::from_passable(&fuzz_test_builder);
//         let b = B::from_passable(&fuzz_test_builder);
//         (*self)(a, b)
//     }
// }

// impl FromPassable for Client {
//     fn from_passable(builder: &PassableState) -> Self {
//         builder.client()
//     }
// }

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

    async fn flow_add_subject(State(state2): State<TestState2>) {
        // println!("{}", client.payer().pubkey());
        // println!("{}", state.borrow_mut().owner);
        // state2.borrow_mut().counter += 1
        let mut locked_state = state2.lock().unwrap();
        locked_state.counter += 1
    }

    async fn flow_add_smth(State(state2): State<TestState2>) {
        // println!("{}", client.payer().pubkey());
        // println!("{}", state.borrow_mut().owner);
        // state2.borrow_mut().counter += 1
        let locked_state = state2.lock().unwrap();
        println!("this shit: {}", locked_state.counter);
    }

    // fn invariant_add_subject(validator: &mut Validator, client: &mut Client) {}

    FuzzTestBuilder::new(validator)
        .add_flow(flow_add_subject)
        .add_flow(flow_add_smth)
        .with_state(TestState {
            owner: Pubkey::new_unique(),
        })
        .with_state(TestState2 { counter: 5 })
        .start()
        .await;
    // tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    // .invariant(invariant_add_subject);
}
