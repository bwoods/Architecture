#![allow(clippy::type_complexity)]

use crate::nord::Nord;
use composable::*;
use dpi::{LogicalSize, PhysicalSize, Pixel};
use meshopt::typed_to_bytes;
use std::borrow::Cow;
use std::sync::Arc;
use take_once::TakeOnce;
use tracing::debug;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Adapter, BlendState, BufferAddress, BufferUsages, ColorTargetState, ColorWrites,
    CurrentSurfaceTexture, Device, DeviceDescriptor, Extent3d, Features, FragmentState,
    IndexFormat, Instance, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor,
    PowerPreference, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, StoreOp, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension,
    TextureUsages, TextureViewDescriptor, VertexBufferLayout, VertexState, VertexStepMode,
    vertex_attr_array,
};

#[derive(Clone, From, TryInto, Reducers)]
pub enum Action {
    /// Performs the initial _async_ setup for rendering.
    Setup(Arc<TakeOnce<(Surface<'static>, Instance)>>),

    /// Finalizes setup; leaving `State` [`Ready`] for rendering.
    ///
    /// [`Ready`]: State::Ready
    #[rustfmt::skip]
    Ready(Arc<TakeOnce<(Surface<'static>, Adapter, Device, Queue)>>),

    /// Reset `State` to `Pending` and then sends the `Error(Error)`
    #[from(skip)]
    Reset(Error),

    /// Reporting errors to parent Domain(s). Not used by this reducer.
    #[reducer(ignore)]
    Error(Error),
}

#[derive(Clone, Debug)]
pub enum Error {
    /// Successfully acquired a surface texture, but texture no longer matches the properties of the underlying surface.
    /// It's highly recommended to call [`Surface::configure`] again for optimal performance.
    Suboptimal,
    /// A timeout was encountered while trying to acquire the next frame.
    ///
    /// Applications should skip the current frame and try again later.
    Timeout,
    /// The window is occluded (e.g. minimized or behind another window).
    ///
    /// Applications should skip the current frame and try again once the window
    /// is no longer occluded.
    Occluded,
    /// The underlying surface has changed, and therefore the surface configuration is outdated.
    ///
    /// Call [`Surface::configure()`] and try again.
    Outdated,
    /// The surface has been lost and needs to be recreated.
    ///
    /// If the device as a whole is lost (see [`set_device_lost_callback()`][crate::Device::set_device_lost_callback]), then
    /// you need to recreate the device and all resources.
    /// Otherwise, call [`Instance::create_surface()`] to recreate the surface,
    /// then [`Surface::configure()`], and try again.
    Lost,
    /// A validation error inside [`Surface::get_current_texture()`] was raised
    /// and caught by an [error scope](crate::Device::push_error_scope) or
    /// [`on_uncaptured_error()`][crate::Device::on_uncaptured_error].
    ///
    /// Applications should attend to the validation error and try again.
    Validation,
}

pub enum State {
    /// `State` is still being initialized. Doing so is asynchronous and `State`
    ///  will remain `Pending` until it completes.
    Pending {
        size: PhysicalSize<u32>,
        scale: f32,
        // vsync: Vsync,
    },
    /// Setup is complete and `State` is ready for rendering.
    Ready {
        surface: Surface<'static>,
        config: SurfaceConfiguration,
        scale: f32,

        pipeline: RenderPipeline,
        device: Device,
        queue: Queue,
    },
}

impl State {
    pub fn render(
        &self,
        vertices: &[(i16, i16, [u8; 4])],
        indices: &[u32],
        send: impl Effects<Action = Action>,
    ) {
        match self {
            State::Pending { .. } => (),
            State::Ready {
                surface,
                device,
                config,
                pipeline,
                queue,
                ..
            } => {
                let output = match surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture) => texture,
                    CurrentSurfaceTexture::Timeout => {
                        send.action(Error::Timeout);
                        return;
                    }
                    CurrentSurfaceTexture::Occluded => {
                        send.action(Error::Occluded);
                        return;
                    }
                    CurrentSurfaceTexture::Suboptimal(texture) => {
                        send.action(Action::Reset(Error::Suboptimal));
                        texture
                    }
                    CurrentSurfaceTexture::Outdated => {
                        send.action(Action::Reset(Error::Outdated));
                        return;
                    }
                    CurrentSurfaceTexture::Lost => {
                        send.action(Action::Reset(Error::Lost));
                        return;
                    }
                    CurrentSurfaceTexture::Validation => {
                        send.action(Action::Reset(Error::Validation));
                        return;
                    }
                };

                let view = output.texture.create_view(&TextureViewDescriptor {
                    format: Some(config.format.add_srgb_suffix()),
                    ..Default::default()
                });

                #[rustfmt::skip]
                let msaa = device
                    .create_texture(&TextureDescriptor {
                        label: None,
                        size: Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
                        mip_level_count: 1,
                        sample_count: 4,
                        dimension: TextureDimension::D2,
                        format: config.format,
                        view_formats: &[],
                        usage: TextureUsages::RENDER_ATTACHMENT,
                    })
                    .create_view(&TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&Default::default());

                let rbg = Nord.f64(0);
                let background = wgpu::Color {
                    r: rbg[0],
                    g: rbg[1],
                    b: rbg[2],
                    a: rbg[3],
                };

                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &msaa,
                        depth_slice: None,
                        resolve_target: Some(&view),
                        ops: Operations {
                            load: LoadOp::Clear(background),
                            store: StoreOp::Discard,
                            // “Storing pre-resolve MSAA data is unnecessary if it isn't used later.
                            //  On tile-based GPU, avoid store can reduce your app's memory footprint.”
                            //    — https://github.com/gfx-rs/wgpu/blob/trunk/examples/features/src/msaa_line/mod.rs#L296
                            //  Also:
                            //    - https://docs.vulkan.org/guide/latest/tile_based_rendering_best_practices.html#msaa-resolve-patterns
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });

                let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: typed_to_bytes(vertices),
                    usage: BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: typed_to_bytes(indices),
                    usage: BufferUsages::INDEX,
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

                queue.submit([encoder.finish()]);
                // window.pre_present_notify();
                output.present();
            }
        }
    }

    fn scale(&self) -> f32 {
        match self {
            State::Pending { scale, .. } => *scale,
            State::Ready { scale, .. } => *scale,
        }
    }

    pub fn physical_size(&self) -> PhysicalSize<u32> {
        match self {
            State::Pending { size, .. } => *size,
            State::Ready { config, .. } => PhysicalSize::new(config.width, config.height),
        }
    }

    pub fn logical_size(&self) -> LogicalSize<f32> {
        self.physical_size().to_logical(self.scale().cast())
    }

    pub fn rescale(&mut self, rescale: f32) {
        match self {
            State::Pending { scale, .. } => *scale = rescale,
            State::Ready { scale, config, .. } => {
                let resize = PhysicalSize::new(config.width, config.height)
                    .to_logical::<u32>(*scale as f64) // resize to the new scale
                    .to_physical(rescale as f64);

                *scale = rescale;
                self.resize(resize.width, resize.height);
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width * height == 0 {
            return;
        }

        debug!(target: module_path!(), "resize {width}×{height}");

        match self {
            State::Pending { size, .. } => {
                *size = PhysicalSize::new(width, height);
            }
            State::Ready {
                config,
                surface,
                device,
                ..
            } => {
                config.width = width;
                config.height = height;
                surface.configure(device, config);
            }
        };
    }
}

impl Reducers for State {
    type Action = Action;

    fn setup(
        &mut self,
        values: Arc<TakeOnce<(Surface<'static>, Instance)>>,
        send: impl Effects<Action = Self::Action>,
    ) {
        let (surface, instance) = values.take().unwrap(); // FIXME: will fail within recursive reducers

        send.future(async move {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::default(), // TODO
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("adapter");

            let (device, queue) = adapter
                .request_device(&DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(), // TODO
                    required_limits: Default::default(),
                    experimental_features: Default::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                })
                .await
                .expect("device");

            Some(Action::Ready(Arc::new(TakeOnce::new_with((
                surface, adapter, device, queue,
            )))))
        })
    }

    fn ready(
        &mut self,
        values: Arc<TakeOnce<(Surface<'static>, Adapter, Device, Queue)>>,
        _send: impl Effects<Action = Self::Action>,
    ) {
        let (surface, adapter, device, queue) = values.take().unwrap();
        let (width, height, scale) = match self {
            State::Pending { size, scale } => (size.width, size.height, *scale),
            State::Ready { .. } => unreachable!(),
        };

        let config = surface
            .get_default_config(&adapter, width, height)
            .map(|mut config| {
                config
                    .view_formats
                    .iter_mut()
                    .for_each(|format| *format = format.add_srgb_suffix());
                // “Request compatibility with the sRGB-format texture view we’re going to create later.”

                // config.present_mode = vsync.into();
                config
            })
            .expect("config");

        surface.configure(&device, &config);

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
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
                    format: config.format,
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
            cache: None,
            multiview_mask: None,
        });

        *self = State::Ready {
            surface,
            config,
            scale,
            pipeline,
            device,
            queue,
        }
    }

    fn reset(&mut self, cause: Error, send: impl Effects<Action = Action>) {
        *self = State::Pending {
            size: self.physical_size(),
            scale: self.scale(),
        };
        send.action(cause)
    }
}

pub fn setup(surface: Surface<'static>, instance: Instance) -> Action {
    Action::Setup(Arc::new(TakeOnce::new_with((surface, instance))))
}
