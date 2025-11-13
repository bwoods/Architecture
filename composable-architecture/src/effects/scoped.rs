use crate::effects::Effects;
use crate::effects::scheduler::{Scheduler, Task};
use futures::Future;
use futures::FutureExt;
use futures::Stream;
use futures::StreamExt;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::time::Instant;

/// An `Effects` that scopes its `Action`s to one that sends child actions.
///
/// This `struct` is created by the [`scope`] method on [`Effects`]. See its
/// documentation for more.
///
/// [`scope`]: Effects::scope
pub struct Scoped<Parent, Child>(pub(crate) Parent, pub(crate) PhantomData<Child>);

impl<Parent, Child> Clone for Scoped<Parent, Child>
where
    Parent: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Scoped(self.0.clone(), PhantomData)
    }
}

impl<Parent, Child> Effects for Scoped<Parent, Child>
where
    <Parent as Effects>::Action: From<Child> + 'static,
    Parent: Effects,
    Child: 'static,
{
    type Action = Child;

    #[inline(always)]
    fn action(&self, action: impl Into<<Self as Effects>::Action>) {
        self.0.action(action.into());
    }

    #[inline(always)]
    fn stream<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) {
        self.0.stream(stream.map(|action| action.into()))
    }

    #[inline(always)]
    fn future<F: Future<Output = Option<<Self as Effects>::Action>> + 'static>(&self, future: F)
    where
        <Self as Effects>::Action: 'static,
    {
        self.0
            .future(future.map(|action| action.map(|action| action.into())))
    }

    #[inline(always)]
    fn task<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) -> Task {
        self.0.task(stream.map(|action| action.into()))
    }
}

#[doc(hidden)]
impl<Parent, Child> Scheduler for Scoped<Parent, Child>
where
    <Parent as Effects>::Action: 'static + From<Child>,
    Parent: Effects,
    Child: 'static,
{
    type Item = Child;

    fn schedule_stream<
        S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static,
    >(
        &self,
        stream: S,
    ) {
        self.0.schedule_stream(stream.map(|action| match action {
            ControlFlow::Continue(action) => ControlFlow::Continue(action.into()),
            ControlFlow::Break(instant) => ControlFlow::Break(instant),
        }))
    }

    fn schedule_task<
        S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static,
    >(
        &self,
        stream: S,
        when: Option<Instant>,
    ) -> Task {
        self.0.schedule_task(
            stream.map(|action| match action {
                ControlFlow::Continue(action) => ControlFlow::Continue(action.into()),
                ControlFlow::Break(instant) => ControlFlow::Break(instant),
            }),
            when,
        )
    }
}
