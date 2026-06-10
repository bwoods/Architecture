use crate::alarm::player::rest::Rest;
use crate::nord::Nord;
use crate::versioning::{CRATE, PACKAGE};
use chrono::{Local, NaiveTime, TimeDelta};
use composable::*;
use composable_views::*;
use directories::ProjectDirs;
use face::Face;
use note::Note;
use raplay::{Sink, Source, source::Symph};
use std::fs::{File, create_dir_all};
use std::io::Cursor;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;

mod face;
mod note;
mod rest;

#[derive(Clone, From, TryInto, Reducers)]
pub enum Action {
    Play,
    Pause(Duration),
    #[from(ignore)]
    Sleep(Duration),

    Next,
    Last,

    Begin,
    #[reducer(ignore)]
    Set(NaiveTime),
}

#[derive(Default)]
pub struct State {
    pub(crate) sink: Sink,
    track: Track,
    task: Option<Task>,
}

impl Reducers for State {
    type Action = Action;

    fn play(&mut self, send: impl Effects<Action = Action>) {
        let (duration, delay) = self.track.play(&mut self.sink);
        self.task = Some(send.after(
            duration,
            // play the next one after a short pause; except for the last track, which loops continuously
            Action::Pause(delay.checked_sub(duration).unwrap_or_default()),
        ));
    }

    fn pause(&mut self, duration: Duration, send: impl Effects<Action = Self::Action>) {
        self.sink.play(false).expect("pause");
        self.task = Some(send.after(duration, Action::Next));
    }

    fn sleep(&mut self, duration: Duration, send: impl Effects<Action = Self::Action>) {
        self.sink.play(false).expect("sleep");
        self.track = Track::None;
        self.task = Some(send.after(duration, Action::Play));
    }

    fn next(&mut self, send: impl Effects<Action = Action>) {
        let playing = self.sink.is_playing().unwrap();
        let lullaby = matches!(self.track, Track::None);

        if playing && lullaby {
            self.track.next();
        }

        self.track.next();
        self.play(send);
    }

    fn last(&mut self, send: impl Effects<Action = Action>) {
        self.track = Track::Custom;

        if self.sink.is_playing().unwrap() {
            self.pause(Duration::from_secs(180), send);
        } else {
            self.play(send);
        }
    }

    /// begin the (alarm) cycle
    fn begin(&mut self, send: impl Effects<Action = Action>) {
        // begin with a soft lullaby; it lets the user adjust their volume
        self.track = Track::None;
        let (duration, _) = self.track.play(&mut self.sink);

        // TODO: read from .toml file
        let wake = NaiveTime::from_hms_opt(7, 45, 0).unwrap();
        let now = Local::now();

        let delay = if now.time() < wake {
            wake.signed_duration_since(now.time())
                .to_std()
                .expect("signed_duration_since (today)")
        } else {
            let tomorrow = (now + TimeDelta::days(1)).date_naive().and_time(wake);
            tomorrow
                .signed_duration_since(now.naive_local())
                .to_std()
                .expect("signed_duration_since (tomorrow)")
        };

        info!(target: module_path!(), "set alarm: {wake} (in {:.2} hours)", delay.as_secs_f32() / 3600.0);
        send.action(Action::Set(wake));

        self.task = Some(send.after(
            duration,
            Action::Sleep(delay.checked_sub(duration).unwrap_or_default()),
        ));
    }
}

impl State {
    pub const W: f32 = 128.0;
    pub const R: f32 = Self::W / 12.0 + 1.0;
    pub const P: f32 = Self::R + 5.0;

    pub const ON: [u8; 4] = Nord.n(7);
    pub const OFF: [u8; 4] = Nord.n(1);
    pub const WAITING: [u8; 4] = Nord.n(3);

    pub fn view(&self, send: impl Effects<Action = Action>) -> impl View {
        (
            self.face(send.clone()).padding_bottom(Self::P),
            self.note(send.clone()).padding_bottom(Self::P),
            self.sleep(send),
        )
    }

    #[allow(clippy::bool_comparison)]
    pub fn face(&self, send: impl Effects<Action = Action>) -> impl View {
        let playing = self.sink.is_playing().unwrap();
        let singing = matches!(self.track, Track::Custom) == false;
        let lullaby = matches!(self.track, Track::None);
        let waiting = self.task.is_some();

        let color = match (playing, singing, lullaby, waiting) {
            (false, true, false, true) => Self::WAITING,
            (true, true, _, _) => Self::ON,
            _ => Self::OFF,
        };

        Face.fixed(Self::W, Self::W)
            .on_tap(ui_id!(), Action::Next, send)
            .background(
                Rectangle { rgba: color }
                    .rounded(Self::R, Self::R)
                    .continuous(),
            )
    }

    pub fn note(&self, send: impl Effects<Action = Action>) -> impl View {
        let playing = self.sink.is_playing().unwrap();
        let ringing = matches!(self.track, Track::Custom);
        let waiting = self.task.is_some();

        let color = match (playing, ringing, waiting) {
            (false, true, true) => Self::WAITING,
            (true, true, _) => Self::ON,
            _ => Self::OFF,
        };

        Note.fixed(Self::W, Self::W)
            .on_tap(ui_id!(), Action::Last, send)
            .background(
                Rectangle { rgba: color }
                    .rounded(Self::R, Self::R)
                    .continuous(),
            )
    }

