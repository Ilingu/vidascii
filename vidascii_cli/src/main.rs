use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use vidascii_core::{img2braille, vid2braille};

fn main() {
    vid2braille::video_to_braille(
        Path::new("./assets/giphy.gif"),
        Path::new("./out"),
        1.0,
        true,
    )
    .unwrap();

    return;
    let image_bytes = fs::read("./assets/GIVENNNNNNNNN.png").unwrap();

    let result_img = img2braille::image_to_braille(&image_bytes, 1.0, true).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./out/result.png")
        .unwrap();
    file.write_all(&result_img).unwrap();
}
