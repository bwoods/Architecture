use flume::unbounded;
use futures::executor::block_on;
use futures::StreamExt;

use crate::effects::Executor;
use crate::reducer::Reducer;
use crate::store::Store;

impl<State: Reducer> Store<State> {
    pub(crate) fn runtime(mut state: State) -> Self
    where
        State: Send + 'static,
        <State as Reducer>::Action: Send,
    {
        let (actions, external) = unbounded();
        let (effects, internal) = unbounded();

        let handle = std::thread::Builder::new()
            .name("Store".into())
            .spawn(move || {
                let executor: Executor<'_, <State as Reducer>::Action> = Executor::default();

                block_on(executor.run(async {
                    let mut stream = external.into_stream();
                    while let Some(action) = stream.next().await {
                        state.reduce(action, (executor.clone(), effects.clone()));

                        // this inner loop ensures all `internal` events are exhausted
                        // before returning to polling `external` events, above
                        for action in internal.try_iter() {
                            state.reduce(action, (executor.clone(), effects.clone()));
                        }
                    }
                }));

                state
            })
            .unwrap();

        Store { actions, handle }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, Mutex};

    use ntest_timeout::timeout;

    use crate::effects::Effects;

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

        fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>) {
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
        let store = Store::new(State {
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
    #[timeout(1000)]
    /// ## Note
    /// If this test **timeout**s, the [`join`][std::thread::JoinHandle::join] in [`Store::into_inner`] is hanging
    fn test_into_inner_returns() {
        struct State;
        enum Action {}

        impl Reducer for State {
            type Action = Action;

            fn reduce(&mut self, _action: Self::Action, _effects: impl Effects<Action = Action>) {}
        }

        let store = Store::new(State);
        store.into_inner();
    }
}
