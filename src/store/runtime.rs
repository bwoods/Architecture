use std::thread::JoinHandle;

use flume::{unbounded, Selector, Sender};

use crate::effects::Effects;
use crate::reducer::Reducer;

pub struct Runtime<State: Reducer> {
    actions: Sender<<State as Reducer>::Action>,
    handle: JoinHandle<State>,
}

impl<State: Reducer> Runtime<State> {
    pub fn new(mut state: State) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
    {
        let (actions, external) = unbounded();
        let (effects, internal) = unbounded();

        let handle = std::thread::Builder::new()
            .name("Store".into())
            .spawn(move || {
                // `Selector` polls in order, so all `internal` events are exhausted
                //  before polling for new `external` events
                while let Some(action) = Selector::new()
                    .recv(&internal, |action| action.ok())
                    .recv(&external, |action| action.ok())
                    .wait()
                {
                    let effects = effects.clone();
                    state.reduce(action, effects);
                }

                state
            })
            .unwrap();

        Runtime { actions, handle }
    }

    /// Stops the `Store`’s runtime and returns the current `State` value.
    pub fn into_inner(self) -> State {
        drop(self.actions); // ends the `Selector` while-let
        self.handle.join().unwrap()
    }
}

impl<State: Reducer> Effects<<State as Reducer>::Action> for Runtime<State> {
    fn send(&self, action: impl Into<<State as Reducer>::Action>) {
        self.actions.send(action.into()).expect("Store::send")
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct State {
        pub characters: Arc<Mutex<Vec<char>>>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Action {
        External(char),
        Internal(char),
    }

    impl Reducer for State {
        type Action = Action;

        fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
            use Action::*;

            match action {
                Internal(ch) => self.characters.lock().unwrap().push(ch),
                External(ch) => {
                    self.characters.lock().unwrap().push(ch);

                    if ch == '1' {
                        effects.send(Internal('A'));
                        effects.send(Internal('B'));
                        effects.send(Internal('C'));
                        effects.send(Internal('D'));
                    }
                }
            }
        }
    }

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            let lhs = self.characters.lock().unwrap();
            let rhs = other.characters.lock().unwrap();

            *lhs == *rhs
        }
    }

    #[test]
    /// Certain domains rely upon a chain of internal effects being uninterruptible by any
    /// additional external actions. This test helps ensure that guarantee.
    ///
    /// ## Note:
    ///
    /// - Enabling flume’s [eventual-fairness] feature **will break** the `Selector`
    ///   behavior that we rely upon.
    /// - Normal tests should use [`clock`]s and a [`TestStore`] rather than the brute-force
    ///   loop and thread manipulations used here.
    ///
    /// [eventual-fairness]: https://docs.rs/flume/latest/flume/select/struct.Selector.html#method.wait
    ///
    fn test_action_ordering_guarantees() {
        let characters = Arc::new(Mutex::new(Default::default()));
        let store = Runtime::new(State {
            characters: characters.clone(),
        });

        use Action::*;
        store.send(External('1'));
        store.send(External('2'));
        store.send(External('3'));

        loop {
            {
                let values = characters.lock().unwrap();
                if values.len() == 7 {
                    break;
                }
            }

            std::thread::yield_now();
        }

        let values = characters.lock().unwrap();
        // '1'’s side-effects happen BEFORE the other actions are dispatched
        assert_eq!(*values, vec!['1', 'A', 'B', 'C', 'D', '2', '3']);
    }

    #[test]
    fn test_into_inner() {
        struct State;
        enum Action {}

        impl Reducer for State {
            type Action = Action;

            fn reduce(&mut self, _action: Self::Action, _effects: impl Effects<Self::Action>) {}
        }

        let store = Runtime::new(State);
        let _result = store.into_inner(); // ensuring that `into_inner` terminates
    }
}
