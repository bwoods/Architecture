//! SVG [`Output`] for `Views`

use svg::node::element::path::{Command, Position};
use svg::{node::element::path::Data, node::element::Path, Document, Node};

///
pub struct Output {
    svg: Document,
    data: Data,
    rgba: [u8; 4],
}

impl Output {
    /// Creates a Scalable Vector Graphics `Output`.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            svg: Document::new().set("viewBox", (0, 0, width, height)),
            data: Data::new(),
            rgba: [0; 4],
        }
    }

    fn append_path_if_needed(&mut self) {
        let data = std::mem::take(&mut self.data);

        let fill = format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
        );

        self.svg
            .append(Path::new().set("fill", fill).set("d", data));
    }

    /// Consumes the `Output` and returns the constructed SVG string.
    pub fn into_inner(mut self) -> String {
        self.append_path_if_needed();
        self.svg.to_string()
    }
}

impl super::Output for Output {
    fn begin(&mut self, x: f32, y: f32, rgba: [u8; 4]) {
        if rgba != self.rgba && !self.data.is_empty() {
            self.append_path_if_needed();
        }

        self.rgba = rgba;
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
