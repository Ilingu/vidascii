use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use vidascii_core::img2braille::image_to_braille;

fn main() {
    let image_bytes = fs::read("./assets/miyano.jpeg").unwrap();

    let result_img = image_to_braille(&image_bytes, 1.0, true).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./out/result.png")
        .unwrap();
    file.write_all(&result_img).unwrap();
}
