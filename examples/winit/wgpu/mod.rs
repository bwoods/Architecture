use std::borrow::Cow;
use std::sync::Arc;

use meshopt::utilities::typed_to_bytes;
use wgpu::util::DeviceExt;
use winit::window::Window;

use composable::views::{Bounds, Size, Transform};

pub struct Surface<'a> {
    surface: wgpu::Surface<'a>,
    scale: f32,

    pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Surface<'_> {
    pub async fn new(window: Arc<Window>) -> Self {
        let (width, height) = window.inner_size().into();
        let scale = window.scale_factor() as f32;

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: Default::default(),
                },
                None,
            )
            .await
            .expect("device");

        let capabilities = surface.get_capabilities(&adapter);

        let format = [
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba32Float,
        ]
        .into_iter()
        .find(|format| surface.get_capabilities(&adapter).formats.contains(format))
        .expect("format");

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    attributes: &wgpu::vertex_attr_array![0 => Uint32, 1 => Uint32],
                    array_stride: std::mem::size_of::<(u32, u32)>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                ..Default::default()
            },
            multiview: None,
        });

        let config = surface
            .get_default_config(&adapter, width, height)
            .expect("config");

        surface.configure(&device, &config);

        Self {
            surface,
            scale,
            pipeline,
            config,
            device,
            queue,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width * height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(
        &mut self,
        vertices: &[(i16, i16, [u8; 4])],
        indices: &[u32],
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let msaa = self
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format: self.config.format,
                view_formats: &[],
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: typed_to_bytes(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: typed_to_bytes(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        #[rustfmt::skip]
        let white = wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &msaa,
                resolve_target: Some(&view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(white),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        let num_indices = indices.len() as u32;
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Converts from Frame buffer to [Normalized Device Coordinates][W3].
    ///
    /// [W3]: https://www.w3.org/TR/webgpu/#coordinate-systems
    pub fn transform(&self) -> Transform {
        Transform::scale(
            2.0 / self.config.width as f32 * self.scale,
            2.0 / self.config.height as f32 * self.scale,
        )
        .then_translate((-1.0, -1.0).into())
        .then_scale(32767.0, 32767.0) // unpack2x16snorm(xy)
        .then_scale(1.0, -1.0) // +y is down
    }

    /// Logical bounds for the WGPU surface
    pub fn bounds(&self) -> Bounds {
        Bounds::from_size(Size::new(
            self.config.width as f32 / self.scale,
            self.config.height as f32 / self.scale,
        ))
    }
}
