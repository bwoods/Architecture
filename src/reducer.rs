use crate::effects::Effects;

pub trait Reducer {
    /// …
    type Action;

    /// …
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>);
}
