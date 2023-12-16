//! SVG [`Output`] for `Views`

use svg::{node::element::path::Data, node::element::Path, Document, Node};

///
pub struct Output {
    svg: Document,
    data: Option<Data>,
    rgba: [u8; 4],
}

impl Output {
    /// Creates a Scalable Vector Graphics `Output`.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            svg: Document::new().set("viewBox", (0, 0, width, height)),
            data: None,
            rgba: [0; 4],
        }
    }

    /// Consumes the `Output` and returns the constructed SVG string.
    pub fn into_inner(self) -> String {
        self.svg.to_string()
    }
}

impl super::Output for Output {
    fn move_to(&mut self, x: f32, y: f32, rgba: [u8; 4]) {
        self.data = Some(Data::new().move_to((x, y)));
        self.rgba = rgba;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.data = self.data.take().map(|data| data.line_to((x, y)));
    }

    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.data = self
            .data
            .take()
            .map(|data| data.quadratic_curve_to((x1, y1, x, y)));
    }

    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.data = self
            .data
            .take()
            .map(|data| data.cubic_curve_to((x1, y1, x2, y2, x, y)));
    }

    fn close(&mut self) {
        if let Some(data) = self.data.take() {
            let fill = format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
            );

            self.svg
                .append(Path::new().set("fill", fill).set("d", data.close()));
        }
    }
}
