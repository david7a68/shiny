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
    let x = [50.0, 190.0, 10.0, 150.0];
    let y = [10.0, 190.0, 190.0, 10.0];
    let curve = CubicSlice::new(&x, &y);

    let mut image = PixelBuffer::new(200, 200, PixelFormat::Rgba8, ColorSpace::Srgb).unwrap();
    image.clear(Color::auto(0.0, 0.0, 0.0, 1.0));

    let (left, right) = curve.split(0.5);

    draw_curve(
        &left.as_slice(),
        0.0,
        1.0,
        Color::auto(1.0, 0.0, 0.0, 1.0),
        &mut image,
    );

    draw_curve(
        &right.as_slice(),
        0.0,
        1.0,
        Color::auto(0.0, 1.0, 0.0, 1.0),
        &mut image,
    );

    write_png(image.get_pixels(), module_path!());
}

fn draw_curve(curve: &CubicSlice, from: f32, to: f32, color: Color, image: &mut PixelBuffer) {
    let mut t = from;
    let d = 0.001;
    loop {
        if t >= to {
            break;
        }

        let p = curve.at(t);
        image.set(p.x.round() as u32, p.y.round() as u32, color);

        t += d;
    }
}
