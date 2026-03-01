#![doc = include_str!("README.md")]

use crate::store::Reactor;
use crate::{Effects, Reducer};
use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

pub struct TestStore<State: Reducer>
where
    <State as Reducer>::Action: Debug,
{
    reactor: Option<Reactor<Subject<State>, <State as Reducer>::Action, Subject<State>>>,
    now: Cell<Instant>,

    audit: Receiver<(<State as Reducer>::Action, State)>,
    state: RefCell<State>,
}

impl<State> TestStore<State>
where
    State: Clone + Debug + Reducer + PartialEq + Send + 'static,
    <State as Reducer>::Action: Clone + Debug + PartialEq + Send + 'static,
{
    pub fn with_initial(state: State) -> Self {
        let (sender, receiver) = channel();
        let subject = Subject {
            state: state.clone(),
            audit: sender,
        };

        let state = RefCell::new(state);
        let reactor = Some(Reactor::new(move || subject));
        let snapshot = Cell::new(Instant::now());

        Self {
            reactor,
            now: snapshot,
            audit: receiver,
            state,
        }
    }

    #[track_caller]
    pub fn send(&self, action: <State as Reducer>::Action, f: impl FnOnce(&mut State)) {
        let actions = self
            .audit
            .try_iter()
            .map(|(action, _)| action)
            .collect::<Vec<_>>();

        assert!(
            actions.is_empty(),
            "extra actions were received: {actions:#?}"
        );

        f(&mut self.state.borrow_mut());

        self.reactor.as_ref().unwrap().events.send(action.clone());
        self.recv(action, |_| {}) // ensure we received what we sent
    }

    #[track_caller]
    pub fn recv(&self, action: <State as Reducer>::Action, f: impl FnOnce(&mut State)) {
        const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
        self.recv_timeout(action, DEFAULT_TIMEOUT, f);
    }

    #[track_caller]
    pub fn recv_timeout(
        &self,
        action: <State as Reducer>::Action,
        timeout: Duration,
        f: impl FnOnce(&mut State),
    ) {
        let mut state = self.state.borrow_mut();
        f(&mut state);

        let (action_, state_) = self
            .audit
            .recv_timeout(timeout)
            .expect("no action received");

        assert_eq!(action_, action);
        assert_eq!(state_, *state);
    }

    pub fn into_inner(mut self) -> State {
        self.reactor.as_ref().unwrap().stop();
        self.reactor.take().unwrap().handle.join().unwrap().state
    }

    pub fn advance(&self, duration: Duration) {
        self.now.update(|instant| instant + duration); // panics on overflow; failing the test

        let barrier = Arc::new(Barrier::new(2));
        self.reactor
            .as_ref()
            .unwrap()
            .other
            .send(crate::store::Reason::Sync(self.now.get(), barrier.clone()));

        barrier.wait();
    }
}

impl<State: Reducer> Drop for TestStore<State>
where
    <State as Reducer>::Action: Debug,
{
    fn drop(&mut self) {
        let actions = self
            .audit
            .try_iter()
            .map(|(action, _)| action)
            .collect::<Vec<_>>();

        assert!(
            actions.is_empty(),
            "extra actions were not tested for: {actions:#?}"
        );
    }
}

struct Subject<State: Reducer> {
    audit: Sender<(<State as Reducer>::Action, State)>,
    state: State,
}

impl<State: Reducer> Reducer for Subject<State>
where
    State: Clone + Debug,
    <State as Reducer>::Action: Clone + Debug,
{
    type Action = <State as Reducer>::Action;

    fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>) {
        self.state.reduce(action.clone(), send);

        let state = self.state.clone();
        self.audit.send((action, state)).unwrap();
    }
}
