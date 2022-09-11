use std::io::Cursor;
use image::{DynamicImage, GenericImageView};
use image::io::Reader as ImgReader;

fn main() {
    let img = ImgReader::open("/home/kristian/src/pixelprinterrs/resources/harnverhalt4.png")
        .unwrap()
        .decode()
        .unwrap();
    print_pixel_info(img)
}

fn print_pixel_info(img: DynamicImage) {
    img.pixels().for_each(|px| {
        let red = px.2.0[0];
        let green = px.2.0[1];
        let blue = px.2.0[2];
        let alpha = px.2.0[3];
        println!("R: {}, G: {}, B: {}, A: {}", red, green, blue, alpha)
    })
}
