use ::wgpu::Surface;
use ::winit::window::Window;

use composable::*;

pub struct State<'window> {
    surface: Surface<'window>,

    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Clone, Debug)]
pub enum Action {
    Resize { width: u32, height: u32 },
    Render,
}

impl Reducer for State<'_> {
    type Action = Action;
    type Output = ();

    fn into_inner(self) -> Self::Output {}

    fn reduce(&mut self, action: Action, _effects: impl Effects<Action = Action>) {
        match action {
            Action::Resize { width, height } => {
                self.resize(width, height);
            }
            Action::Render => {
                self.render().ok();
            }
        }
    }
}

impl<'window> State<'window> {
    pub(crate) async fn new(window: &'window Window) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let (width, height) = window.inner_size().into();

        let adapter = instance
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

        Self {
            surface,
            config,
            device,
            queue,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width * height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        #[rustfmt::skip]
        let white = wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(white),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // render_pass.set_pipeline(&self.pipeline);
        // render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        // render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        //
        // render_pass.draw_indexed(0..num_indices, 0, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
