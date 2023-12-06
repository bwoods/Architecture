use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use futures::executor::block_on;

use crate::reducer::Reducer;

#[doc = include_str!("README.md")]
pub struct TestStore<State: Reducer>
where
    <State as Reducer>::Action: Debug,
{
    state: Option<State>, // `Option` so that `into_inner` does not break `Drop`
    effects: Rc<RefCell<VecDeque<<State as Reducer>::Action>>>,
}

impl<State: Reducer> TestStore<State>
where
    <State as Reducer>::Action: Debug,
{
    pub fn with_initial(state: State) -> Self {
        Self {
            state: Some(state),
            effects: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn new<F>(with: F) -> Self
    where
        F: (FnOnce() -> State),
    {
        Self::with_initial(with())
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

        block_on(
            self.state
                .as_mut()
                .unwrap()
                .reduce_async(action, self.effects.clone()),
        );
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

        block_on(
            self.state
                .as_mut()
                .unwrap()
                .reduce_async(action, self.effects.clone()),
        );
        assert_eq!(self.state, expected);
    }

    pub fn into_inner(mut self) -> <State as Reducer>::Output {
        self.state.take().unwrap().into_inner()
    }
}

impl<State: Reducer> Default for TestStore<State>
where
    State: Default,
    <State as Reducer>::Action: Debug,
{
    fn default() -> Self {
        Self::new(|| State::default())
    }
}

impl<State: Reducer> Drop for TestStore<State>
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
