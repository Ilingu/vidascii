use image::{io::Reader as ImageReader, GenericImageView};
use std::{fs, io::Cursor};

use crate::{
    utils::{braille_pixels_to_string, braille_text_to_img, to_braille},
    CoreError,
};

const DOTS_POS: [(usize, usize, u8); 8] = [
    (0, 0, 0),
    (0, 1, 1),
    (0, 2, 2),
    (0, 3, 6),
    (1, 0, 3),
    (1, 1, 4),
    (1, 2, 5),
    (1, 3, 7),
];

/// `ratio` is an integer greater than 1 that can be describred by this sentences: "1 pixel on the braille image equals <ratio> pixels on the original image"
pub fn image_to_braille(image_bytes: &[u8], ratio: u32) -> Result<Vec<u8>, CoreError> {
    // decode img
    let img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|_| CoreError::WrongExtension)?
        .decode()
        .map_err(|_| CoreError::FrameDecodeError)?;

    // compute new img width/height
    let (width_chars_count, height_chars_count) = (
        (img.width() as f64 / (ratio as f64 * 2.0)).ceil() as u32,
        (img.height() as f64 / (ratio as f64 * 4.0)).ceil() as u32,
    );

    // compute each new pixels brightness
    let mut new_img_avg_pixels = vec![
        vec![vec![vec![256_u16; 4]; 2]; width_chars_count as usize];
        height_chars_count as usize
    ];
    for (x, y, rgb) in img.pixels() {
        if rgb[3] < 128 {
            continue;
        }

        let brightness = (rgb[0] as u16 + rgb[1] as u16 + rgb[2] as u16) / 3;
        let (char_x, char_y) = ((x / (2 * ratio)) as usize, (y / (4 * ratio)) as usize);
        let (inner_x, inner_y) = ((x % 2) as usize, (y % 4) as usize);

        // do avg of avg
        let new_pixel = &mut new_img_avg_pixels[char_y][char_x][inner_x][inner_y];
        *new_pixel = match *new_pixel == 256 {
            true => brightness,
            false => (*new_pixel + brightness) / 2,
        };
    }

    // map pixels to braille according to their brightness
    let braille_pixels = new_img_avg_pixels
        .iter()
        .map(|raw| {
            raw.iter()
                .map(|p| {
                    let mut dots = vec![];
                    for (x, y, dot) in DOTS_POS {
                        let avg_brightness = p[x][y];
                        if avg_brightness == 256 {
                            continue;
                        }
                        if avg_brightness >= 128 {
                            dots.push(dot)
                        }
                    }

                    to_braille(&dots)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let braille_text = braille_pixels_to_string(braille_pixels);
    fs::write("./out/result.txt", braille_text.clone()).unwrap();
    let braille_img = braille_text_to_img(braille_text).map_err(|e| {
        eprintln!("{e}");
        CoreError::FailedToConvertToImage
    })?;

    Ok(braille_img)
}
