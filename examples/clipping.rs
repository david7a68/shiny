mod common;

use common::write_png;

use shiny::{
    color::{Rgb, Srgb8},
    image::{cpu_image::CpuImage, Image},
    shapes::{path::Builder, point::Point},
};

fn main() {
    let mut image = CpuImage::new(300, 300);
    image.clear(Srgb8 {
        color: Rgb { r: 0, g: 0, b: 0 },
    });

    let path1 = {
        let mut builder = Builder::new(Point::new(24.0, 21.0));
        builder.add_cubic(Point::new(189.0, 40.0), Point::new(159.0, 137.0), Point::new(101.0, 261.0));
        builder.build()
    };
    let curve1 = path1.iter().next().unwrap().next().unwrap();

    let path2 = {
        let mut builder = Builder::new(Point::new(18.0, 122.0));
        builder.add_cubic(Point::new(15.0, 178.0), Point::new(247.0, 173.0), Point::new(251.0, 242.0));
        builder.build()
    };
    let curve2 = path2.iter().next().unwrap().next().unwrap();

    // test for intersection

    write_png(image.get_pixels(), module_path!());
}
