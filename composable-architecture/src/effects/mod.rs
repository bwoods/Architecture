#![doc = include_str!("README.md")]

mod scheduler;
mod scoped;

pub use crate::effects::scheduler::{Scheduler, Task};
use futures::Stream;
use futures::StreamExt;
use futures::stream;
use scoped::Scoped;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::future::Future;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::time::Instant;

pub trait Effects: Clone + Scheduler<Item = Self::Action> + 'static {
    type Action;

    /// An effect that sends an [`Action`][`Self::Action`] through
    /// the `Store`’s [`Reducer`][`crate::Reducer`].
    #[doc(alias = "send")]
    fn action(&self, action: impl Into<<Self as Effects>::Action>);

    /// An effect that runs a [`Stream`](https://docs.rs/futures/latest/futures/stream/index.html)
    /// and sends every [`Action`][`Self::Action`] it returns through the `Store`’s
    /// [`Reducer`][`crate::Reducer`].
    #[doc(alias = "run")]
    fn stream<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S);

    /// An effect that runs a [`Future`][`std::future`] and, if it returns an
    /// [`Action`][`Self::Action`], sends it through the `Store`’s [`Reducer`][`crate::Reducer`].
    #[doc(alias = "run")]
    fn future<F: Future<Output = Option<<Self as Effects>::Action>> + 'static>(&self, future: F)
    where
        <Self as Effects>::Action: 'static;

    /// A [`Task`] represents asynchronous work that will then [`send`][`crate::Store::send`]
    /// zero or more [`Action`][`Self::Action`]s back into the `Store`’s [`Reducer`][`crate::Reducer`]
    /// as it runs.
    ///
    /// Use this method if you need to ability to [`cancel`][Task::cancel] the task
    /// while it is running. Otherwise [`future`][Effects::future] or [`stream`][Effects::stream]
    /// should be preferred.
    fn task<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) -> Task;

    /// Scopes the `Effects` down to one that sends child actions.
    #[inline(always)]
    fn scope<ChildAction>(&self) -> Scoped<Self, ChildAction>
    where
        Self::Action: From<ChildAction>,
    {
        Scoped(self.clone(), PhantomData)
    }
}

//
pub(crate) struct Inner<Action> {
    #[allow(clippy::type_complexity)]
    pub tasks: BTreeMap<u64, Pin<Box<dyn Stream<Item = ControlFlow<Instant, Action>>>>>,
    next_id: u64,

    pub timers: VecDeque<(Instant, u64)>,
    pub actions: VecDeque<Action>,
}

impl<Action> Inner<Action> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Inner {
            next_id: Default::default(),
            tasks: Default::default(),
            timers: Default::default(),
            actions: Default::default(),
        }))
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn is_empty(&self) -> bool {
        let is_empty = self.tasks.is_empty() && self.timers.is_empty();
        debug_assert!((is_empty && self.actions.is_empty()) || !is_empty); // actions is always emptied first

        is_empty
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        debug_assert_ne!(id, 0); // see reactor::SHUTDOWN
        id
    }
}

#[doc(hidden)]
impl<Action: 'static> Effects for Weak<RefCell<Inner<Action>>> {
    type Action = Action;

    fn action(&self, action: impl Into<Action>) {
        if let Some(effects) = self.upgrade() {
            effects.borrow_mut().actions.push_back(action.into())
        }
    }

    fn stream<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) {
        let stream = stream.map(|action| ControlFlow::Continue(action));
        self.schedule_stream(stream)
    }

    fn future<F: Future<Output = Option<<Self as Effects>::Action>> + 'static>(&self, future: F) {
        let stream = stream::once(future).filter_map(move |action| async { action });
        Effects::stream(self, stream)
    }

    fn task<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) -> Task {
        let stream = stream.map(|action| ControlFlow::Continue(action));
        self.schedule_task(stream, None)
    }
}
