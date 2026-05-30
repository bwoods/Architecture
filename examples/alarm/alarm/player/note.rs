use crate::nord::Nord;
use composable_views::{Output, Path, Transform};

#[derive(Clone)]
pub struct Note;

impl Path for Note {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        icon(x, y, w, h, Nord.0, onto)
    }
}

#[rustfmt::skip]
fn icon(x: f32, y: f32, w: f32, h: f32, rgba: [u8; 4], output: &mut impl Output) {
    let transform = Transform::translation(x, y)
        .pre_scale(w / 128.0, h / 128.0);

    output.begin(rgba, &transform);
    output.move_to(65.259735, 83.172264);
    output.cubic_bezier_to(64.10102, 82.38143, 62.584747, 81.877655, 60.808857, 81.703896);
    output.cubic_bezier_to(57.03509, 81.33347, 52.01472, 82.43432, 47.17792, 85.01888);
    output.cubic_bezier_to(42.31627, 87.62102, 38.76416, 91.130135, 37.21504, 94.35925);
    output.cubic_bezier_to(36.02379, 96.837975, 35.97589, 99.19339, 37.124622, 101.037544);
    output.cubic_bezier_to(38.245476, 102.835846, 40.419235, 103.95712, 43.260838, 104.23424);
    output.cubic_bezier_to(47.0346, 104.60425, 52.054974, 103.50341, 56.891773, 100.91883);
    output.cubic_bezier_to(61.75385, 98.316696, 65.30553, 94.80801, 66.85465, 91.57845);
    output.cubic_bezier_to(67.45423, 90.33141, 67.76423, 89.11467, 67.77673, 87.98134);
    output.line_to(67.779236, 38.5424);
    output.cubic_bezier_to(69.814644, 40.635735, 73.38425, 43.66112, 79.28644, 46.92619);
    output.cubic_bezier_to(88.30723, 51.911148, 88.19353, 58.224323, 88.19353, 58.224323);
    output.cubic_bezier_to(88.19353, 58.241825, 88.191025, 58.259323, 88.191025, 58.274742);
    output.cubic_bezier_to(88.191025, 66.81682, 84.06228, 70.07421, 83.53854, 70.451805);
    output.line_to(83.48312, 70.491806);
    output.cubic_bezier_to(82.89355, 70.86223, 82.71729, 71.63805, 83.087715, 72.22759);
    output.cubic_bezier_to(83.455215, 72.81675, 84.233955, 72.99342, 84.82318, 72.62551);
    output.line_to(85.08777, 72.44176);
    output.cubic_bezier_to(86.34483, 71.52218, 91.702805, 66.94256, 91.71027, 55.71536);
    output.cubic_bezier_to(91.72319, 55.224102, 91.82361, 46.50832, 81.15315, 39.75482);
    output.cubic_bezier_to(70.6211, 33.092316, 67.71422, 24.554823, 67.71422, 24.554823);
    output.cubic_bezier_to(67.51755, 23.970236, 66.925896, 23.612326, 66.315926, 23.71316);
    output.cubic_bezier_to(65.70884, 23.811493, 65.26051, 24.338161, 65.26051, 24.955292);
    output.close();
}
