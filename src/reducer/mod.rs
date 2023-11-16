use crate::effects::Effects;

/// A trait that describes how to evolve the current state of an application to the next state,
/// given an action, and describes what Effect’s should be executed later by the store, if any.
///
/// Conform types to this trait to represent the domain, logic and behavior for your feature.
#[doc = include_str!("README.md")]
pub trait Reducer {
    /// …
    type Action;

    /// …
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>);
}
