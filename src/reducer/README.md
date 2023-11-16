This trait to represents the domain, logic and behavior for a feature. The domain is specified by a `State` and the `Actions` which act upon it

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

    fn reduce(&mut self, action: Action, _effects: impl Effects<Action = Action>) {
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

The `reduce` method’s first responsibility is to mutate the feature’s current state given an action. Its second responsibility is to return effects that will be executed asynchronously and feed their data back into the system. Currently `Feature` does not need to run any effects, and so [`none`](https://pointfreeco.github.io/swift-composable-architecture/main/documentation/composablearchitecture/effect/none) is returned.

If the feature does need to do effectful work, then more would need to be done. For example, suppose the feature has the ability to start and stop a timer, and with each tick of the timer the `count` will be incremented. That could be done like so:



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

    // This reducer ensures the value is always an even number
    fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
        match action {
            Increment => {
                self.n += 1;
                if self.n % 2 == 1 {
                    effects.send(Increment);
                }
            }
            Decrement => {
                self.n -= 1;
                if self.n % 2 == 1 {
                    effects.send(Decrement);
                }
            }
        }
    }
}
```

