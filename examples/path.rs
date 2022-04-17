mod common;

use common::write_png;

use shiny::{
    color::{Color, Space as ColorSpace},
    image::{Image, PixelFormat},
    pixel_buffer::PixelBuffer,
    shapes::{bezier::Bezier, path::Builder, point::Point},
};

fn main() {
    let path = {
        let mut builder = Builder::default();
        builder.move_to(Point::new(0.0, 100.0));
        builder
            .add_cubic(
                Point::new(10.0, 50.0),
                Point::new(100.0, 100.0),
                Point::new(100.0, 0.0),
            )
            .unwrap();
        builder
            .add_cubic(
                Point::new(200.0, 50.0),
                Point::new(150.0, 0.0),
                Point::new(200.0, 100.0),
            )
            .unwrap();
        builder
            .add_cubic(
                Point::new(180.0, 135.0),
                Point::new(135.0, 180.0),
                Point::new(100.0, 200.0),
            )
            .unwrap();
        builder
            .add_cubic(
                Point::new(50.0, 150.0),
                Point::new(50.0, 150.0),
                Point::new(0.0, 100.0),
            )
            .unwrap();
        builder.build().unwrap()
    };

    let mut image = PixelBuffer::new(200, 200, PixelFormat::Rgba8, ColorSpace::Srgb).unwrap();

    let color = Color::auto(80.0, 0.8, 0.9, 1.0);

    for segment in path.iter() {
        for curve in segment {
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
        }
    }

    write_png(image.get_pixels(), module_path!());
}
