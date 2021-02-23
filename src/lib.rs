extern crate wasm_bindgen;
extern crate image;
extern crate imageproc;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::io::Cursor;
use std::panic;
use image::ImageBuffer;
use image::DynamicImage;
use image::io::Reader;
use imageproc::integral_image::{integral_image, integral_squared_image, sum_image_pixels, variance};


#[test]
fn normalize_gray_image_test1() {
    assert_eq!(normalize_gray_image(vec![1,52,31]), vec![0,255,150]);
}
#[test]
fn normalize_gray_image_test2() {
    assert_eq!(normalize_gray_image(vec![1,255,30]), vec![0,255,29]);
}

#[test]
fn generate_base_paper_image_vec_test1() {
    assert_eq!(generate_base_paper_image_vec(vec![1,1,1,1,10,1,1,1,1],3,3,1), vec![3+4,2+3,3+4,2+3,2+2,2+3,3+4,2+3,3+4]);
}

#[test]
fn generate_line_vec_test1() {
    assert_eq!(generate_line_vec(vec![10,10,10,10,1,10,10,10,10],vec![10,10,10,10,10,10,10,10,10]), vec![255,255,255,255,0,255,255,255,255]);
}


// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

/**
 * (0,0,0,0)のピクセルは0として判定されるので注意
 */
#[wasm_bindgen]
pub fn raw_img_to_gray_vec(raw_data: Vec<u8>) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let image = decode_raw_data(raw_data);
    let gray_image = image.grayscale();
    let gray_vec = gray_image.to_luma8().into_vec();
    normalize_gray_image(gray_vec)
}

pub fn decode_raw_data(raw_data: Vec<u8>) -> DynamicImage {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let reader = Reader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .expect("Cursor io never fails");
    reader.decode().unwrap()
}

pub fn normalize_gray_image(gray_image_vec: Vec<u8>) -> Vec<u8> {
    let min:u8 = match gray_image_vec.iter().min() {
        Some(n) => *n,
        None => 0,
    };
    let max:u8 = match gray_image_vec.iter().max() {
        Some(n) => *n,
        None => 255,
    };
    let normalize_magnification:f32 = (255_f32)/((max-min) as f32);
    gray_image_vec.into_iter()
        .map(|n| ((n - min) as f32 * normalize_magnification) )
        .map(|n| if n > 255.0 {
                255.0
            } else if n < 0.0 {
                0.0
            } else {
                n
            }
        )
        .map(|n| n as u8 )
        .collect()
}

// generate image without line
// bottle neck
// ￣￣￣\/\/￣￣￣ => average:￣￣￣----￣￣￣ + deviation:_____----_____ = base:￣￣￣￣￣￣￣￣
#[wasm_bindgen]
pub fn generate_base_paper_image_vec(gray_image_vec: Vec<u8>, width: u32, height :u32, radius: u32) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let gray_image = ImageBuffer::<image::Luma<u8>, Vec<_>>
        ::from_vec(width, height, gray_image_vec).unwrap();
    let integral_image = integral_image::<_, u32>(&gray_image);// (width+1,height+1)
    let integral_squared_image = integral_squared_image::<_, u32>(&gray_image);// (width+1,height+1)

    let mut base_paper_image_vec:Vec<u8> = Vec::new();
    for y in 0..height {
        for x in 0..width {
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
                    width - 1
                };
            let end_y:u32 = if y + radius < height {
                    y + radius
                } else {
                    height- 1
                };

            let partial_sum:u32 = sum_image_pixels(&integral_image,start_x,start_y,end_x,end_y)[0];
            let pixel_sum:u32 = (end_x - start_x + 1) * (end_y - start_y + 1);

            let average:f64 = partial_sum as f64 / pixel_sum as f64;
            let variance:f64 = variance(&integral_image, &integral_squared_image, start_x,start_y,end_x,end_y);

            let base_f64:f64 = average + variance.sqrt();
            base_paper_image_vec.push(
                if base_f64 > u8::max_value() as f64 {
                    u8::max_value()
                } else if base_f64 < u8::min_value() as f64 {
                    u8::min_value()
                } else {
                    base_f64 as u8
                }
            );
        }
    }
    base_paper_image_vec
}

#[wasm_bindgen]
pub fn generate_line_vec(gray_image_vec: Vec<u8>, base_paper_image_vec: Vec<u8>) -> Vec<u8> {
    if gray_image_vec.len() != base_paper_image_vec.len() {
        panic!();
    }

    return normalize_gray_image(
        gray_image_vec.iter()// ￣￣\/￣￣
        .zip(base_paper_image_vec.iter())// ￣￣￣￣￣￣
        .map(|(n, m)| *m as i16 - *n as i16)
        .map(|n|
            if n > u8::max_value() as i16 {
                u8::max_value()
            } else if n < u8::min_value() as i16 {
                u8::min_value()
            } else {
                n as u8
            }
        )// ___/\___
        .map(|n| u8::max_value() - n)
        .collect()
    );
}

#[wasm_bindgen]
pub fn threshold_line_vec(line_vec: Vec<u8>, threshold_level: u8) -> Vec<u8> {
    return line_vec.iter()
    .map(|n| if *n < threshold_level {
        u8::min_value()
    } else {
        u8::max_value()
    })
    .collect();
}

