One of the benefits of a Composable Architecture is how much easier it makes the testing of complex domains.

## Example

Here is the second [`Reducer`] example being tested with a [`TestStore`].

```rust
# use composable::*;
# use composable::testing::*;
#
#[derive(Clone, Debug, Default, PartialEq)]
struct State {
    n: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Action {
    Increment,
    Decrement,
}

use Action::*;
impl Reducer for State {
    type Action = Action;

    // This reducer ensures the value is always an even number
    fn reduce(&mut self, action: Action, send: impl Effects<Action=Self::Action>) {
        match action {
            Increment => {
                self.n += 1;
                if self.n % 2 == 1 {
                    send.action(Increment);
                }
            }
            Decrement => {
                self.n -= 1;
                if self.n % 2 == 1 {
                    send.action(Decrement);
                }
            }
        }
    }
}

let store = TestStore::with_initial(State::default());

store.send(Increment, |state| state.n = 1);
store.recv(Increment, |state| state.n = 2);

store.send(Increment, |state| state.n = 3);
store.recv(Increment, |state| state.n = 4);

store.send(Decrement, |state| state.n = 3);
store.recv(Decrement, |state| state.n = 2);
```

## Scheduling

```rust
# use composable::*;
# use composable::testing::*;
# use std::time::Duration;
#
#[derive(Debug, Default)]
struct State {
    previous: Option<Task>,
    n: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.n.eq(&other.n)
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            previous: None,
            n: self.n,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
enum Action {
    Send,
    Recv,
}

use Action::*;

impl Reducer for State {
    type Action = Action;

    fn reduce(&mut self, action: Action, send: impl Effects<Action = Self::Action>) {
        match action {
            Send => {
                send.debounce(
                    Recv,
                    &mut self.previous,
                    Interval::Trailing(Duration::from_secs(4)),
                );
            }
            Recv => {
                self.n += 1;
            }
        }
    }
}

let store = TestStore::with_initial(State::default());
let no_change: fn(&mut State) = |_| {};

store.send(Send, no_change);
store.advance(Duration::from_secs(3));

store.send(Send, no_change);
store.advance(Duration::from_secs(8));
store.recv(Recv, |state| state.n = 1);

store.send(Send, no_change);
store.advance(Duration::from_secs(1));
store.advance(Duration::from_secs(1));
store.advance(Duration::from_secs(1));
store.advance(Duration::from_secs(1));
store.recv(Recv, |state| state.n = 2);
```

