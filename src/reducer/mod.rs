use crate::effects::Effects;

#[doc = include_str!("README.md")]
pub trait Reducer {
    /// …
    type Action;

    /// …
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>);

    /// …
    type Output;
    /// …
    fn into_inner(self) -> Self::Output;
}
