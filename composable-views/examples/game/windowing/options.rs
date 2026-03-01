use crate::preferences::Preferences;
use crate::windowing;
use dpi::{PhysicalPosition, PhysicalSize};
use fallible_iterator::FallibleIterator;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::io::{self, ErrorKind::NotFound};
use std::sync::Arc;
use uuid::Uuid;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

#[derive(Serialize, Deserialize)]
struct Options {
    monitor: Uuid,
    position: [i32; 2],
    size: [u32; 2],
    scale: f64,
}

impl Options {
    fn new(window: Arc<dyn Window>, monitor: Uuid) -> Self {
        let scale = window.scale_factor();
        let size = window.surface_size().into();
        let position = window.outer_position().unwrap_or_default().into();

        Self {
            monitor,
            position,
            size,
            scale,
        }
    }
}

static KEY: &str = "main_window";

pub fn read_prefs(event_loop: &dyn ActiveEventLoop) -> io::Result<WindowAttributes> {
    let monitors: Vec<_> = event_loop
        .available_monitors()
        .map(|monitor| Uuid::from_u128_le(monitor.id()))
        .collect();

    let preferences = Preferences::default()?;

    // most recent window location (whose monitor is currently in the system)
    let most_recent = match preferences.iter(KEY) {
        Ok(iter) => iter
            .filter(|options: &Options| Ok(monitors.contains(&options.monitor)))
            .next()?,
        Err(err) if err.kind() == NotFound => None,
        Err(err) => return Err(err),
    };

    let mut attributes = match most_recent {
        Some(options) => WindowAttributes::default()
            .with_position::<PhysicalPosition<i32>>(options.position.into())
            .with_surface_size(PhysicalSize::<u32>::from(options.size)),
        None => {
            let within = event_loop
                .primary_monitor()
                .and_then(|monitor| monitor.0.current_video_mode())
                .map(|mode| mode.size());

            match within {
                None => WindowAttributes::default()
                    .with_surface_size(*windowing::sizes::DEFAULT)
                    .with_position(PhysicalPosition::<i32>::default()),
                Some(PhysicalSize { width, height }) => {
                    let size = windowing::sizes::fit_width(width);
                    let x = (width as i32 - size.width as i32) / 2;
                    let y = (height as i32 - size.height as i32) / 2;

                    WindowAttributes::default()
                        .with_surface_size(size)
                        .with_position(PhysicalPosition::new(x, y))
                }
            }
        }
    };

    attributes = attributes.with_visible(false);

    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::WindowAttributesMacOS;
        use winit::window::PlatformWindowAttributes;

        attributes = attributes.with_platform_attributes(
            WindowAttributesMacOS::default()
                .with_titlebar_transparent(true)
                .with_fullsize_content_view(true)
                .box_clone(),
        );
    }

    Ok(attributes)
}

pub fn save_prefs(window: Arc<dyn Window>) -> io::Result<()> {
    let uuid = match window.current_monitor() {
        Some(monitor) => Uuid::from_u128_le(monitor.id()),
        None => return Ok(()), // no monitor handle → no info to save…
    };

    let mut preferences = Preferences::default()?;
    let mut all: VecDeque<Options> = match preferences.iter(KEY) {
        Ok(iter) => iter.collect()?,
        Err(err) if err.kind() == NotFound => Default::default(),
        Err(err) => return Err(err),
    };

    all.retain(|setting| setting.monitor != uuid);
    all.push_front(Options::new(window, uuid));

    const MAX: usize = 12; // only keep so many window options around
    preferences.replace(KEY, all.into_iter().take(MAX))?;
    preferences.commit()
}
