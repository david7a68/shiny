mod common;
use common::*;

use shiny::{shapes::{
    path::{Builder as PathBuilder, Path},
    point::Point, bezier::Bezier,
}, image::{cpu_image::CpuImage, Image}, color::{Srgb8, Rgb}, math::vector::Vec2};

fn main() {
    // let file = std::fs::read_to_string("./test_files/tiger.svg").unwrap();
    let file = std::fs::read_to_string("./test_files/car.svg").unwrap();

    let paths = read_svg(&file);

    let mut image = CpuImage::new(4000, 2000);
    image.clear(Srgb8 {
        color: Rgb { r: 0, g: 0, b: 0 },
    });

    let color = Srgb8 {
        color: Rgb {
            r: 100,
            g: 200,
            b: 239,
        },
    };

    for path in paths {
        for segment in path.iter() {
            for curve in segment {
                let mut t = 0.0;
                let delta = 0.0001;
                loop {
                    if t >= 1.0 {
                        break;
                    }
    
                    let p = curve.at(t) + Vec2::new(100.0, 100.0);
                    image.set(p.x.round() as u32, p.y.round() as u32, color);
                    t += delta;
                }
            }
        }
    }

    write_png(image.get_pixels(), module_path!());
}

fn read_svg(data: &str) -> Vec<Path> {
    let dom = roxmltree::Document::parse(data).unwrap();
    let svg = dom.descendants().filter(|n| n.tag_name().name() == "svg");

    let mut paths = vec![];
    let mut num_paths = 0;
    let mut num_segments = 0;

    // for each svg element
    for node in svg {
        // extract only path information
        for p in node.descendants().filter(|n| n.tag_name().name() == "path") {
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
                        path.line_to(Point::new(4.0 * x as f32, 4.0 * y as f32)).unwrap();
                    }
                    svgtypes::PathSegment::HorizontalLineTo { abs, x } => {
                        if let Some(cursor) = path.cursor() {
                            path.line_to(Point::new(4.0 * x as f32, 4.0 * cursor.y as f32)).unwrap();
                        } else {
                            // Bad Path... skip this path.
                            println!("Bad Path (horizontal)");
                            continue;
                        }
                    }
                    svgtypes::PathSegment::VerticalLineTo { abs, y } => {
                        if let Some(cursor) = path.cursor() {
                            path.line_to(Point::new(4.0 * cursor.x as f32, 4.0 * y as f32)).unwrap();
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
                        ).unwrap();
                    }
                    svgtypes::PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        println!("smooth cubic");
                    }
                    svgtypes::PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        println!("quadratic");
                    }
                    svgtypes::PathSegment::SmoothQuadratic { abs, x, y } => {
                        println!("smooth quadratic");
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
                    }
                    svgtypes::PathSegment::ClosePath { abs } => {
                        path.close().unwrap();
                    }
                }
            }
            
            paths.push(path.build());
        }
    }

    println!(
        "num_paths (expected): {}, num_paths (reported): {}, num_segments: {}, avg segments/path: {:.4}",
        num_paths,
        paths.len(),
        num_segments,
        num_segments as f32 / num_paths as f32
    );
    paths
}
