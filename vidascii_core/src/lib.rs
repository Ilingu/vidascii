mod tests;

use std::{fs, path::Path, thread};

use image::{io::Reader as ImageReader, GenericImageView, ImageFormat};
use std::io::Cursor;
use vid2img::FileSource;

#[derive(Debug)]
pub enum CoreError {
    FileNotFound,
    NotAFile,
    WrongExtension,
    StreamError,
    StreamNotFound,
    FailedToConvert,
    FrameDecodeError,
    FailedToConvertToBraille,
}

pub fn video_to_ascii(file_path: &Path) -> Result<(), CoreError> {
    if !file_path.exists() {
        return Err(CoreError::FileNotFound);
    }
    if !file_path.is_file() {
        return Err(CoreError::NotAFile);
    }

    let extension = file_path.extension().ok_or(CoreError::WrongExtension)?;
    if extension != "mp4" {
        return Err(CoreError::WrongExtension);
    }

    let mut convert_tasks = vec![];
    // Decode video to frames
    {
        let frame_source = FileSource::new(file_path, (200, 200)).unwrap();
        for frame in frame_source.into_iter() {
            let png_img_data = frame
                .map_err(|_| CoreError::StreamError)?
                .ok_or(CoreError::StreamNotFound)?;

            convert_tasks.push(thread::spawn(move || image_to_ascii(&png_img_data, 100)));
        }
    }

    for tasks in convert_tasks {
        tasks.join().map_err(|_| CoreError::FailedToConvert)??;
    }

    Ok(())
}

/// `ratio` is an integer greater than 1 that can be describred by this sentences: "1 pixel on the braille image equals <ratio> pixels on the original image"
pub fn image_to_ascii(image_bytes: &[u8], ratio: u32) -> Result<(), CoreError> {
    let mut img_config = ImageReader::new(Cursor::new(image_bytes));
    img_config.set_format(ImageFormat::Png);

    let img = img_config
        .decode()
        .map_err(|_| CoreError::FrameDecodeError)?;

    let (width_chars_count, height_chars_count) = (
        (img.width() as f64 / (ratio as f64 * 2.0)).ceil() as u32,
        (img.height() as f64 / (ratio as f64 * 4.0)).ceil() as u32,
    );

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

    let img_text = braille_pixels_to_string(braille_pixels);
    fs::write("./out/result.txt", img_text).unwrap();

    Ok(())
}

fn braille_pixels_to_string(braille_pixels: Vec<Vec<char>>) -> String {
    let lines = braille_pixels.join(&'\n');

    let mut img_text = String::new();
    img_text.extend(lines.iter());

    img_text
}

/* --> Sum of 2**digit == offset
0 3
1 4
2 5
6 7
*/

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
