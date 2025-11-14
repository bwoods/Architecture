#![doc = include_str!("README.md")]
use crate::store::Reactor;
use crate::{Effects, Reducer};
use std::cell::RefCell;
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

pub struct TestStore<State: Reducer>
where
    <State as Reducer>::Action: Debug,
{
    reactor: Option<Reactor<Subject<State>, <State as Reducer>::Action, Subject<State>>>,
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

        Self {
            reactor,
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
        let mut state = self.state.borrow_mut();
        f(&mut state);

        let (action_, state_) = self
            .audit
            .recv_timeout(Duration::from_secs(5)) // FIXME: self.timeout
            .unwrap_or_else(|_| panic!("no action received; {action:?} expected"));

        assert_eq!(action_, action);
        assert_eq!(state_, *state);
    }

    pub fn into_inner(mut self) -> State {
        self.reactor.as_ref().unwrap().stop();
        self.reactor.take().unwrap().handle.join().unwrap().state
    }

    pub fn advance(&self, duration: Duration) {
        assert!(duration > Duration::ZERO);

        #[cfg(test)]
        self.reactor
            .as_ref()
            .unwrap()
            .wake_ups
            .send(crate::store::Reason::Advance(duration));
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

    #[track_caller]
    fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>) {
        self.state.reduce(action.clone(), send);

        let state = self.state.clone();
        self.audit.send((action, state)).unwrap();
    }
}
