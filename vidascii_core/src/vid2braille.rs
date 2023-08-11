use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
    thread,
};

use scopeguard::defer;

use crate::{img2braille::image_to_braille, utils::open_app_path, CoreError};

fn extract_frame(file_path: &Path) -> Result<Vec<Vec<u8>>, CoreError> {
    if !file_path.exists() {
        return Err(CoreError::FileNotFound);
    }
    if !file_path.is_file() {
        return Err(CoreError::NotAFile);
    }

    let app_path = open_app_path()?;
    defer! {
        let _ = fs::remove_dir_all(&app_path);
    }

    let full_path = file_path
        .canonicalize()
        .map_err(|_| CoreError::FileNotFound)?;

    Command::new("ffmpeg")
        .args([
            "-i",
            &full_path.display().to_string(),
            "-vf",
            "fps=12",
            &format!("{app_path}/out%d.png"),
        ])
        .output()
        .map_err(|_| CoreError::StreamError)?;

    let mut i = 1;
    let mut frames = vec![];
    while let Ok(img_data) = fs::read(format!("{app_path}/out{}.png", i)) {
        i += 1;
        frames.push(img_data);
    }

    if frames.is_empty() {
        return Err(CoreError::StreamNotFound);
    }

    Ok(frames)
}

pub fn video_to_braille(
    file_path: &Path,
    out_path: &Path,
    ratio: f32,
    dithering: bool,
) -> Result<(), CoreError> {
    if !out_path.exists() || !out_path.is_dir() {
        return Err(CoreError::OutputNotFound);
    }

    let app_path = open_app_path()?;
    defer! {
        let _ = fs::remove_dir_all(&app_path);
    }

    let mut convert_tasks = vec![];
    // Decode video to frames
    {
        for (file_id, png_frame_data) in extract_frame(file_path)?.into_iter().enumerate() {
            let app_path_copy = app_path.clone();
            convert_tasks.push(thread::spawn(move || {
                let img_datas = image_to_braille(&png_frame_data, ratio, dithering)?;

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(format!("{app_path_copy}/{file_id}.png"))
                    .map_err(|_| CoreError::FailedToSave)?;
                file.write_all(&img_datas)
                    .map_err(|_| CoreError::FailedToSave)?;
                Ok(())
            }));
        }
    }

    // wait for all frames to be converted
    for tasks in convert_tasks {
        tasks.join().map_err(|_| CoreError::FailedToConvert)??;
    }

    // re encode to video
    Command::new("ffmpeg")
        .args([
            "-framerate",
            "12",
            "-pattern_type",
            "glob",
            "-i",
            &format!("{app_path}/*.png"),
            "-c:v",
            "libx264",
            &format!("{}/output.mp4", out_path.display()),
        ])
        .output()
        .map_err(|_| CoreError::VideoEncodingError)?;

    Ok(())
}
