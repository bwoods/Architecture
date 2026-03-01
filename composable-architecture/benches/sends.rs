use composable::Effects;
use composable::Reducer;
use divan::{bench as benchmark, main as run_benchmarks};
use futures::{StreamExt, future, stream};

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

    #[inline(never)]
    fn reduce(&mut self, action: Action, send: impl Effects<Action = Self::Action>) {
        use Action::*;

        match action {
            A => self.0 += std::hint::black_box(1),
            B => {
                for _ in 0..std::hint::black_box(N) {
                    send.action(std::hint::black_box(A))
                }
            }
            C => send.stream(stream::repeat(std::hint::black_box(A)).take(std::hint::black_box(N))),
            D => {
                for _ in 0..std::hint::black_box(N) {
                    send.future(future::ready(Some(std::hint::black_box(A))))
                }
            }
        }
    }
}

impl From<State> for usize {
    fn from(value: State) -> Self {
        value.0
    }
}

const N: usize = 100000;

mod one_hundred_thousand {
    #[allow(unused_imports)]
    use super::*;
    use composable::Store;

    #[benchmark(min_time = 1)]
    fn external_sends() {
        let store = Store::with_initial(State(0));
        for _ in 0..N {
            store.send(std::hint::black_box(Action::A));
        }

        let n: usize = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark(min_time = 1)]
    fn internal_sends() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::B));

        let n: usize = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark(min_time = 1)]
    fn task_sends_batched() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::C));

        let n: usize = store.into_inner();
        assert_eq!(n, N);
    }

    #[benchmark(min_time = 1)]
    fn task_sends() {
        let store = Store::with_initial(State(0));
        store.send(std::hint::black_box(Action::D));

        let n: usize = store.into_inner();
        assert_eq!(n, N);
    }
}
