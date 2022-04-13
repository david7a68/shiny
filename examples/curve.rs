mod common;

use shiny::{
    color::{Color, Space as ColorSpace},
    image::{Image, PixelFormat},
    pixel_buffer::PixelBuffer,
    shapes::{
        bezier::{Bezier, CubicSlice},
        point::Point,
    },
};

use common::write_png;

fn main() {
    let mut image = PixelBuffer::new(200, 200, PixelFormat::Rgba8, ColorSpace::Srgb).unwrap();
    image.clear(Color::auto(1.0, 1.0, 1.0, 1.0));

    let points = [
        Point::new(50.0, 10.0),
        Point::new(190.0, 190.0),
        Point::new(10.0, 190.0),
        Point::new(150.0, 10.0),
    ];
    let curve = CubicSlice::new(&points);

    let color = Color::auto(1.0, 0.0, 0.0, 1.0);

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
