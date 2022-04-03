mod common;

use common::write_png;

use shiny::{
    color::{Color, Rgb, Srgb8},
    image::{cpu_image::CpuImage, Image},
    shapes::{
        bezier::{Bezier, Cubic, CubicSlice},
        point::Point,
    },
};

fn main() {
    let curve1 = Cubic {
        points: [
            Point::new(18.0, 122.0),
            Point::new(15.0, 178.0),
            Point::new(247.0, 173.0),
            Point::new(251.0, 242.0),
        ],
    };

    let curve2 = Cubic {
        points: [
            Point::new(20.0, 213.0),
            Point::new(189.0, 40.0),
            Point::new(85.0, 283.0),
            Point::new(271.0, 217.0),
        ],
    };

    let mut image = CpuImage::new(300, 300);
    draw_curve(
        curve1.borrow(),
        0.0,
        1.0,
        Srgb8 {
            color: Rgb {
                r: 100,
                g: 100,
                b: 100,
            },
        },
        &mut image,
    );

    draw_curve(
        curve2.borrow(),
        0.0,
        1.0,
        Srgb8 {
            color: Rgb {
                r: 100,
                g: 100,
                b: 100,
            },
        },
        &mut image,
    );

    let intersections = curve1.intersections(&curve2);
    println!("{:?}", intersections);

    for t in intersections.iter() {
        let intersection = curve1.at(t.0);
        println!("{:?}", intersection);
        image.set(
            intersection.x.round() as u32,
            intersection.y.round() as u32,
            Srgb8 {
                color: Rgb { r: 255, g: 0, b: 0 },
            },
        );
    }

    write_png(image.get_pixels(), module_path!());
}

fn draw_curve<C: Color>(curve: CubicSlice, from: f32, to: f32, color: C, image: &mut CpuImage<C>) {
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
