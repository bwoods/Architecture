use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::thread::Thread;

use flume::{unbounded, WeakSender};
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

use crate::dependency::with_dependency;
use crate::effects::Executor as EffectsExecutor;
use crate::reducer::Reducer;
use crate::store::Store;

impl<State: Reducer> Store<State> {
    pub(crate) fn runtime<F>(named: String, with: F) -> Self
    where
        F: (FnOnce() -> State) + Send + 'static,
        <State as Reducer>::Action: Send + 'static,
        <State as Reducer>::Output: Send + 'static,
    {
        let (sender, receiver) = unbounded();
        let actions: WeakSender<Result<<State as Reducer>::Action, Thread>> = sender.downgrade();

        let handle = std::thread::Builder::new()
            .name(named)
            .spawn(move || {
                let mut executor = LocalPool::new();
                let spawner = executor.spawner();

                let mut state = with();
                let runtime = EffectsExecutor::new(spawner.clone(), actions);
                let effects = Rc::new(RefCell::new(VecDeque::new()));

                with_dependency(runtime, || {
                    executor.run_until(async {
                        while let Ok(result) = receiver.recv_async().await {
                            match result {
                                Ok(action) => {
                                    state.reduce(action, effects.clone());

                                    while let Some(action) = effects.borrow_mut().pop_front() {
                                        state.reduce(action, effects.clone());
                                    }
                                }
                                Err(parked) => {
                                    spawner
                                        // `unpark` a thread that is waiting for the store to shutdown;
                                        //  we use a future so that it happens after other (waiting) futures
                                        //
                                        //  See: `Store::into_inner` for the other size of this
                                        .spawn_local(async move {
                                            parked.unpark();
                                        })
                                        .expect("unpark");
                                }
                            }
                        }
                    });

                    state.into_inner()
                })
            })
            .unwrap();

        Store { sender, handle }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, Mutex};

    #[cfg(not(miri))]
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

        fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
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

        type Output = Self;

        fn into_inner(self) -> Self::Output {
            self
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
    /// - Normal tests should use [`clock`]s and a [`TestStore`] rather than the brute-force
    ///   loop and thread manipulations used here.
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
    #[cfg(not(miri))]
    #[timeout(1000)]
    /// ## Note
    /// If this test **timeout**s, the [`join`][std::thread::JoinHandle::join] in [`Store::into_inner`] is hanging
    fn test_into_inner_returns() {
        struct State;

        #[derive(Debug)]
        enum Action {}

        impl Reducer for State {
            type Action = Action;

            fn reduce(&mut self, _action: Action, _effects: impl Effects<Action = Action>) {}

            type Output = Self;

            fn into_inner(self) -> Self::Output {
                self
            }
        }

        let store = Store::new(State);
        store.into_inner();
    }
}
