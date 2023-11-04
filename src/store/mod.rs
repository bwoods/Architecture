use std::fmt::Debug;

use stack_dst::{buffers, Value};

#[cfg(test)]
mod testing;

mod runtime;

trait Store {
    type Action;

    fn send(&self, action: Self::Action);
}

/// An owned dynamically typed [`Store`] for use in cases where you can't
/// statically type your result or need to add some indirection.
struct Boxed<Action> {
    inline: Value<dyn Store<Action = Action>, buffers::U64_2>, // <arc>, <vtbl>
}

impl<Action> Boxed<Action> {
    fn new<S: Store<Action = Action> + Debug + 'static>(store: S) -> Boxed<Action> {
        Self {
            inline: Value::new_stable(store, |val| val as _).unwrap(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::Store;

    struct State;

    enum Action {}

    impl Store for State {
        type Action = Action;

        fn send(&self, _action: Self::Action) {}
    }

    #[test]
    fn test_object_safety() {
        let _object: Box<dyn Store<Action = Action>> = Box::new(State);

        // …
    }
}