    pub fn sleep(&self, send: impl Effects<Action = Action>) -> impl View {
        let lullaby = matches!(self.track, Track::None);
        let waiting = self.task.is_some();

        let color = match (lullaby, waiting) {
            (true, true) => Self::WAITING,
            _ => Self::OFF,
        };

        Rest.fixed(Self::W, Self::W)
            .on_tap(ui_id!(), Action::Begin, send)
            .background(
                Rectangle { rgba: color }
                    .rounded(Self::R, Self::R)
                    .continuous(),
            )
    }
}

#[derive(Debug, Default)]
enum Track {
    /// [female humming.wav][wav] by Alexandra Drotz Ruhn | License: [Attribution 4.0][CC]
    ///
    /// [wav]: https://freesound.org/people/drotzruhn/sounds/405207/
    /// [CC]: https://creativecommons.org/licenses/by/4.0/
    Humming,
    /// [byssanlull.wav][wav] by Alexandra Drotz Ruhn | License: [Attribution 4.0][CC]
    ///
    /// [wav]: https://freesound.org/people/drotzruhn/sounds/405202/
    /// [CC]: https://creativecommons.org/licenses/by/4.0/
    ByssanLull,
    /// [female singing ooo][wav] by Alexandra Drotz Ruhn | License: [Attribution 4.0][CC]
    ///
    /// [wav]: https://freesound.org/people/drotzruhn/sounds/625399/
    /// [CC]: https://creativecommons.org/licenses/by/4.0/
    Singing,
    /// [clearing throat.wav][wav] by Alexandra Drotz Ruhn | License: [Attribution 4.0][CC]
    ///
    /// [wav]: https://freesound.org/people/drotzruhn/sounds/405205/
    /// [CC]: https://creativecommons.org/licenses/by/4.0/
    Ahem,
    /// [laughter.wav][wav] by Alexandra Drotz Ruhn | License: [Attribution 4.0][CC]
    ///
    /// [wav]: https://freesound.org/people/drotzruhn/sounds/405209/
    /// [CC]: https://creativecommons.org/licenses/by/4.0/
    Laughter,
    /// Custom audio track; added by the user
    Custom,
    #[default]
    None,
}

impl Track {
    fn play(&mut self, sink: &mut Sink) -> (Duration, Duration) {
        info!(target: module_path!(), "{self:?}");

        let bytes: &'static [u8] = match self {
            // afconvert -c 2 -f mp4f -d 'aac ' {FILENAME}.wav
            Track::Humming | Track::None => include_bytes!("drotzruhn/female-humming.mp4"),
            Track::ByssanLull => include_bytes!("drotzruhn/byssanlull.mp4"),
            Track::Singing => include_bytes!("drotzruhn/female-singing-ooo.mp4"),
            Track::Ahem => include_bytes!("drotzruhn/clearing-throat.mp4"),
            Track::Laughter => include_bytes!("drotzruhn/laughter.mp4"),
            Track::Custom => return self.custom(sink), // custom tracks are loaded from the file-system
        };

        let source = Symph::try_new(Cursor::new(bytes), &Default::default()).expect("source");
        let length = source.get_time().map(|ts| ts.total).unwrap_or_default();

        sink.pause().expect("pause");
        sink.volume(self.volume()).expect("volume");
        sink.load(Box::new(source), true).expect("load");

        (length, Duration::from_secs(180))
    }

    fn custom(&mut self, sink: &mut Sink) -> (Duration, Duration) {
        let directories = ProjectDirs::from("", PACKAGE, CRATE) // fails on Android and iOS
            .expect("directories");

        let directory = directories.config_local_dir();
        let mut path = PathBuf::new();
        path.push(directory);
        create_dir_all(&path).expect("create_dir_all");

        path.push("Custom");
        path.set_extension("mp4");

        let file = File::open(&path).unwrap();
        let source = Symph::try_new(file, &Default::default()).expect("source");
        let length = source.get_time().map(|ts| ts.total).unwrap_or_default();

        sink.pause().expect("pause");
        sink.volume(self.volume()).expect("volume");
        sink.load(Box::new(source), true).expect("load");

        (length, length + Duration::from_secs(2))
    }

    fn volume(&self) -> f32 {
        match self {
            Track::None => 0.35,
            Track::Humming => 0.5,
            Track::ByssanLull => 0.75,
            Track::Singing => 0.5,
            Track::Ahem => 1.0,
            Track::Laughter => 0.5,
            Track::Custom => 1.0,
        }
    }

    fn next(&mut self) -> &mut Self {
        *self = match self {
            Track::None => Track::Humming,
            Track::Humming => Track::ByssanLull,
            Track::ByssanLull => Track::Singing,
            Track::Singing => Track::Ahem,
            Track::Ahem => Track::Laughter,
            Track::Laughter => Track::Custom,
            Track::Custom => Track::Custom,
        };

        self
    }
}
