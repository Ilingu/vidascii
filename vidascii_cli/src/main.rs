use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use vidascii_core::img2braille::image_to_braille;

fn main() {
    let image_bytes = fs::read("./assets/GIVENNNNNNNNN.png").unwrap();

    let result_img = image_to_braille(&image_bytes, 4).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./out/result.png")
        .unwrap();
    file.write_all(&result_img).unwrap();
}
