use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Rc;

use flume::{Sender, WeakSender};
use futures::executor::LocalSpawner;
use futures::future::RemoteHandle;
use futures::task::LocalSpawnExt;
use futures::{future, Stream, StreamExt};

pub trait Effects: Clone {
    type Action;

    fn send(&self, action: Self::Action);

    #[inline(always)]
    fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    where
        <Self as Effects>::Action: From<ChildAction>,
    {
        (self.clone(), Marker)
    }

    fn task<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) -> Task;
}

#[doc(hidden)]
// Nested tuples are used by the [`scope`] function
impl<Action, Parent> Effects for (Parent, Marker<Action>)
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Action>,
{
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.0.send(action.into());
    }

    #[inline(always)]
    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        self.0.task(stream.map(|action| action.into()))
    }
}

#[doc(hidden)]
// `Parent` tuple for `Effect::scope` tuples
impl<Action: 'static> Effects
    for Rc<(RefCell<VecDeque<Action>>, LocalSpawner, WeakSender<Action>)>
{
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.0.borrow_mut().push_back(action);
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        match self.2.upgrade() {
            None => Task(None),
            Some(sender) => {
                let stream = stream.then(move |action| sender.clone().into_send_async(action));
                let future = stream.for_each(|_| future::ready(())); // discard `send`s return value

                // may return a `Task(None)` while the `Store` is shutting down
                Task(self.1.spawn_local_with_handle(future).ok())
            }
        }
    }
}

#[doc(hidden)]
// `Parent` for `TestStore` effects
impl<Action> Effects for Sender<Action> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        let _ = self.send(action);
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, _stream: S) -> Task {
        todo!()
    }
}

/// `Task` based `Effects` are run on an local async executor
#[must_use = "Dropping a Task cancels it"]
pub struct Task(Option<RemoteHandle<()>>);

impl Task {
    pub fn detach(self) {
        if let Some(handle) = self.0 {
            handle.forget()
        }
    }

    pub fn cancel(self) {
        // dropping it cancels it
    }
}
