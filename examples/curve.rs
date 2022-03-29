mod common;

use shiny::{
    color::{Rgb, Srgb8},
    image::image::{CpuImage, Image},
    math::{bezier::{CubicBezierSlice, Bezier}, point::Point},
};

use common::write_png;

fn main() {
    let mut image = CpuImage::new(200, 200);
    image.clear(Srgb8 {
        color: Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
    });

    let points = [
        Point(50.0, 10.0),
        Point(190.0, 190.0),
        Point(10.0, 190.0),
        Point(150.0, 10.0),
    ];
    let curve = CubicBezierSlice::new(&points);

    let color = Srgb8 {
        color: Rgb { r: 255, g: 0, b: 0 },
    };

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

    write_png(image.get_pixels(), module_path!());
}
