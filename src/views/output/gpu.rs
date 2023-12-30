//! GPU [`Output`] for `Views`
use std::cell::Cell;
use std::rc::Rc;

use lyon::path::builder::{NoAttributes, Transformed};
use lyon::tessellation::{
    FillBuilder, FillGeometryBuilder, FillOptions, FillTessellator, FillVertex, GeometryBuilder,
    GeometryBuilderError, VertexId,
};

use crate::views::Transform;

///
pub struct Output<'a> {
    builder: NoAttributes<Transformed<FillBuilder<'a>, Transform>>,
    rgba: Rc<Cell<[u8; 4]>>,
}

impl<'a> Output<'a> {
    /// Creates an indexed-triangle data `Output`.
    pub fn new(
        options: &'a FillOptions,
        tessellator: &'a mut FillTessellator,
        storage: &'a mut Storage,
    ) -> Self {
        let transform = Transform::default();
        let rgba = Rc::new(Cell::default());
        storage.rgba = Rc::clone(&rgba);

        Self {
            builder: tessellator.builder(options, storage).transformed(transform),
            rgba,
        }
    }

    /// `Output` has multiple dependencies with lifetime constraints. Use [`Output::defaults`]
    /// to create these dependencies, then pass them directly into [`Output::new`] for easy
    /// construction.
    ///
    /// ```
    /// # use composable::views::gpu::Output;
    /// let (mut options, mut tessellator, mut storage) = Output::defaults();
    /// let output = Output::new(&options, &mut tessellator, &mut storage);
    ///
    /// // …
    ///
    /// let (vertices, indices) = storage.into_inner();
    /// ```
    /// Once all the views have been drawn [`Storage::into_inner`] can be used to retrieved the
    /// indexed-triangle data.
    pub fn defaults() -> (FillOptions, FillTessellator, Storage) {
        let options = FillOptions::default();
        let tessellator = FillTessellator::default();
        let storage = Storage::default();

        (options, tessellator, storage)
    }
}

impl super::Output for Output<'_> {
    #[inline]
    fn begin(&mut self, x: f32, y: f32, rgba: [u8; 4], transform: &Transform) {
        self.rgba.set(rgba);
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

    #[inline]
    fn close(&mut self) {
        self.builder.close();
    }
}

///
#[derive(Default)]
pub struct Storage {
    vertices: Vec<(f32, f32, u32)>,
    indices: Vec<u32>,
    rgba: Rc<Cell<[u8; 4]>>,
}

impl Storage {
    /// Consumes the `Output` and returns the constructed indexed-triangle data.
    /// - vertices are stored as (x, y, rgba) tuples
    /// - indices are stored as 32-bit offsets
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
        self.vertices
            .push((x, y, u32::from_be_bytes(self.rgba.get())));

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
