mod common;

use shiny::{
    color::{Color, Space as ColorSpace},
    image::{Image, PixelFormat},
    math::vector::Vec2,
    pixel_buffer::PixelBuffer,
    shapes::{
        bezier::{Bezier, CubicSlice},
        point::Point,
    },
};

use common::write_png;

fn main() {
    let mut image = PixelBuffer::new(500, 500, PixelFormat::Rgba8, ColorSpace::Srgb).unwrap();
    image.clear(Color::BLACK);

    // let points = [
    //     Point::new(50.0, 10.0),
    //     Point::new(190.0, 190.0),
    //     Point::new(10.0, 190.0),
    //     Point::new(150.0, 10.0),
    // ];
    let mut points = [
        Point::new(78.17871, -45.604248),
        Point::new(3004.715, 1307.1124),
        Point::new(2961.2825, 1202.874),
        Point::new(2917.8499, 1202.874),
    ];
    
    for p in &mut points {
        *p = (Vec2::new(100.0, 100.0) + p.vec() * 0.1).into();
    }

    let curve = CubicSlice::new(&points);

    for p in &points {
        // println!("{:?}", p);
        image.set(p.x as u32, p.y as u32, Color::GREEN);
    }

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

    if let Some((a, b)) = curve.find_self_intersection() {
        let p1 = curve.at(a);
        let p2 = curve.at(b);

        println!("{:?}", p1);
        println!("{:?}", p2);
        image.set(p1.x as u32, p1.y as u32, Color::GREEN);
    } else {
        println!("No self intersection");
    }

    let bounds = curve.coarse_bounds();
    image.set(bounds.left() as u32, bounds.top() as u32, Color::BLUE);
    image.set(bounds.right() as u32, bounds.top() as u32, Color::BLUE);
    image.set(bounds.right() as u32, bounds.bottom() as u32, Color::BLUE);
    image.set(bounds.left() as u32, bounds.bottom() as u32, Color::BLUE);

    write_png(image.get_pixels(), module_path!());
}
