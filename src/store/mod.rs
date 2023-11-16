use std::thread::JoinHandle;

use flume::Sender;

use crate::effects::Effects;
use crate::reducer::Reducer;

pub(crate) mod testing;

mod runtime;

pub struct Store<State: Reducer> {
    actions: Sender<<State as Reducer>::Action>,
    handle: JoinHandle<State>,
}

impl<State: Reducer> Store<State> {
    pub fn new(state: State) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
    {
        Store::runtime(state)
    }

    #[inline(always)]
    pub fn send(&self, action: impl Into<<State as Reducer>::Action>) {
        self.actions.send(action.into()).expect("Store::send")
    }

    #[inline(always)]
    pub fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    where
        <State as Reducer>::Action: From<ChildAction>,
    {
        (self.actions.clone(), Default::default())
    }

    /// Stops the [`Store`]’s runtime and returns the current `State` value.
    pub fn into_inner(self) -> State {
        drop(self.actions); // ends the runtime’s while-let
        self.handle.join().unwrap()
    }
}
