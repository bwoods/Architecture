use crate::inter::Inter;
use crate::nord::Nord;
use chrono::{DateTime, Datelike, DurationRound, Local, NaiveTime, TimeDelta, Weekday};
use composable::*;
use composable_views::text::Font;
use composable_views::*;
use itertools::Itertools;
use log::trace;
use std::sync::LazyLock;
use std::time::Duration;

#[derive(Clone, From, TryInto, Reducers)]
pub enum Action {
    Tick,
    Pause(bool),
}

#[derive(Default)]
pub struct State {
    now: DateTime<Local>,
    task: Option<Task>,
    alarm: Option<NaiveTime>,
}

impl Reducers for State {
    type Action = Action;

    fn tick(&mut self, send: impl Effects<Action = Action>) {
        trace!(target: module_path!(), "tick");

        self.now = Local::now();
        self.task = Some(send.after(self.delay(&self.now), Action::Tick));
    }

    fn pause(&mut self, _pause: bool, _send: impl Effects<Action = Self::Action>) {
        todo!()
    }
}

impl State {
    pub fn view(&self, send: impl Effects<Action = Action>) -> impl View {
        if self.task.is_none() {
            send.action(Action::Tick);
        };

        (
            Spacer::fill(),
            self.time(),
            Spacer::fill(),
            self.alarm(),
            Spacer::fill(),
            Spacer::fill(),
            Spacer::fill(),
            self.week(),
            Spacer::height(12.0),
        )
            .justify(Center)
    }

    fn time(&self) -> impl View {
        let str = format!("{}", self.now.time().format("%R"));
        LARGE.text(Nord.5, &str)
    }

    fn alarm(&self) -> impl View {
        let str = self
            .alarm
            .map(|when| {
                let day = if when > self.now.time() {
                    "Today"
                } else {
                    "Tomorrow"
                };

                when.format(&format!("{day} at %R")).to_string()
            })
            .unwrap_or_default();

        SMALL.text(Nord.yellow(), &str)
    }

    fn week(&self) -> impl View {
        (0..7)
            .map(|i| self.day(i))
            .collect_array::<7>()
            .unwrap()
            .inline()
    }

    fn day(&self, day: u8) -> impl View {
        let str = format!("{}", Weekday::try_from(day).unwrap());

        if self.now.weekday().num_days_from_monday() == day as u32 {
            BOLD.text(Nord.4, &str)
        } else {
            BODY.text(Nord.3, &str)
        }
        .padding_horizontal(12.0)
    }

    fn delay(&self, from: &DateTime<Local>) -> Duration {
        let next = from
            .duration_round_up(TimeDelta::minutes(1))
            .expect("duration_round_up");

        next.signed_duration_since(Local::now())
            .to_std()
            .unwrap_or(Duration::from_secs(0))
    }

    pub fn show(&mut self, alarm: NaiveTime) {
        self.alarm = Some(alarm)
    }
}

pub static LARGE: LazyLock<Font<'static>> = LazyLock::new(|| {
    Inter
        .variation(b"wght", 135.0)
        // .feature(b"tnum", 1)
        .size(225.0)
});

pub static BOLD: LazyLock<Font<'static>> = LazyLock::new(|| {
    Inter //
        .variation(b"wght", 350.0)
        .size(28.0)
});

pub static BODY: LazyLock<Font<'static>> = LazyLock::new(|| {
    Inter //
        .variation(b"wght", 200.0)
        .size(28.0)
});

pub static SMALL: LazyLock<Font<'static>> = LazyLock::new(|| {
    Inter //
        .variation(b"wght", 300.0)
        .size(26.0)
});
