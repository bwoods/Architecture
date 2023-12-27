//! SVG [`Output`] for `Views`

use svg::node::element::path::{Command, Position};
use svg::{node::element::path::Data, node::element::Path, Document, Node};

use crate::Transform;

///
pub struct Output {
    svg: Document,
    transform: Transform,
    data: Data,
    rgba: [u8; 4],
}

impl Output {
    /// Creates a Scalable Vector Graphics `Output`.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            svg: Document::new().set("viewBox", (0, 0, width, height)),
            transform: Default::default(),
            data: Default::default(),
            rgba: [0; 4],
        }
    }

    fn end_current_node(&mut self) {
        let data = std::mem::take(&mut self.data);

        let fill = format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
        );

        let array = self.transform.to_array();
        let transform = format!(
            "matrix({} {} {} {} {} {})",
            array[0], array[1], array[2], array[3], array[4], array[5]
        );

        self.svg.append(
            Path::new()
                .set("transform", transform)
                .set("fill", fill)
                .set("d", data),
        );
    }

    /// Consumes the `Output` and returns the constructed SVG string.
    pub fn into_inner(mut self) -> String {
        self.end_current_node();
        self.svg.to_string()
    }
}

impl super::Output for Output {
    fn begin(&mut self, x: f32, y: f32, rgba: [u8; 4], transform: &Transform) {
        if !self.data.is_empty() && (rgba != self.rgba || !transform.approx_eq(&self.transform)) {
            self.end_current_node();
        }

        self.rgba = rgba;
        self.transform = *transform;

        self.data
            .append(Command::Move(Position::Absolute, (x, y).into()));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.data
            .append(Command::Line(Position::Absolute, (x, y).into()));
    }

    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.data.append(Command::QuadraticCurve(
            Position::Absolute,
            (x1, y1, x, y).into(),
        ));
    }

    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.data.append(Command::CubicCurve(
            Position::Absolute,
            (x1, y1, x2, y2, x, y).into(),
        ));
    }

    fn close(&mut self) {
        self.data.append(Command::Close);
    }
}
