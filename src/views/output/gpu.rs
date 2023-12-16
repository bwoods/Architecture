//! GPU `Output` for `Views`
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use lyon::tessellation::{StrokeOptions, StrokeTessellator, StrokeVertex};
use lyon::{path::builder::NoAttributes, path::path::BuilderImpl, path::Path};

///
pub struct Output {
    builder: NoAttributes<BuilderImpl>,
}

impl Output {
    /// Creates an indexed-triangle data `Output`.
    #[allow(unused_variables)]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            builder: Path::builder(),
        }
    }

    /// Consumes the `Output`` and returns the constructed geometry
    /// - vertexes are stored as (x, y, rgba) tuples
    /// - indicies are stored as 32-bit offsets
    pub fn build(self) -> (Vec<(f32, f32, u32)>, Vec<u32>) {
        let mut geometry: VertexBuffers<(f32, f32, u32), u32> = VertexBuffers::new();

        let paths = self.builder.build();
        FillTessellator::new()
            .tessellate_path(
                &paths,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |v: FillVertex| {
                    let (x, y) = v.position().into();
                    (x, y, 0u32) // #black
                }),
            )
            .unwrap();

        if cfg!(stroke) {
            // ensure stroke tessellation compiles even though it is not used (yet?)
            StrokeTessellator::new()
                .tessellate_path(
                    &paths,
                    &StrokeOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |v: StrokeVertex| {
                        let (x, y) = v.position().into();
                        (x, y, 0u32) // #black
                    }),
                )
                .unwrap();
        }

        (geometry.vertices, geometry.indices)
    }
}

impl super::Output for Output {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin((x, y).into());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to((x, y).into());
    }

    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder
            .quadratic_bezier_to((x1, y1).into(), (x, y).into());
    }

    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder
            .cubic_bezier_to((x1, y1).into(), (x2, y2).into(), (x, y).into());
    }

    fn close(&mut self) {
        self.builder.close();
    }
}
