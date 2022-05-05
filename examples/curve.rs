mod common;

use shiny::{
    color::{Color, Space as ColorSpace},
    image::{Image, PixelFormat},
    math::vector2::Vec2,
    pixel_buffer::PixelBuffer,
    shapes::{
        bezier::{Bezier, CubicSlice},
        point::Point,
    },
};

use common::write_png;

fn main() {
    let mut image = PixelBuffer::new(500, 500, PixelFormat::Rgba8, ColorSpace::LinearSrgb).unwrap();
    image.clear(Color::BLACK);

    // let points = [
    //     Point::new(50.0, 10.0),
    //     Point::new(190.0, 190.0),
    //     Point::new(10.0, 190.0),
    //     Point::new(150.0, 10.0),
    // ];

    let mut x = [78.17871, 3004.715, 2961.2825, 2917.8499,];
    let mut y = [-45.604248, 1307.1124, 1202.874, 1202.874,];

    for x in &mut x {
        *x = 100.0 + *x * 0.1;
    }

    for y in &mut y {
        *y = 100.0 + *y * 0.1;
    }

    let curve = CubicSlice::new(&x, &y);

    let mut t = 0.0;
    let delta = 0.001;
    loop {
        if t >= 1.0 {
            break;
        }

        let p: Point = curve.at(t);
        image.set(p.x.round() as u32, p.y.round() as u32, Color::RED);

        t += delta;
    }

    let bounds = curve.coarse_bounds();
    image.set(bounds.left() as u32, bounds.top() as u32, Color::BLUE);
    image.set(bounds.right() as u32, bounds.top() as u32, Color::BLUE);
    image.set(bounds.right() as u32, bounds.bottom() as u32, Color::BLUE);
    image.set(bounds.left() as u32, bounds.bottom() as u32, Color::BLUE);

    write_png(image.get_pixels(), module_path!());
}
