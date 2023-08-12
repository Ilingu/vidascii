mod console;

use std::{
    fs::{self, OpenOptions},
    io::Write as FileWrite,
    path::Path,
    time::Duration,
};

use clap::Parser;
use colored::Colorize;
use console::{console_log, Level};
use indicatif::{ProgressBar, ProgressStyle};
use vidascii_core::{img2braille, vid2braille};

/// A simple *image/video* to braille *image/video* art converter 📼
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the video or image file to convert
    #[arg(short, long)]
    input: String,

    /// Path to the output **directory** 📂
    #[arg(short, long)]
    output: String,

    /// The 'ratio' is the amount of braille characters dots per pixel.
    /// For exemple:
    ///
    /// - `-ratio=1.0` mean that, one braille dot = one pixel, in other words each pixel is conserved on the ouput,
    /// original image quality is conserved
    ///
    /// - `-ratio=2.0` mean that, one braille dot = 4 pixels, so there is a data lost, output image quality will be degraded
    ///
    /// - in general: `-ratio=n` with n >= 1.0; mean that, one braille dot = 4**(n-1) pixels.
    ///
    /// You can specified a ratio between 0 and 1, this will expand the output image,
    /// but there are no real image quality improvement by doing so. This will only be a lot slower to compute
    #[arg(short, long, default_value_t = 1.0)]
    ratio: f32,

    /// whether or not you want to enable 'dithering' pre-processing
    ///
    /// Dithering significally improve the end image quality.
    /// if enable this step have a really tiny impact on the waiting time, this is why it's enabled by default.
    ///
    /// @see: https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
    #[arg(short, long, default_value_t = true)]
    dithering: bool,
}

fn check_io(input: &Path, output: &Path) -> Result<(), ()> {
    let is_err = !input.exists() || !output.exists() || !input.is_file() || !output.is_dir();

    // check if exists
    if !input.exists() {
        console_log(
            format!(
                "'{}' does not seem to exist...",
                input.display().to_string().italic()
            ),
            Level::Error,
        );
    }
    if !output.exists() {
        console_log(
            format!(
                "'{}' does not seem to exist...",
                output.display().to_string().italic()
            ),
            Level::Error,
        );
    }

    // check filetype
    if !input.is_file() {
        console_log(
            format!(
                "'{}' is not a file...",
                input.display().to_string().italic()
            ),
            Level::Error,
        );
    }
    if !output.is_dir() {
        console_log(
            format!(
                "'{}' is not a directory...",
                output.display().to_string().italic()
            ),
            Level::Error,
        );
    }

    // return status
    if is_err {
        Err(())
    } else {
        Ok(())
    }
}

fn init_progress_bar() -> (ProgressBar, impl Fn(&'static str, u8)) {
    let pb = ProgressBar::new(100);

    pb.enable_steady_tick(Duration::from_millis(250));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.yellow} [{elapsed}] |{msg}| [{wide_bar:.yellow/white}]",
        )
        .unwrap()
        .progress_chars("#-"),
    );

    let pb_ui = pb.clone();
    let set_progress = move |msg: &'static str, new_position: u8| {
        pb_ui.set_message(msg);
        pb_ui.set_position(new_position as u64);
    };

    (pb, set_progress)
}

fn main() {
    let args = Args::parse();
    console_log("Welcome to vidascii!".bold(), Level::Info);

    let (input, output) = (Path::new(&args.input), Path::new(&args.output));
    if check_io(input, output).is_err() {
        return;
    }

    // get input mime type
    let guess = mime_guess::from_path(input);
    let mime = match guess.first() {
        Some(mime) => mime,
        None => {
            console_log("Failed to read input file extension", Level::Error);
            return;
        }
    };

    let mut mime_type = mime.type_();
    if mime == mime_guess::mime::IMAGE_GIF {
        mime_type = mime_guess::mime::VIDEO;
    }

    match mime_type {
        mime_guess::mime::VIDEO => {
            console_log("📼 Video detected. Converting... ⏳", Level::Info);

            let (progress_bar, set_progress) = init_progress_bar();
            let convertion_result = vid2braille::video_to_braille(
                input,
                output,
                args.ratio,
                args.dithering,
                set_progress,
            );
            progress_bar.finish();

            match convertion_result {
                Ok(_) => {
                    console_log(
                        format!(
                            "Video successfully converted! Check: {}",
                            format!("{}/output.mp4", output.display()).bold().italic()
                        ),
                        Level::Success,
                    );
                }
                Err(why) => console_log(format!("Failed to convert Video: {why:?}"), Level::Error),
            }
        }
        mime_guess::mime::IMAGE => {
            console_log("🖼 Image detected.", Level::Info);

            let image_bytes = fs::read(input).unwrap();
            console_log("Image loaded. Converting... ⏳", Level::Info);

            let (progress_bar, set_progress) = init_progress_bar();
            let conversion_result = img2braille::image_to_braille(
                &image_bytes,
                args.ratio,
                args.dithering,
                Some(set_progress),
            );
            progress_bar.finish();

            match conversion_result {
                Ok(out_img_datas) => {
                    console_log("Image successfully converted! Saving... 💾", Level::Success);

                    let output_file = format!("{}/output.png", output.display());
                    let mut file = match OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(&output_file)
                    {
                        Ok(f) => f,
                        Err(_) => {
                            console_log("Failed to save image", Level::Error);
                            return;
                        }
                    };
                    if file.write_all(&out_img_datas).is_err() {
                        console_log("Failed to save image", Level::Error);
                        return;
                    }

                    console_log(
                        format!(
                            "Image saved sucessfully! Check: {}",
                            output_file.bold().italic()
                        ),
                        Level::Success,
                    );
                }
                Err(why) => console_log(format!("Failed to convert Image: {why:?}"), Level::Error),
            }
        }
        _ => console_log("Input file is nor an image nor a video", Level::Error),
    }
}
