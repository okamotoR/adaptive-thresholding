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
use imageproc::integral_image::integral_squared_image;
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
pub fn raw_img_to_gray_vec(raw_data: Vec<u8>) -> Vec<u8> {
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

pub fn normalize_gray_image(gray_image: DynamicImage) -> DynamicImage {
    return gray_image;
}

// generate image without line
// ￣￣￣\/\/￣￣￣ => average:￣￣￣----￣￣￣ + deviation:_____----_____ = base:￣￣￣￣￣￣￣￣
pub fn generate_base_paper_image_vec(gray_image_vec: Vec<u8>, width: u32, height :u32, radius: u32) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let gray_image = ImageBuffer::<image::Luma<u8>, Vec<_>>
        ::from_vec(width, height, gray_image_vec).unwrap();
    let integral_image = integral_image::<_, u8>(&gray_image);// (width+1,height+1)
    let integral_squared_image = integral_squared_image::<_, u8>(&gray_image);// (width+1,height+1)

    let mut base_paper_image_vec = Vec::new();
    for y in 1..height {
        for x in 1..width {
            let start_x:u32 = if x > radius {
                    x - radius
                } else {
                    0
                };
            let start_y:u32 = if y > radius {
                    y - radius
                } else {
                    0
                };
            let end_x:u32 = if x + radius < width {
                    x + radius
                } else {
                    width
                };
            let end_y:u32 = if y + radius < height {
                    y + radius
                } else {
                    height
                };

            let partial_sum:u32 = integral_image.get_pixel(end_x, end_y)[0] as u32
                - integral_image.get_pixel(end_x, start_y)[0] as u32
                - integral_image.get_pixel(start_x, end_y)[0] as u32
                + integral_image.get_pixel(start_x, start_y)[0] as u32;
            let partial_sq_sum:u32 = integral_squared_image.get_pixel(end_x, end_y)[0] as u32
                - integral_squared_image.get_pixel(end_x, start_y)[0] as u32
                - integral_squared_image.get_pixel(start_x, end_y)[0] as u32
                + integral_squared_image.get_pixel(start_x, start_y)[0] as u32;
            let pixel_sum:u32 = (end_x - start_x) * (end_y - start_y);

            let average:f32 = partial_sum as f32 / pixel_sum as f32;
            let deviation:f32 = (partial_sq_sum as f32 / pixel_sum as f32 - (partial_sum * partial_sum) as f32).sqrt();

            let base:u8 = (average + deviation) as u8;
            base_paper_image_vec.push(base);
        }
    }
    return base_paper_image_vec;
}

pub fn extract_line(gray_image: DynamicImage, threshold_level: u8) -> DynamicImage {
    return gray_image;
}





#[wasm_bindgen]
pub fn convert(image_vec: Vec<u8>, width: u32, height :u32) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let image = ImageBuffer::<image::Luma<u8>, Vec<_>>
        ::from_vec(width, height, image_vec).unwrap();

    let integral_image = integral_image::<_, u8>(&image);
    return integral_image.into_vec();
}



