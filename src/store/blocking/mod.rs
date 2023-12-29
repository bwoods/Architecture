use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use futures::executor::LocalPool;

use crate::reducer::Reducer;

pub struct Store<State: Reducer> {
    state: State,
    pub(crate) effects: Rc<RefCell<VecDeque<<State as Reducer>::Action>>>,
    pub(crate) pool: LocalPool,
}

impl<State: Reducer> Store<State> {
    pub fn with_initial(state: State) -> Self {
        let pool = LocalPool::new();
        let effects = Rc::new(RefCell::new(VecDeque::new()));

        Self {
            state,
            effects,
            pool,
        }
    }

    pub fn send(&mut self, action: impl Into<<State as Reducer>::Action>)
    where
        <State as Reducer>::Action: 'static,
    {
        self.pool.run_until(async {
            self.state
                .reduce(action.into(), Rc::downgrade(&self.effects));

            // wrapping the `borrow_mut` in a closure to ensure that the
            // borrow is dropped immediately
            let next = || self.effects.borrow_mut().pop_front();

            while let Some(action) = next() {
                self.state.reduce(action, Rc::downgrade(&self.effects));
            }
        });
    }
}

impl<State: Reducer> Default for Store<State>
where
    State: Default,
{
    fn default() -> Self {
        Self::with_initial(State::default())
    }
}
