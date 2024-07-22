use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Instant;

use futures::Stream;

use crate::dependencies::Dependency;
use crate::effects::scheduler::Reactor;

pub(crate) enum State {
    New(Instant),
    Waiting(Waker),
    Ready,
    Done,
}

pub struct Delay(Arc<Mutex<State>>);

impl Future for Delay {
    type Output = ();

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_next(cx).map(|_| ()) // Ready(Some()) → Ready(())
    }
}

impl Stream for Delay {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self
            .0
            .lock() //
            .unwrap_or_else(|err| err.into_inner());

        match &mut *state {
            State::New(instant) => {
                let instant = *instant;
                *state = State::Waiting(cx.waker().clone());
                drop(state);

                // Now that it has a Waker…
                let scheduler = Dependency::<Reactor>::new();
                scheduler.add(instant, self.0.clone());

                Poll::Pending
            }
            State::Waiting(waker) => {
                waker.clone_from(cx.waker()); // update the waker if needed
                Poll::Pending
            }
            State::Ready => {
                *state = State::Done;
                Poll::Ready(Some(()))
            }
            State::Done => Poll::Ready(None),
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

impl Delay {
    pub fn new(instant: Instant) -> Self {
        Delay(Arc::new(Mutex::new(State::New(instant))))
    }
}
