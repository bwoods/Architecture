use divan::{bench as benchmark, main as run_benchmarks};
use futures::{future, stream, StreamExt};

use composable::{Effects, Reducer, Store};

fn main() {
    run_benchmarks();
}

struct State(usize);

#[derive(Clone, Debug)]
enum Action {
    A,
    B,
    C,
    D,
}

impl Reducer for State {
    type Action = Action;
    type Output = usize;

    fn into_inner(self) -> Self::Output {
        self.0
    }

    #[inline(never)]
    fn reduce(&mut self, action: Action, effects: impl Effects<Action>) {
        use Action::*;

        match action {
            A => self.0 += std::hint::black_box(1),
            B => {
                for _ in 0..std::hint::black_box(N) {
                    effects.send(std::hint::black_box(A))
                }
            }
            C => effects
                .stream(stream::repeat(std::hint::black_box(A)).take(std::hint::black_box(N))),
            D => {
                for _ in 0..std::hint::black_box(N) {
                    effects.future(future::ready(Some(std::hint::black_box(A))))
                }
            }
        }
    }
}

const N: usize = 100000;

mod one_hundred_thousand {
    #[allow(unused_imports)]
    use super::*;

    #[benchmark]
    fn external_sends() {
        let store = Store::with_initial(State(0));
        for _ in 0..N {
            store.send(std::hint::black_box(Action::A));
        }

        let n = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark]
    fn internal_sends() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::B));

        let n = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark]
    fn task_sends_batched() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::C));

        let n = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark]
    fn task_sends() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::D));

        let n = store.into_inner();
        assert_eq!(n, N);
    }
}
