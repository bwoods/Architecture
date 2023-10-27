use std::fmt::Debug;

use flume::{unbounded, Receiver, Sender};

use crate::reducer::Reducer;

pub struct Store<State: Reducer> {
    state: State,
    effects: Sender<<State as Reducer>::Action>,
    actions: Receiver<<State as Reducer>::Action>,
}

impl<State: Reducer> Store<State> {
    pub fn new(initial: State) -> Self {
        let (effects, actions) = unbounded();

        Self {
            state: initial,
            effects,
            actions,
        }
    }

    #[track_caller]
    pub fn send(
        mut self,
        action: <State as Reducer>::Action,
        assert: impl FnOnce(&mut State),
    ) -> Self
    where
        State: Clone + Debug + PartialEq,
    {
        let mut expected = self.state.clone();
        assert(&mut expected);

        assert!(self.actions.is_empty(), "an extra action was received");

        self.state.reduce(action, self.effects.clone());
        assert_eq!(self.state, expected);
        self
    }

    #[track_caller]
    pub fn receive(
        mut self,
        action: <State as Reducer>::Action,
        assert: impl FnOnce(&mut State),
    ) -> Self
    where
        State: Clone + Debug + PartialEq,
        <State as Reducer>::Action: Debug + PartialEq,
    {
        let mut expected = self.state.clone();
        assert(&mut expected);

        let received = self.actions.recv().expect("no action received");
        assert_eq!(received, action);

        self.state.reduce(action, self.effects.clone());
        assert_eq!(self.state, expected);
        self
    }
}

impl<State: Reducer> Drop for Store<State> {
    #[track_caller]
    fn drop(&mut self) {
        assert!(
            self.actions.is_empty(),
            "one or more extra actions were not tested for"
        );
    }
}
