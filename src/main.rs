use std::any::Any;
use std::fs::File;
use std::io::{Cursor, Write};
use image::{DynamicImage, GenericImage, GenericImageView};
use image::imageops::FilterType;
use image::io::Reader as ImgReader;

fn main() {
    let img = ImgReader::open("/home/kristian/src/pixelprinterrs/resources/harnverhalt4.png")
        .unwrap()
        .decode()
        .unwrap();
    print_pixel_info(img.clone());

    let gaussian = gaussian_pyramid(img.clone(), 5);

    gaussian.into_iter().enumerate().for_each(|(idx, image)| {
        let path = format!("/home/kristian/src/pixelprinterrs/resources/gauss{}.png", idx);
        image.save(path).unwrap()
    })

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

// Imperative solution
fn gaussian_pyramid(img: DynamicImage, iterations: u32) -> Vec<DynamicImage> {
    let mut pyramid: Vec<_> = vec![];

    let mut current = img.clone();

    pyramid.push(img);

    for _ in 0..iterations {
        let processed = gaussian(current);
        current = processed;
        pyramid.push(current.clone())
    }

    pyramid
}

fn gaussian(img: DynamicImage) -> DynamicImage {
    img.resize(img.width() / 2,img.height() / 2, FilterType::Gaussian)
}