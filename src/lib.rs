extern crate wasm_bindgen;
extern crate image;
extern crate imageproc;

use wasm_bindgen::prelude::*;
use std::io::Cursor;
use image::ImageFormat;
use image::ImageBuffer;
use image::io::Reader;
use imageproc::integral_image::integral_image;
use imageproc::gray_image;

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
pub fn say(s: &str) -> i32 {
  let r = String::from("hello ");
  return 11;
}

#[wasm_bindgen]
pub fn vec_test(s: Vec<u8>) -> Vec<u8> {
  let r = String::from("hello ");
  return s;
}



pub fn main() {
    let raw_data = b"P1 2 2\n\
        0 1\n\
        1 0\n";
    let reader = Reader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .expect("Cursor io never fails");
    assert_eq!(reader.format(), Some(ImageFormat::Pnm));

    let image = reader.decode().unwrap();
    let gray_image = image.grayscale();
    let uooo = gray_image.to_luma8();


    let integral_image = integral_image::<_, u32>(&uooo);
}


