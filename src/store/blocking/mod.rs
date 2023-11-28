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
                .reduce_async(action.into(), self.effects.clone())
                .await;

            // wrapping the `borrow_mut` in a closure to ensure the borrow
            // is dropped before the `await` that follows
            let next = || self.effects.borrow_mut().pop_front();

            // see:
            //  https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref
            while let Some(action) = next() {
                self.state.reduce_async(action, self.effects.clone()).await;
            }
        });
    }

    pub fn into_inner(self) -> <State as Reducer>::Output {
        self.state.into_inner()
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
