use futures::future::Future;

use crate::effects::Effects;

#[doc = include_str!("README.md")]
pub trait Reducer {
    /// …
    type Action;

    #[allow(unused_variables)]
    fn reduce(
        &mut self,
        action: Self::Action,
        effects: impl Effects<Action = Self::Action>,
    ) -> impl Future<Output = ()>;

    /// …
    type Output;
    /// …
    fn into_inner(self) -> Self::Output;
}
