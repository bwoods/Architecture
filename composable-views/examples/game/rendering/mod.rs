#![allow(clippy::type_complexity)]

use composable::{Effects, Reducers};
use dpi::{PhysicalInsets, PhysicalSize};
use meshopt::typed_to_bytes;
use std::borrow::Cow;
use std::sync::Arc;
use take_once::TakeOnce;
use wgpu::util::DeviceExt;
use wgpu::{
    Adapter, BlendState, BufferAddress, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, DeviceDescriptor, Extent3d, Features, FragmentState,
    IndexFormat, Instance, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor,
    PowerPreference, PresentMode, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderSource, StoreOp, Surface, SurfaceConfiguration, SurfaceError,
    TextureDescriptor, TextureDimension, TextureUsages, TextureViewDescriptor, VertexBufferLayout,
    VertexState, VertexStepMode, util, vertex_attr_array,
};

#[derive(Clone, Reducers)]
pub enum Action {
    Initializing(Arc<TakeOnce<(Surface<'static>, Instance, PhysicalInsets<u32>)>>),
    Initialized(
        Arc<
            TakeOnce<(
                Surface<'static>,
                Adapter,
                Device,
                Queue,
                PhysicalInsets<u32>,
            )>,
        >,
    ),
}

/// # Returns
/// An action that performs the initial _async_ setup for rendering.
pub fn setup(surface: Surface<'static>, instance: Instance, bounds: PhysicalInsets<u32>) -> Action {
    Action::Initializing(Arc::new(TakeOnce::new_with((surface, instance, bounds))))
}

/// # Returns
/// An action that finalizes setup; leaving `State` [`Ready`] for rendering.
///
/// [`Ready`]: State::Ready
#[rustfmt::skip]
fn initialized(surface: Surface<'static>, adapter: Adapter, device: Device, queue: Queue, bounds: PhysicalInsets<u32>) -> Action {
    Action::Initialized(Arc::new(TakeOnce::new_with((surface, adapter, device, queue, bounds))))
}

pub enum State {
    /// State is still being initialized. Doing so is asynchronous and `State`
    /// will remain `Pending` until it completes.
    Pending {
        size: PhysicalSize<u32>,
        scale: f64,
        vsync: Vsync,
    },
    /// Initialization is complete and State is ready for rendering.
    Ready {
        surface: Surface<'static>,
        config: SurfaceConfiguration,
        bounds: PhysicalInsets<u32>,
        scale: f64,

        pipeline: RenderPipeline,
        device: Device,
        queue: Queue,
    },
}

impl Reducers for State {
    type Action = Action;

    fn initializing(
        &mut self,
        values: Arc<TakeOnce<(Surface<'static>, Instance, PhysicalInsets<u32>)>>,
        send: impl Effects<Action = Self::Action>,
    ) {
        let (surface, instance, bounds) = values.take().unwrap(); // FIXME: will fail within recursive reducers
        send.future(async move {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("adapter");

            let (device, queue) = adapter
                .request_device(&DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Default::default(),
                    experimental_features: Default::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                })
                .await
                .expect("device");

            Some(initialized(surface, adapter, device, queue, bounds))
        });
    }

    fn initialized(
        &mut self,
        values: Arc<
            TakeOnce<(
                Surface<'static>,
                Adapter,
                Device,
                Queue,
                PhysicalInsets<u32>,
            )>,
        >,
        send: impl Effects<Action = Self::Action>,
    ) {
        let (surface, adapter, device, queue, bounds) = values.take().unwrap();
        let (width, height, scale, vsync) = match self {
            State::Pending { size, scale, vsync } => (size.width, size.height, *scale, *vsync),
            State::Ready { .. } => unreachable!(),
        };

        let format = *surface
            .get_capabilities(&adapter)
            .formats
            .first() // “The first format in the vector is preferred.”
            .expect("formats");

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[VertexBufferLayout {
                    attributes: &vertex_attr_array![0 => Uint32, 1 => Uint32],
                    array_stride: size_of::<(u32, u32)>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState {
                count: 4,
                ..Default::default()
            },
            multiview: None,
            cache: None,
        });

        let config = surface
            .get_default_config(&adapter, width, height)
            .map(|mut config| {
                config.present_mode = vsync.into();
                config
            })
            .expect("config");

        surface.configure(&device, &config);

        *self = State::Ready {
            surface,
            config,
            bounds,
            scale,
            pipeline,
            device,
            queue,
        }
    }
}

impl State {
    pub fn rescale(&mut self, rescale: f64) {
        match self {
            State::Pending { scale, .. } => *scale = rescale,
            State::Ready {
                scale,
                config,
                bounds,
                ..
            } => {
                let bounds = *bounds;
                let resize = PhysicalSize::new(config.width, config.height)
                    .to_logical::<u32>(*scale) // resize to the new scale
                    .to_physical(rescale);

                *scale = rescale;
                self.resize(resize, bounds);
            }
        }
    }

    pub fn resize(&mut self, resize: PhysicalSize<u32>, reinset: PhysicalInsets<u32>) {
        if resize.width * resize.height == 0 {
            return;
        }

        match self {
            State::Pending { size, .. } => {
                *size = resize;
            }
            State::Ready {
                config,
                surface,
                device,
                bounds,
                ..
            } => {
                config.width = resize.width;
                config.height = resize.height;
                surface.configure(device, config);
                *bounds = reinset;
            }
        };
    }

    pub fn render(
        &self,
        vertices: &[(i16, i16, [u8; 4])],
        indices: &[u32],
    ) -> Result<(), SurfaceError> {
        match self {
            State::Pending { .. } => Ok(()),
            State::Ready {
                surface,
                device,
                config,
                pipeline,
                queue,
                ..
            } => {
                let output = surface.get_current_texture()?;
                let view = output
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                let msaa = device
                    .create_texture(&TextureDescriptor {
                        label: None,
                        size: Extent3d {
                            width: config.width,
                            height: config.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 4,
                        dimension: TextureDimension::D2,
                        format: config.format,
                        view_formats: &[],
                        usage: TextureUsages::RENDER_ATTACHMENT,
                    })
                    .create_view(&TextureViewDescriptor::default());

                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor { label: None });

                let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
                    label: None,
                    contents: typed_to_bytes(vertices),
                    usage: BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
                    label: None,
                    contents: typed_to_bytes(indices),
                    usage: BufferUsages::INDEX,
                });

                #[rustfmt::skip]
                let background = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &msaa,
                        depth_slice: None,
                        resolve_target: Some(&view),
                        ops: Operations {
                            load: LoadOp::Clear(background),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                #[allow(clippy::bool_comparison)]
                if indices.is_empty() == false {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);

                    let num_indices = indices.len() as u32;
                    render_pass.draw_indexed(0..num_indices, 0, 0..1);
                }

                drop(render_pass);
                queue.submit(std::iter::once(encoder.finish()));
                output.present();

                Ok(())
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum Vsync {
    Off = 1, // https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html
    #[default]
    On = 2,
    Adaptive = 0,
}

impl From<Vsync> for PresentMode {
    fn from(value: Vsync) -> Self {
        match value {
            Vsync::Off => PresentMode::AutoNoVsync,
            Vsync::On => PresentMode::Fifo,
            Vsync::Adaptive => PresentMode::AutoVsync,
        }
    }
}
