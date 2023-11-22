#![allow(dead_code)]

use futures::{stream, StreamExt};

use composable::{Effects, Reducer, Store};

struct State(usize);
#[derive(Clone, Debug)]
enum Action {
    A,
    B,
    C,
}

impl Reducer for State {
    type Action = Action;

    #[inline(never)]
    fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
        use Action::*;

        match action {
            A => self.0 += std::hint::black_box(1),
            B => {
                for _ in 0..N {
                    effects.send(std::hint::black_box(A))
                }
            }
            C => effects.stream(stream::repeat(std::hint::black_box(A)).take(N)),
        }
    }
}

const N: usize = 100000;

mod one_hundred_thousand {
    #[allow(unused_imports)]
    use super::*;

    #[divan::bench]
    fn external_sends() {
        let store = Store::new(State(0));
        for _ in 0..N {
            store.send(std::hint::black_box(Action::A));
        }

        let n = store.into_inner().0;
        assert_eq!(n, N);
    }

    #[divan::bench]
    fn internal_sends() {
        let store = Store::new(State(0));
        store.send(std::hint::black_box(Action::B));

        let n = store.into_inner().0;
        assert_eq!(n, N);
    }

    #[divan::bench]
    fn executor_sends() {
        let store = Store::new(State(0));
        store.send(std::hint::black_box(Action::C));

        let n = store.into_inner().0;
        assert_eq!(n, N);
    }
}

fn main() {
    divan::main();
}
