extern crate wasm_bindgen;
extern crate image;
extern crate imageproc;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::io::Cursor;
use std::panic;
use image::ImageFormat;
use image::ImageBuffer;
use image::DynamicImage;
use image::io::Reader;
use imageproc::integral_image::integral_image;
use imageproc::gray_image;
use imageproc::definitions::Image;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn raw_img_to_vec(raw_data: Vec<u8>) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let image = decode_raw_data(raw_data);
    let gray_image = image.grayscale();
    let gray_vec = gray_image.to_luma8().into_vec();
    return gray_vec;
}

pub fn decode_raw_data(raw_data: Vec<u8>) -> DynamicImage {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let reader = Reader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .expect("Cursor io never fails");

    return reader.decode().unwrap();
}

#[wasm_bindgen]
pub fn convert(image_vec: Vec<u8>, width: u32, height :u32) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let image = ImageBuffer::<image::Luma<u8>, Vec<_>>
        ::from_vec(width, height, image_vec).unwrap();

    let integral_image = integral_image::<_, u8>(&image);
    return integral_image.into_vec();
}



