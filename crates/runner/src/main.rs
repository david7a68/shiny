use std::fs::File;

use image::{
    color::{Rgb, Srgb8},
    image::{CpuImage, Image},
};
use storage::png::export_png;

fn main() {
    let mut image = CpuImage::new(200, 200);

    for px in image.raw_mut().pixels_mut() {
        *px = Srgb8 {
            color: Rgb {
                r: 255u8,
                g: 255u8,
                b: 255u8,
            },
        };
    }

    let mut file = File::create("sample.png").unwrap();
    export_png(image.get_pixels(), &mut file);
}
