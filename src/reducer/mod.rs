use std::future::Future;

use crate::effects::Effects;

#[doc = include_str!("README.md")]
pub trait Reducer {
    /// …
    type Action;

    #[allow(unused_variables)]
    /// …
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>) {}

    #[inline(always)]
    fn reduce_async(
        &mut self,
        action: Self::Action,
        effects: impl Effects<Action = Self::Action>,
    ) -> impl Future<Output = ()> {
        async {
            self.reduce(action, effects);
        }
    }

    /// …
    type Output;
    /// …
    fn into_inner(self) -> Self::Output;
}
