`Reducer`s  are responsible for updating a `Store`’s  `state` in response to its `Action`s.

```rust
#[derive(Clone, Debug, Default, PartialEq)]
struct State {
    n: usize,
}

#[derive(Debug, PartialEq)]
enum Action {
    Increment,
    Decrement,
}
```

The logic of the feature is performed by mutating its current state with actions. This is most easily done by implementing the [`Reducer`] trait directly on it’s `State`.



## Example

```rust
# #[derive(Clone, Debug, Default, PartialEq)]
# struct State {
#     n: usize,
# }
# 
# #[derive(Debug, PartialEq)]
# enum Action {
#     Increment,
#     Decrement,
# }
# 
# use composable::*;
#
use Action::*;
impl Reducer for State {
    type Action = Action;
    type Output = usize;
  
    fn into_inner(self) -> Self::Output {
        self.n
    }

    fn reduce(&mut self, action: Action, _effects: impl Effects<Action>) {
        match action {
            Increment => {
                self.n += 1;
            }
            Decrement => {
                self.n -= 1;
            }
        }
    }
}
```

The `reduce` method’s first responsibility is to mutate the feature’s current state given an action. Its second responsibility is to trigger effects that feed their actions back into the system. Currently `reduce` does not need to run any effects so `_effects` goes unused.

If the action does need side effects, then more would need to be done. For example, if `reduce` always maintained an even number for the `State`, then each `Increment` and `Decrement` would need an effect to follow:[^actually…]

[^actually…]: <small>Granted, real code could just adjust the values by two. It *is* a contrived example to show how to use `effects`, after all.</small>

```rust
# #[derive(Clone, Debug, Default, PartialEq)]
# struct State {
#     n: usize,
# }
# 
# #[derive(Debug, PartialEq)]
# enum Action {
#     Increment,
#     Decrement,
# }
# 
# use composable::*;
#
use Action::*;
impl Reducer for State {
    type Action = Action;
    type Output = usize;
 
    fn into_inner(self) -> Self::Output {
        self.n
    }

    // This reducer ensures the value is always an even number
    fn reduce(&mut self, action: Action, effects: impl Effects<Action>) {
        match action {
            Increment => {
                self.n += 1;
                if self.n % 2 == 1 {
                    effects.send(Increment); // ⬅︎
                }
            }
            Decrement => {
                self.n -= 1;
                if self.n % 2 == 1 {
                    effects.send(Decrement); // ⬅︎
                }
            }
        }
    }
}
```

- See [`TestStore`][`crate::TestStore`] for a more complete test of this example.
- See [`Effects`] for all of the effects that can be used within a `Reducer`.

