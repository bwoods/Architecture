//! GPU [`Output`] for `Views`

use crate::Transform;
use log::{trace, warn};
use lyon::path::builder::{NoAttributes, Transformed};
use lyon::path::{BuilderImpl as Builder, Path};
use lyon::tessellation::{
    FillGeometryBuilder, FillOptions, FillTessellator, FillVertex, GeometryBuilder,
    GeometryBuilderError, VertexId,
};
use std::mem::replace;

///
pub struct Output {
    builder: NoAttributes<Transformed<Builder, Transform>>,
    storage: Storage,
    options: FillOptions,
    width: f32,
    height: f32,
}

impl Output {
    /// Creates an indexed-triangle data `Output`.
    ///
    /// ## Note
    /// The output is stored in [Normalized Device Coordinates][W3].
    ///
    /// [W3]: https://www.w3.org/TR/webgpu/#coordinate-systems
    pub fn new(rounding: f32, width: f32, height: f32) -> Self {
        let builder = Path::builder().transformed(Default::default());
        let storage = Storage::default();
        let options = FillOptions::non_zero()
            // .with_intersections(true)
            .with_tolerance(if rounding > 0.0 {
                rounding
            } else {
                warn!(target: module_path!(), "rounding should be >= 0, not {rounding}");
                FillOptions::DEFAULT_TOLERANCE
            });

        Self {
            builder,
            storage,
            options,
            width,
            height,
        }
    }

    /// Consumes the `Output` and returns the constructed indexed-triangle data.
    /// - vertices are stored as (x, y, rgba) tuples
    /// - indices are stored as 32-bit offsets
    ///
    /// ## Example
    /// An example WGSL shader that consumes this `Output`’s output:
    ///
    /// ```wgsl
    #[doc = include_str!("../../../examples/play/rendering/shader.wgsl")]
    /// ```
    /// ## Note
    /// The output is stored in [Normalized Device Coordinates][W3].
    ///
    /// [W3]: https://www.w3.org/TR/webgpu/#coordinate-systems
    #[allow(clippy::type_complexity)]
    pub fn into_inner(mut self) -> (Vec<(i16, i16, [u8; 4])>, Vec<u32>) {
        self.tessellate();
        self.storage.into_inner()
    }

    fn tessellate(&mut self) {
        let builder = replace(
            &mut self.builder,
            Path::builder().transformed(Default::default()),
        );

        let path = builder.build();
        let mut tessellator = FillTessellator::default();
        tessellator
            .tessellate_path(&path, &self.options, &mut self.storage)
            .expect("tessellate_path");
    }

    fn transform(&self, outer: &Transform) -> Transform {
        let normalized = Transform::translation(-1.0, -1.0)
            .then_scale(32767.0, -32767.0) // unpack2x16snorm(xy)
            .pre_scale(2.0 / self.width, 2.0 / self.height); // doing the variable values last…

        outer.then(&normalized)
    }
}

impl super::Output for Output {
    fn begin(&mut self, rgba: [u8; 4], transform: &Transform) {
        if rgba != self.storage.rgba {
            self.tessellate(); // before the color change…
            self.storage.rgba = rgba;
        }

        let transform = self.transform(transform);
        self.builder.inner_mut().set_transform(transform);
    }

    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
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

    #[inline]
    fn close(&mut self) {
        self.builder.close();
    }
}

///
#[derive(Default)]
struct Storage {
    vertices: Vec<(i16, i16, [u8; 4])>,
    indices: Vec<u32>,
    rgba: [u8; 4],
}

impl Storage {
    #[allow(clippy::type_complexity)]
    pub fn into_inner(self) -> (Vec<(i16, i16, [u8; 4])>, Vec<u32>) {
        trace!(target: module_path!(), "{} vertices", self.vertices.len());
        (self.vertices, self.indices)
    }
}

#[doc(hidden)]
impl FillGeometryBuilder for Storage {
    #[inline]
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        let id = self.vertices.len() as u32;
        let (x, y) = vertex.position().into();

        self.vertices.push((x as i16, y as i16, self.rgba));
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
