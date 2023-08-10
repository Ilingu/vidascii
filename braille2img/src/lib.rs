pub mod config;

use std::{error::Error, io::Cursor};

use config::Braille2ImgOptions;
use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{Font, Scale};

pub fn braille2img(
    text: &str,
    options: Option<Braille2ImgOptions>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let options = options.unwrap_or_default();
    if text.trim().is_empty() {
        return Err("Text is empty".into());
    }

    let lines = text.lines().collect::<Vec<_>>();
    let (image_width, image_height) = (
        options.char_width * lines[0].chars().count() as u32,
        options.char_height * lines.len() as u32,
    );

    // Create a new RGB image buffer
    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    // Set the background color to black
    let background_color = Rgb(options.background_color);
    for pixel in image.pixels_mut() {
        *pixel = background_color;
    }

    // Set the text color
    let text_color = Rgb(options.text_color);

    // Set font
    let font_size = options.font_size;
    let font_data: &[u8] = include_bytes!("../fonts/Braille.ttf");
    let font: Font<'static> =
        Font::try_from_bytes(font_data).ok_or("Failed to load braille font")?;

    // Add the text to the image
    for (line_id, line) in lines.iter().enumerate() {
        imageproc::drawing::draw_text_mut(
            &mut image,
            text_color,
            0,
            (line_id as u32 * options.char_height).try_into()?,
            Scale::uniform(font_size),
            &font,
            line,
        );
    }

    // output the image datas
    let mut img_bytes: Vec<u8> = Vec::new();
    image.write_to(
        &mut Cursor::new(&mut img_bytes),
        image::ImageOutputFormat::Png,
    )?;

    Ok(img_bytes)
}
