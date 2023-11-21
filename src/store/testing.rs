use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use crate::reducer::Reducer;

pub struct Store<State: Reducer>
where
    <State as Reducer>::Action: Debug,
{
    state: Option<State>, // `Option` so that `into_inner` does not break `Drop`
    effects: Rc<RefCell<VecDeque<<State as Reducer>::Action>>>,
    // TODO: actions: Sender<Action>
}

impl<State: Reducer> Store<State>
where
    <State as Reducer>::Action: Debug,
{
    pub fn new(initial: State) -> Self {
        let effects = Rc::new(RefCell::new(VecDeque::new()));

        Self {
            state: Some(initial),
            effects,
        }
    }

    #[track_caller]
    pub fn send(&mut self, action: <State as Reducer>::Action, assert: impl FnOnce(&mut State))
    where
        State: Clone + Debug + PartialEq,
        <State as Reducer>::Action: 'static,
    {
        let mut expected = self.state.clone();
        assert(expected.as_mut().unwrap());

        assert!(
            self.effects.borrow().is_empty(),
            "an extra action was received: {:#?}",
            self.effects.borrow_mut().drain(..).collect::<Vec<_>>()
        );

        self.state
            .as_mut()
            .unwrap()
            .reduce(action, self.effects.clone());
        assert_eq!(self.state, expected);
    }

    #[track_caller]
    pub fn recv(&mut self, action: <State as Reducer>::Action, assert: impl FnOnce(&mut State))
    where
        State: Clone + Debug + PartialEq,
        <State as Reducer>::Action: Debug + PartialEq + 'static,
    {
        let mut expected = self.state.clone();
        assert(expected.as_mut().unwrap());

        let received = self
            .effects
            .borrow_mut()
            .pop_front()
            .expect("no action received");
        assert_eq!(received, action);

        self.state
            .as_mut()
            .unwrap()
            .reduce(action, self.effects.clone());
        assert_eq!(self.state, expected);
    }

    pub fn into_inner(mut self) -> State {
        self.state.take().unwrap()
    }
}

impl<State: Reducer> Drop for Store<State>
where
    <State as Reducer>::Action: Debug,
{
    #[track_caller]
    fn drop(&mut self) {
        assert!(
            self.effects.borrow().is_empty(),
            "one or more extra actions were not tested for: {:#?}",
            self.effects.borrow_mut().drain(..).collect::<Vec<_>>()
        );
    }
}

#[test]
fn test_test_store() {
    use crate::effects::Effects;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct State {
        n: usize,
    }

    #[derive(Debug, PartialEq)]
    enum Action {
        Increment,
        Decrement,
    }

    use Action::*;
    impl Reducer for State {
        type Action = Action;

        // This reducer ensures the value is always an even number
        fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
            match action {
                Increment => {
                    self.n += 1;
                    if self.n % 2 == 1 {
                        effects.send(Increment);
                    }
                }
                Decrement => {
                    self.n -= 1;
                    if self.n % 2 == 1 {
                        effects.send(Decrement);
                    }
                }
            }
        }
    }

    let mut store = Store::new(State::default());

    store.send(Increment, |state| state.n = 1);
    store.recv(Increment, |state| state.n = 2);

    store.send(Increment, |state| state.n = 3);
    store.recv(Increment, |state| state.n = 4);

    store.send(Decrement, |state| state.n = 3);
    store.recv(Decrement, |state| state.n = 2);

    let result = store.into_inner();
    assert_eq!(result.n, 2);
}
