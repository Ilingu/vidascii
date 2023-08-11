use std::error::Error;

use image::DynamicImage;

pub struct FloydSteinbergDithering();
impl FloydSteinbergDithering {
    pub fn apply_to(img: &mut DynamicImage) -> Result<(), Box<dyn Error>> {
        if img.as_mut_rgba8().is_some() {
            return floyd_steinberg_dithering_rgba::apply_to(img);
        } else if img.as_mut_rgb8().is_some() {
            return floyd_steinberg_dithering_rgb::apply_to(img);
        }
        Err("Failed to get pixels".into())
    }
}

mod floyd_steinberg_dithering_rgba {
    use std::error::Error;

    use image::{DynamicImage, Rgba};

    use crate::img2braille::{compute_brightness, GrayScaleMode};

    pub fn apply_to(img: &mut DynamicImage) -> Result<(), Box<dyn Error>> {
        let (width, height) = (img.width(), img.height());
        let pixels = img.as_mut_rgba8().unwrap();

        for y in 0..height {
            for x in 0..width {
                let opixel = pixels.get_pixel_mut(x, y);
                let npixel = find_closest_color(opixel);

                let quant_error = [
                    opixel.0[0] as i16 - npixel[0] as i16,
                    opixel.0[1] as i16 - npixel[1] as i16,
                    opixel.0[2] as i16 - npixel[2] as i16,
                ];
                *opixel = Rgba(npixel);

                propagate_error(
                    pixels.get_pixel_mut_checked(x + 1, y),
                    quant_error,
                    7.0 / 16.0,
                );
                if x > 0 {
                    propagate_error(
                        pixels.get_pixel_mut_checked(x - 1, y + 1),
                        quant_error,
                        3.0 / 16.0,
                    );
                }
                propagate_error(
                    pixels.get_pixel_mut_checked(x, y + 1),
                    quant_error,
                    5.0 / 16.0,
                );
                propagate_error(
                    pixels.get_pixel_mut_checked(x + 1, y + 1),
                    quant_error,
                    1.0 / 16.0,
                );
            }
        }

        Ok(())
    }

    fn propagate_error(pixels: Option<&mut Rgba<u8>>, error: [i16; 3], coef: f32) {
        if let Some(Rgba([r, g, b, _])) = pixels {
            *r = (*r as f32 + error[0] as f32 * coef).max(0.0).min(255.0) as u8;
            *g = (*g as f32 + error[1] as f32 * coef).max(0.0).min(255.0) as u8;
            *b = (*b as f32 + error[2] as f32 * coef).max(0.0).min(255.0) as u8;
        }
    }

    fn find_closest_color(Rgba([r, g, b, _]): &Rgba<u8>) -> [u8; 4] {
        if compute_brightness([*r, *g, *b], GrayScaleMode::Average) >= 128 {
            [255, 255, 255, 255]
        } else {
            [0, 0, 0, 255]
        }
    }
}

mod floyd_steinberg_dithering_rgb {
    use std::error::Error;

    use image::{DynamicImage, Rgb};

    use crate::img2braille::{compute_brightness, GrayScaleMode};

    pub fn apply_to(img: &mut DynamicImage) -> Result<(), Box<dyn Error>> {
        let (width, height) = (img.width(), img.height());
        let pixels = img.as_mut_rgb8().unwrap();

        for y in 0..height {
            for x in 0..width {
                let opixel = pixels.get_pixel_mut(x, y);
                let npixel = find_closest_color(opixel);

                let quant_error = [
                    opixel.0[0] as i16 - npixel[0] as i16,
                    opixel.0[1] as i16 - npixel[1] as i16,
                    opixel.0[2] as i16 - npixel[2] as i16,
                ];
                *opixel = Rgb(npixel);

                propagate_error(
                    pixels.get_pixel_mut_checked(x + 1, y),
                    quant_error,
                    7.0 / 16.0,
                );
                if x > 0 {
                    propagate_error(
                        pixels.get_pixel_mut_checked(x - 1, y + 1),
                        quant_error,
                        3.0 / 16.0,
                    );
                }
                propagate_error(
                    pixels.get_pixel_mut_checked(x, y + 1),
                    quant_error,
                    5.0 / 16.0,
                );
                propagate_error(
                    pixels.get_pixel_mut_checked(x + 1, y + 1),
                    quant_error,
                    1.0 / 16.0,
                );
            }
        }

        Ok(())
    }

    fn propagate_error(pixels: Option<&mut Rgb<u8>>, error: [i16; 3], coef: f32) {
        if let Some(Rgb([r, g, b])) = pixels {
            *r = (*r as f32 + error[0] as f32 * coef).clamp(0.0, 255.0) as u8;
            *g = (*g as f32 + error[1] as f32 * coef).clamp(0.0, 255.0) as u8;
            *b = (*b as f32 + error[2] as f32 * coef).clamp(0.0, 255.0) as u8;
        }
    }

    fn find_closest_color(Rgb([r, g, b]): &Rgb<u8>) -> [u8; 3] {
        if compute_brightness([*r, *g, *b], GrayScaleMode::Average) >= 128 {
            [255, 255, 255]
        } else {
            [0, 0, 0]
        }
    }
}
