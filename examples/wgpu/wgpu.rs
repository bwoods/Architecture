pub use ::wgpu::{Instance, Surface};
use ::winit::window::Window;

use composable::*;

#[derive(Debug, Default)]
pub struct State {
    inner: Option<Inner>,
}

#[derive(Clone, Debug)]
pub enum Action {
    Setup(&'static Window),
}

impl Reducer for State {
    type Action = Action;
    type Output = ();

    fn into_inner(self) -> Self::Output {}

    async fn reduce_async(
        &mut self,
        action: Self::Action,
        _effects: impl Effects<Action = Self::Action>,
    ) {
        match action {
            Action::Setup(window) => self.set(window).await,
        }
    }

    // fn into_inner(self) -> Self::Output {}
}

#[derive(Debug)]
struct Inner {
    surface: Surface<'static>,

    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'a> State {
    pub async fn set(&mut self, window: &'static Window) {
        let instance = wgpu::Instance::default();
        let surface = surface_from(&instance, window);

        let (width, height) = window.inner_size().into();

        let adapter =
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    // Request an adapter which can render to our surface
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("device");

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let inner = Inner {
            surface,
            config,
            device,
            queue,
        };

        self.inner.replace(inner);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(inner) = self.inner.as_mut() {
            inner.config.width = width;
            inner.config.height = height;
            inner.surface.configure(&inner.device, &inner.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(inner) = self.inner.as_mut() {
            let output = inner.surface.get_current_texture()?;
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                inner
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("encoder"),
                    });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            inner.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }

        Ok(())
    }
}

/// Given that [`create_surface`] has this warning
///
/// > # Panics
/// > - On macOS/Metal: will panic if not called on the main thread.
///
/// We work around this limitation when `#[cfg(target_os = "macos")]` rather than complicate
/// the logic for _all_ platforms.
///
/// [`create_surface`]: wgpu::Instance::create_surface
fn surface_from(instance: &Instance, window: &'static Window) -> Surface<'static> {
    #[cfg(target_os = "macos")]
    {
        let queue = dispatch::Queue::main();
        queue.exec_sync(|| {
            return instance.create_surface(window).unwrap();
        })
    }

    #[cfg(not(target_os = "macos"))]
    {
        return instance.create_surface(window).unwrap();
    }
}
