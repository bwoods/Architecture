use std::thread::{JoinHandle, Thread};

use flume::Sender;

use crate::reducer::Reducer;

mod runtime;

pub(crate) mod testing;

pub struct Store<State: Reducer> {
    sender: Sender<Result<<State as Reducer>::Action, Thread>>,
    handle: JoinHandle<<State as Reducer>::Output>,
}

impl<State: Reducer> Store<State> {
    /// Creates a new `Store` with `state` as its initial state.
    ///
    /// If `state` can not be passed to the `Store`’s threads,
    /// use [`build`][`Store::build`] instead.
    pub fn new(state: State) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
        <State as Reducer>::Output: Send,
    {
        Store::runtime("Store".into(), || state)
    }

    /// Creates a new `Store` with its initial state as set by the value returned from a function.
    ///
    /// Useful if `State` itself can not pass between threads, but the arguments used to construct
    /// it can be.
    pub fn build<F>(named: String, with: F) -> Self
    where
        F: (FnOnce() -> State) + Send + 'static,
        <State as Reducer>::Action: Send + 'static,
        <State as Reducer>::Output: Send + 'static,
    {
        Store::runtime(named, with)
    }

    pub fn send(&self, action: impl Into<<State as Reducer>::Action>) {
        self.sender.send(Ok(action.into())).expect("Store::send")
    }

    /// Stops the [`Store`]’s runtime and returns its current `state` value.
    pub fn into_inner(self) -> <State as Reducer>::Output {
        self.sender
            .send(Err(std::thread::current()))
            .expect("Store::into_inner");
        std::thread::park(); // waiting for any async tasks to finish up

        drop(self.sender); // ends the runtime’s (outer) while-let
        self.handle.join().unwrap()
    }
}

impl<State: Reducer> Default for Store<State>
where
    State: Default,
    <State as Reducer>::Action: Send + 'static,
    <State as Reducer>::Output: Send + 'static,
{
    /// Creates a new `Store` with a default minitial state.
    fn default() -> Self {
        Store::build("Store".into(), || Default::default())
    }
}
