mod common;

use common::write_png;

use shiny::{
    color::{Rgb, Srgb8},
    image::image::{CpuImage, Image},
    math::{point::Point, bezier::Bezier},
    shapes::path::PathBuilder,
};

fn main() {
    let path = {
        let mut builder = PathBuilder::new(Point(0.0, 100.0));
        builder.add_cubic(Point(10.0, 50.0), Point(100.0, 100.0), Point(100.0, 0.0));
        builder.add_cubic(Point(200.0, 50.0), Point(150.0, 0.0), Point(200.0, 100.0));
        builder.add_cubic(
            Point(180.0, 135.0),
            Point(135.0, 180.0),
            Point(100.0, 200.0),
        );
        builder.add_cubic(Point(50.0, 150.0), Point(50.0, 150.0), Point(0.0, 100.0));
        builder.build()
    };

    let mut image = CpuImage::new(200, 200);
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

    for segment in path.iter() {
        for curve in segment {
            let mut t = 0.0;
            let delta = 0.001;
            loop {
                if t >= 1.0 {
                    break;
                }

                let p = curve.at(t);
                image.set(p.x().round() as u32, p.y().round() as u32, color);
                t += delta;
            }
        }
    }

    write_png(image.get_pixels(), module_path!());
}
