mod common;

use common::write_png;

use shiny::{
    color::{Color, Rgb, Srgb8},
    image::{cpu_image::CpuImage, Image},
    shapes::{
        bezier::{intersections, Bezier, CubicSlice},
        path::Builder,
        point::Point,
    },
};

fn main() {
    let mut image = CpuImage::new(300, 300);

    let path1 = {
        let mut builder = Builder::new(Point::new(24.0, 21.0));
        builder.add_cubic(
            Point::new(189.0, 40.0),
            Point::new(159.0, 137.0),
            Point::new(101.0, 261.0),
        );
        builder.build()
    };
    let curve1 = path1.iter().next().unwrap().next().unwrap();

    let path2 = {
        let mut builder = Builder::new(Point::new(18.0, 122.0));
        builder.add_cubic(
            Point::new(15.0, 178.0),
            Point::new(247.0, 173.0),
            Point::new(251.0, 242.0),
        );
        builder.build()
    };
    let curve2 = path2.iter().next().unwrap().next().unwrap();

    draw_curve(
        &curve1,
        0.0,
        1.0,
        Srgb8 {
            color: Rgb { r: 255, g: 0, b: 0 },
        },
        &mut image,
    );

    draw_curve(
        &curve2,
        0.0,
        1.0,
        Srgb8 {
            color: Rgb { r: 0, g: 255, b: 0 },
        },
        &mut image,
    );

    let intersections = intersections(&curve1.points, &curve2.points);
    println!("{:?}", &intersections.0);

    for t in intersections.0.iter() {
        let points = curve1.at(*t);
        image.set(
            points.x.round() as u32,
            points.y.round() as u32,
            Srgb8 {
                color: Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            },
        );
    }

    write_png(image.get_pixels(), module_path!());
}

fn draw_curve<C: Color>(curve: &CubicSlice, from: f32, to: f32, color: C, image: &mut CpuImage<C>) {
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
