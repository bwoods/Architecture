use std::thread::{JoinHandle, Thread};

use flume::Sender;

use crate::reducer::Reducer;

mod runtime;
pub(crate) mod testing;

pub struct Store<State: Reducer> {
    sender: Sender<Result<<State as Reducer>::Action, Thread>>,
    handle: JoinHandle<State>,
}

impl<State: Reducer> Store<State> {
    pub fn new(state: State) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
    {
        Store::with_name(state, "Store".into())
    }

    pub fn with_name(state: State, name: String) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
    {
        Store::runtime(state, name)
    }

    pub fn send(&self, action: impl Into<<State as Reducer>::Action>) {
        self.sender.send(Ok(action.into())).expect("Store::send")
    }

    /// Stops the [`Store`]’s runtime and returns the current `State` value.
    pub fn into_inner(self) -> State {
        self.sender
            .send(Err(std::thread::current()))
            .expect("Store::into_inner");
        std::thread::park(); // waiting for any async tasks to finish up

        drop(self.sender); // ends the runtime’s (outer) while-let
        self.handle.join().unwrap()
    }
}
