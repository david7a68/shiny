mod common;

use shiny::{
    color::{Rgb, Srgb8},
    image::{cpu_image::CpuImage, Image},
    shapes::{
        bezier::{Bezier, CubicSlice},
        point::Point,
    },
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
        Point::new(50.0, 10.0),
        Point::new(190.0, 190.0),
        Point::new(10.0, 190.0),
        Point::new(150.0, 10.0),
    ];
    let curve = CubicSlice::new(&points);

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
        image.set(p.x.round() as u32, p.y.round() as u32, color);

        t += delta;
    }

    write_png(image.get_pixels(), module_path!());
}
