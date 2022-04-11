mod common;

use shiny::{
    color::{Color, Rgb, Srgb8},
    image::Image,
    pixel_buffer::PixelBuffer,
    shapes::{
        bezier::{Bezier, CubicSlice},
        point::Point,
    },
};

use common::write_png;

fn main() {
    let points = [
        Point::new(50.0, 10.0),
        Point::new(190.0, 190.0),
        Point::new(10.0, 190.0),
        Point::new(150.0, 10.0),
    ];
    let curve = CubicSlice::new(&points);

    let mut image = PixelBuffer::new(200, 200);

    let (left, right) = curve.split(0.0);

    draw_curve(
        &left.borrow(),
        0.0,
        1.0,
        Srgb8 {
            color: Rgb { r: 255, g: 0, b: 0 },
        },
        &mut image,
    );

    draw_curve(
        &right.borrow(),
        0.0,
        1.0,
        Srgb8 {
            color: Rgb { r: 0, g: 255, b: 0 },
        },
        &mut image,
    );

    write_png(image.get_pixels(), module_path!());
}

fn draw_curve<C: Color>(
    curve: &CubicSlice,
    from: f32,
    to: f32,
    color: C,
    image: &mut PixelBuffer<C>,
) {
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
