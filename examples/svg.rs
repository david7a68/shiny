mod common;
use common::*;

use shiny::{
    backends::software::Software,
    canvas::{Canvas, CanvasOps, CanvasOptions},
    color::{Color, Space as ColorSpace},
    image::{Image, PixelFormat},
    paint::PaintConfig,
    shapes::{
        path::{Builder as PathBuilder, Path},
        point::Point,
    },
};

fn main() {
    let backend = Software::new();
    let mut canvas = backend
        .new_canvas(
            4000,
            2000,
            PixelFormat::Rgb10a2,
            ColorSpace::LinearSrgb,
            CanvasOptions {
                debug_randomize_color: true,
            },
        )
        .unwrap();
    canvas.clear(Color::BLACK);

    let paint = canvas.create_paint(PaintConfig {
        fill_color: Color::RED,
        stroke_color: Color::GREEN,
    });

    // let file = std::fs::read_to_string("./test_files/tiger.svg").unwrap();
    // let file = std::fs::read_to_string("./test_files/car.svg").unwrap();
    let file = std::fs::read_to_string("./test_files/p1.svg").unwrap();

    let paths = read_svg(&file);
    let start_time = std::time::Instant::now();
    for path in &paths {
        canvas.fill_path(path, paint);
    }
    // canvas.fill_path(&paths[114], paint);
    // canvas.fill_path(&paths[318], paint);
    println!("Render time: {:?}", start_time.elapsed());

    println!("writing images");
    let image = canvas.get_pixels();
    write_png(image.get_pixels(), module_path!());
    let linear = image.convert(PixelFormat::Rgb10a2, ColorSpace::LinearSrgb);
    write_png(linear.get_pixels(), "hahaha");
}

fn read_svg(data: &str) -> Vec<Path> {
    let dom = roxmltree::Document::parse(data).unwrap();
    let svg = dom.descendants().filter(|n| n.tag_name().name() == "svg");

    let mut paths = vec![];
    let mut num_paths = 0;
    let mut num_segments = 0;
    let mut longest_path = 0;
    let mut longest_path_idx = 0;

    // for each svg element
    for node in svg {
        // extract only path information
        'path: for p in node.descendants().filter(|n| n.tag_name().name() == "path") {
            let mut path = PathBuilder::default();

            let d = p.attribute("d").unwrap();

            num_paths += 1;
            for segment in svgtypes::PathParser::from(d) {
                num_segments += 1;
                match segment.unwrap() {
                    svgtypes::PathSegment::MoveTo { abs, x, y } => {
                        path.move_to(Point::new(4.0 * x as f32, 4.0 * y as f32));
                    }
                    svgtypes::PathSegment::LineTo { abs, x, y } => {
                        path.line_to(Point::new(4.0 * x as f32, 4.0 * y as f32))
                            .unwrap();
                    }
                    svgtypes::PathSegment::HorizontalLineTo { abs, x } => {
                        if let Some(cursor) = path.cursor() {
                            path.line_to(Point::new(4.0 * x as f32, 4.0 * cursor.y as f32))
                                .unwrap();
                        } else {
                            // Bad Path... skip this path.
                            println!("Bad Path (horizontal)");
                            continue;
                        }
                    }
                    svgtypes::PathSegment::VerticalLineTo { abs, y } => {
                        if let Some(cursor) = path.cursor() {
                            path.line_to(Point::new(4.0 * cursor.x as f32, 4.0 * y as f32))
                                .unwrap();
                        } else {
                            // Bad Path... skip this path.
                            println!("Bad Path (vertical)");
                            continue;
                        }
                    }
                    svgtypes::PathSegment::CurveTo {
                        abs,
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        path.add_cubic(
                            Point::new(4.0 * x1 as f32, 4.0 * y1 as f32),
                            Point::new(4.0 * x2 as f32, 4.0 * y2 as f32),
                            Point::new(4.0 * x as f32, 4.0 * y as f32),
                        )
                        .unwrap();
                    }
                    svgtypes::PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        println!("smooth cubic");
                        break 'path;
                    }
                    svgtypes::PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        println!("quadratic");
                        break 'path;
                    }
                    svgtypes::PathSegment::SmoothQuadratic { abs, x, y } => {
                        println!("smooth quadratic");
                        break 'path;
                    }
                    svgtypes::PathSegment::EllipticalArc {
                        abs,
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc,
                        sweep,
                        x,
                        y,
                    } => {
                        println!("arc");
                        path.line_to(Point::new(4.0 * x as f32, 4.0 * y as f32))
                            .unwrap();
                        // break 'path;
                    }
                    svgtypes::PathSegment::ClosePath { abs } => {
                        path.close().unwrap();
                    }
                }
            }

            let p = path.build().unwrap();
            if p.x.len() > longest_path {
                longest_path = p.x.len();
                longest_path_idx = paths.len();
            }

            paths.push(p);
        }
    }

    println!(
        "num_paths (expected): {}, num_paths (reported): {}, num_segments: {}, avg segments/path: {:.4}, longest path: {}, longest path idx: {}",
        num_paths,
        paths.len(),
        num_segments,
        num_segments as f32 / num_paths as f32,
        longest_path,
        longest_path_idx
    );
    paths
}
