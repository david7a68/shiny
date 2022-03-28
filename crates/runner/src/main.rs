use std::fs::File;

use image::{
    color::{Rgb, Srgb8},
    image::{CpuImage, Image},
};
use math::{bezier::CubicBezier, point::Point};
use storage::png::export_png;

fn main() {
    let mut image = CpuImage::new(200, 200);
    image.clear(Srgb8 {
        color: Rgb {
            r: 0u8,
            g: 0u8,
            b: 0u8,
        },
    });

    let curve = CubicBezier {
        p0: Point(10.0, 10.0),
        p1: Point(10.0, 150.0),
        p2: Point(150.0, 10.0),
        p3: Point(150.0, 150.0),
    };

    let color = Srgb8 {
        color: Rgb {
            r: 255u8,
            g: 255u8,
            b: 255u8,
        },
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

    let mut file = File::create("sample.png").unwrap();
    export_png(image.get_pixels(), &mut file);
}
