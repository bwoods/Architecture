//! GPU [`Output`] for `Views`
use lyon::path::builder::{NoAttributes, Transformed};
use lyon::path::{BuilderImpl as Builder, Path};
use lyon::tessellation::{
    FillGeometryBuilder, FillOptions, FillTessellator, FillVertex, GeometryBuilder,
    GeometryBuilderError, VertexId,
};

use crate::views::Transform;

///  
pub struct Output {
    storage: Storage,
    options: FillOptions,
    builder: NoAttributes<Transformed<Builder, Transform>>,
}

impl Output {
    /// Creates an indexed-triangle data `Output`.
    pub fn new(options: FillOptions) -> Self {
        let builder = Self::builder();
        let storage = Storage::default();

        Self {
            storage,
            options,
            builder,
        }
    }

    /// Consumes the `Output` and returns the constructed indexed-triangle data.
    /// - vertices are stored as (x, y, rgba) tuples
    /// - indices are stored as 32-bit offsets
    ///
    /// ## Example
    /// A WGSL shader that consumes this `Output`’s output:
    ///
    /// ```wgsl
    #[doc = include_str!("../../../examples/winit/wgpu/shader.wgsl")]
    /// ```
    pub fn into_inner(mut self) -> (Vec<(f32, f32, u32)>, Vec<u32>) {
        self.tessellate();
        self.storage.into_inner()
    }

    fn tessellate(&mut self) {
        let builder = std::mem::replace(&mut self.builder, Self::builder());

        let path = builder.build();
        let mut tessellator = FillTessellator::default();
        tessellator
            .tessellate_path(&path, &self.options, &mut self.storage)
            .expect("tessellate_path");
    }

    fn builder() -> NoAttributes<Transformed<Builder, Transform>> {
        Path::builder().transformed(Default::default())
    }
}

impl super::Output for Output {
    #[inline]
    fn begin(&mut self, x: f32, y: f32, rgba: [u8; 4], transform: &Transform) {
        if rgba != self.storage.rgba {
            self.tessellate();
        }

        self.storage.rgba = rgba;
        self.builder.inner_mut().set_transform(*transform);

        self.builder.begin((x, y).into());
    }

    #[inline]
    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to((x, y).into());
    }

    #[inline]
    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder
            .quadratic_bezier_to((x1, y1).into(), (x, y).into());
    }

    #[inline]
    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder
            .cubic_bezier_to((x1, y1).into(), (x2, y2).into(), (x, y).into());
    }

    #[inline(never)]
    fn close(&mut self) {
        self.builder.close();
    }
}

///
#[derive(Default)]
struct Storage {
    vertices: Vec<(f32, f32, u32)>,
    indices: Vec<u32>,
    rgba: [u8; 4],
}

impl Storage {
    pub fn into_inner(self) -> (Vec<(f32, f32, u32)>, Vec<u32>) {
        (self.vertices, self.indices)
    }
}

#[doc(hidden)]
impl FillGeometryBuilder for Storage {
    #[inline]
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        let id = self.vertices.len() as u32;
        let (x, y) = vertex.position().into();

        self.vertices // TODO: packSnorm2x16 ?
            .push((x, y, u32::from_le_bytes(self.rgba)));

        Ok(id.into())
    }
}

#[doc(hidden)]
impl GeometryBuilder for Storage {
    #[inline]
    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        let triangle: [u32; 3] = [a, b, c].map(|id| id.into());
        self.indices.extend_from_slice(&triangle);
    }
}
