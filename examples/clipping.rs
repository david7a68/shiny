mod common;

use common::write_png;

use shiny::{
    color::{Color, Rgb, Srgb8},
    image::image::{CpuImage, Image},
    math::{point::Point},
    shapes::curve::clipping::clip,
    shapes::{curve::Bezier, path::PathBuilder},
};

fn main() {
    let mut image = CpuImage::new(300, 300);
    image.clear(Srgb8 {
        color: Rgb { r: 0, g: 0, b: 0 },
    });

    let path1 = {
        let mut builder = PathBuilder::new(Point(24.0, 21.0));
        builder.add_cubic(Point(189.0, 40.0), Point(159.0, 137.0), Point(101.0, 261.0));
        builder.build()
    };
    let curve1 = path1.iter().next().unwrap().next().unwrap();

    let path2 = {
        let mut builder = PathBuilder::new(Point(18.0, 122.0));
        builder.add_cubic(Point(15.0, 178.0), Point(247.0, 173.0), Point(251.0, 242.0));
        builder.build()
    };
    let curve2 = path2.iter().next().unwrap().next().unwrap();

    // test for intersection

    write_png(image.get_pixels(), module_path!());
}
