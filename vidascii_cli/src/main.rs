use std::fs;

use vidascii_core::image_to_ascii;

fn main() {
    let image_bytes = fs::read("./assets/GIVENNNNNNNNN.png").unwrap();
    image_to_ascii(&image_bytes, 1).unwrap();
}
