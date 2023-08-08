mod tests;

use std::{path::Path, thread};

use image::{io::Reader as ImageReader, GenericImageView, ImageFormat};
use std::io::Cursor;
use vid2img::FileSource;

const HIGH_RES_CHARS: &str = "⠀⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏";

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

// 255 67
// 124

/* --> Sum of 2**digit == offset
0 3
1 4
2 5
6 7
*/

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
fn image_to_ascii(image_bytes: &[u8], ratio: u32) -> Result<(), CoreError> {
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
        vec![vec![vec![0_u16; 4]; 2]; width_chars_count as usize];
        height_chars_count as usize
    ];
    for (x, y, rgb) in img.pixels() {
        let brightness = rgb.0[0..=2].iter().fold(0_u16, |acc, pv| acc + *pv as u16) / 3;
        let (char_x, char_y) = (
            ((x * width_chars_count) as f64 / img.width() as f64).round() as usize,
            ((y * height_chars_count) as f64 / img.height() as f64).round() as usize,
        );
        let (inner_x, inner_y) = ((x % 2) as usize, (y % 4) as usize);

        // do avg
        new_img_avg_pixels[char_y][char_x][inner_x][inner_y] =
            (new_img_avg_pixels[char_y][char_x][inner_x][inner_y] + brightness) / 2;
    }

    // to parallel
    // let new_img_braille_pixels = new_img_avg_pixels
    //     .iter()
    //     .map(|raw| raw.iter().map(|p| p.iter().enumerate().filter(|(x, s)| {})));
    Ok(())
}

fn to_braille(dots: &[u8]) -> Result<char, CoreError> {
    let offset = dots.iter().try_fold(0_u32, |acc, &dot| {
        if dot > 7 {
            return Err(CoreError::FailedToConvertToBraille);
        }

        let all_combination = 2_u32.pow(dot as u32);
        Ok(acc + all_combination)
    })?;
    char::from_u32(0x2800 + offset).ok_or(CoreError::FailedToConvertToBraille)
}
