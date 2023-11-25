…



## Example 

Here is the second [`Reducer`] example being tested with a [`TestStore`].

```rust
# use composable::*;
#
#[derive(Clone, Debug, Default, PartialEq)]
struct State {
    n: usize,
}

#[derive(Debug, PartialEq)]
enum Action {
    Increment,
    Decrement,
}

use Action::*;
impl Reducer for State {
    type Action = Action;
    type Output = usize;

    // This reducer ensures the value is always an even number
    async fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
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

    fn into_inner(self) -> Self::Output {
        self.n
    }
}

let mut store = TestStore::<State>::default();

store.send(Increment, |state| state.n = 1);
store.recv(Increment, |state| state.n = 2);

store.send(Increment, |state| state.n = 3);
store.recv(Increment, |state| state.n = 4);

store.send(Decrement, |state| state.n = 3);
store.recv(Decrement, |state| state.n = 2);

let n = store.into_inner();
assert_eq!(n, 2);
```
