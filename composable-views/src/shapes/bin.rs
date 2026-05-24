use std::fmt::Display;
use std::io::{Read, StdoutLock, Write, stdout};
use std::ops::Add;

use clap::{ArgGroup, Parser};
use convert_case::{Case, Casing};
use usvg::tiny_skia_path::PathSegment;
use usvg::{Node, Tree};

#[derive(Parser)]
#[clap(about, author, version)]
#[command(group(ArgGroup::new("resize").args(["scale", "width", "height"])))]
struct Args {
    /// path to the SVG (or SGZ) file
    path: std::path::PathBuf,

    /// scale the resulting paths by this amount
    #[arg(short, long)]
    scale: Option<f32>,

    /// scale the resulting paths to match this width
    #[arg(short, long)]
    width: Option<f32>,

    /// scale the resulting paths to match this height
    #[arg(long)]
    height: Option<f32>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut data = Vec::new();
    let mut file = std::fs::File::open(&args.path).map_err(|err| err_msg(err, &args.path))?;
    file.read_to_end(&mut data)
        .map_err(|err| err_msg(err, &args.path))?;

    let name = args
        .path
        .as_path()
        .file_stem()
        .ok_or(err_msg("no filename?", &args.path))?
        .to_string_lossy()
        .to_case(Case::Snake);

    let resize = match (args.scale, args.width, args.height) {
        (Some(s), None, None) => Resize::Scale(s),
        (None, Some(w), None) => Resize::Width(w),
        (None, None, Some(h)) => Resize::Height(h),
        _ => Resize::Scale(1.0),
    };

    let options = usvg::Options::default();
    let svg = Tree::from_data(&data, &options).map_err(|err| err_msg(err, &args.path))?;
    svg_paths(&name, resize, svg).map_err(|err| err_msg(err, &args.path))
}

enum Resize {
    Scale(f32),
    Width(f32),
    Height(f32),
}

#[rustfmt::skip]
fn svg_paths(name: &str, resize: Resize, svg: Tree) -> std::io::Result<()> {
    let size = svg.size();

    let scale = match resize {
        Resize::Scale(scale) => scale,
        Resize::Width(width) => width / size.width(),
        Resize::Height(height) => height / size.height(),
    };

    let mut lock = stdout().lock();
    writeln!(lock, "#[rustfmt::skip]")?;
    writeln!(lock, "fn {name}(x: f32, y: f32, w: f32, h: f32, rgba: [u8; 4], output: &mut impl Output) {{")?;
    writeln!(lock, "    let transform = Transform::translation(x, y)")?;
    writeln!(lock, "        .pre_scale(w / {:?}, h / {:?});", size.width() * scale, size.width() * scale)?;
    writeln!(lock)?;

    writeln!(lock, "    output.begin(rgba, &transform);")?;
    recurse(scale, &mut lock, svg.root().children())?;
    writeln!(lock, "}}")
}

#[rustfmt::skip]
fn recurse(scale: f32, lock: &mut StdoutLock, nodes: &[Node]) -> std::io::Result<()> {
    for element in nodes {
        let mut transform = element.abs_transform();
        transform = transform.post_scale(scale, scale);

        match element {
            Node::Group(group) => recurse(scale, lock, group.children())?,
            Node::Path(path) => {
                for segment in path.data().segments() {
                    match segment {
                        PathSegment::MoveTo(mut p) => {
                            transform.map_point(&mut p);
                            writeln!(lock, "    output.move_to({:?}, {:?});", p.x, p.y)?;
                        }
                        PathSegment::LineTo(mut p) => {
                            transform.map_point(&mut p);
                            writeln!(lock, "    output.line_to({:?}, {:?});", p.x, p.y)?;
                        }
                        PathSegment::QuadTo(mut p1, mut p) => {
                            transform.map_point(&mut p);
                            transform.map_point(&mut p1);
                            writeln!(lock, "    output.quadratic_bezier_to({:?}, {:?}, {:?}, {:?});",p1.x, p1.y, p.x, p.y)?;
                        }
                        PathSegment::CubicTo(mut p1, mut p2, mut p) => {
                            transform.map_point(&mut p);
                            transform.map_point(&mut p1);
                            transform.map_point(&mut p2);
                            writeln!(lock, "    output.cubic_bezier_to({:?}, {:?}, {:?}, {:?}, {:?}, {:?});",p1.x, p1.y, p2.x, p2.y, p.x, p.y)?;
                        }
                        PathSegment::Close => {
                            writeln!(lock, "    output.close();")?;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn err_msg<E: Display>(error: E, path: &std::path::Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
        .add(": ")
        .add(&error.to_string())
}
