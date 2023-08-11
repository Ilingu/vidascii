mod dithering;

use braille2img::braille2img;
use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use std::io::Cursor;

use crate::CoreError;

use self::dithering::FloydSteinbergDithering;

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
pub fn image_to_braille(
    image_bytes: &[u8],
    ratio: f32,
    dithering: bool,
) -> Result<Vec<u8>, CoreError> {
    // decode img
    let mut img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|_| CoreError::WrongExtension)?
        .decode()
        .map_err(|_| CoreError::FrameDecodeError)?;

    if dithering {
        FloydSteinbergDithering::apply_to(&mut img).map_err(|_| CoreError::DitheringFailed)?;
    }

    // compute new img width/height
    let (width_chars_count, height_chars_count) = (
        (img.width() as f32 / (ratio * 2.0)).ceil() as u32,
        (img.height() as f32 / (ratio * 4.0)).ceil() as u32,
    );

    // compute each new pixels brightness
    let mut new_img_avg_pixels =
        vec![vec![[[256_u16; 4]; 2]; width_chars_count as usize]; height_chars_count as usize];

    let sub_divised_by = (1.0 / ratio).round() as u32;
    if sub_divised_by > 1 {
        for (x, y, Rgba([r, g, b, a])) in img.pixels() {
            if a < 128 {
                continue;
            }

            let brightness = compute_brightness([r, g, b], GrayScaleMode::Luminance);
            let sub_x_pixels = (x * (sub_divised_by / 2))..((sub_divised_by / 2) * (x + 1));
            let sub_y_lines = sub_divised_by as usize; // = sub_x_pixels.len() * 2
            let inner_chars_lines = sub_y_lines as f32 / 4.0;

            for char_x in sub_x_pixels {
                for line_id in 0..sub_y_lines {
                    let (inner_char_y, inner_y) = (
                        line_id / 4,
                        if inner_chars_lines >= 1.0 {
                            line_id % 4
                        } else {
                            2 * (y as usize % 2) + line_id
                        },
                    );
                    let char_y =
                        (inner_chars_lines * y as f32 + inner_char_y as f32).floor() as usize;

                    let new_pixel = &mut new_img_avg_pixels[char_y][char_x as usize];
                    new_pixel[0][inner_y] = brightness;
                    new_pixel[1][inner_y] = brightness;
                }
            }
        }
    } else {
        let ratio = ratio.round();
        for (x, y, Rgba([r, g, b, a])) in img.pixels() {
            if a < 128 {
                continue;
            }

            let brightness = compute_brightness([r, g, b], GrayScaleMode::Average);
            let (char_x, char_y) = (
                (x as f32 / (2.0 * ratio)) as usize,
                (y as f32 / (4.0 * ratio)) as usize,
            );
            let (inner_x, inner_y) = (
                ((x as f32 / ratio) % 2.0) as usize,
                ((y as f32 / ratio) % 4.0) as usize,
            );

            // do avg of avg
            let new_pixel = &mut new_img_avg_pixels[char_y][char_x][inner_x][inner_y];
            *new_pixel = match *new_pixel == 256 {
                true => brightness,
                false => (*new_pixel + brightness) / 2,
            };
        }
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
    let braille_img_datas =
        braille2img(&braille_text, None).map_err(|_| CoreError::FailedToConvertToImage)?;

    Ok(braille_img_datas)
}

pub enum GrayScaleMode {
    Luminance,
    Average,
}

fn compute_brightness([r, g, b]: [u8; 3], mode: GrayScaleMode) -> u16 {
    match mode {
        GrayScaleMode::Luminance => {
            (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32).round() as u16
        }
        GrayScaleMode::Average => (r as u16 + g as u16 + b as u16) / 3,
    }
}

fn braille_pixels_to_string(braille_pixels: Vec<Vec<char>>) -> String {
    let lines = braille_pixels.join(&'\n');

    let mut img_text = String::new();
    img_text.extend(lines.iter());

    img_text
}

fn to_braille(dots: &[u8]) -> Result<char, CoreError> {
    let offset = dots.iter().fold(0_u32, |acc, &dot| {
        let all_combination = 2_u32.pow(dot as u32);
        acc + all_combination
    });

    if offset > 255 {
        return Err(CoreError::FailedToConvertToBraille);
    }
    char::from_u32(0x2800 + offset).ok_or(CoreError::FailedToConvertToBraille)
}
