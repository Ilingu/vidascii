use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    thread,
};

use ffmpeg_sidecar::command::FfmpegCommand;
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

    // decoce video into streams of png frames
    let decoding = FfmpegCommand::new()
        .input(&full_path.display().to_string())
        .args(["-vf", "fps=12"])
        .output(&format!("{app_path}/out%d.png"))
        .spawn()
        .map_err(|_| CoreError::VideoDecodingError)?
        .wait()
        .map_err(|_| CoreError::VideoDecodingError)?;
    if !decoding.success() {
        return Err(CoreError::VideoDecodingError);
    }

    // get number of frames
    let frame_count = fs::read_dir(&app_path)
        .map_err(|_| CoreError::StreamNotFound)?
        .try_fold(0_usize, |acc, entry| -> Result<usize, ()> {
            let entry = entry.map_err(|_| ())?;
            let filetype = entry.file_type().map_err(|_| ())?;

            let filename_os = entry.file_name();
            let filename = filename_os.to_str().ok_or(())?;

            Ok(if filetype.is_file() && filename.ends_with(".png") {
                acc + 1
            } else {
                acc
            })
        })
        .map_err(|_| CoreError::StreamNotFound)?;

    // retreive the png frames
    let frames = (1..=frame_count)
        .map(|i| fs::read(format!("{app_path}/out{}.png", i)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| CoreError::StreamError)?;
    if frames.len() != frame_count {
        return Err(CoreError::StreamNotFound);
    }

    Ok(frames)
}

pub fn video_to_braille<T: Fn(&'static str, u8)>(
    file_path: &Path,
    out_path: &Path,
    ratio: f32,
    dithering: bool,
    set_progress: T,
) -> Result<(), CoreError> {
    if !out_path.exists() || !out_path.is_dir() {
        return Err(CoreError::OutputNotFound);
    }

    // fetch ffmpeg lib if not installed in user machine
    set_progress("Looking for FFmpeg", 0);
    ffmpeg_sidecar::download::auto_download().map_err(|_| CoreError::FFmpegAutoDownloadFailed)?;

    let app_path = open_app_path()?;
    defer! {
        let _ = fs::remove_dir_all(&app_path);
    }

    // Decode video to frames
    set_progress("Decoding video...", 25);
    let frames = extract_frame(file_path)?;

    set_progress("converting frames...", 50);
    let mut convert_tasks = vec![];
    for (file_id, png_frame_data) in frames.into_iter().enumerate() {
        let app_path_copy = app_path.clone();
        convert_tasks.push(thread::spawn(move || {
            let img_datas = image_to_braille(&png_frame_data, ratio, dithering, None::<T>)?;

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

    // wait for all frames to be converted
    for tasks in convert_tasks {
        tasks.join().map_err(|_| CoreError::FailedToConvert)??;
    }
    set_progress("frames converted, encoding to video", 75);

    // re encode to video
    let encoding = FfmpegCommand::new()
        .args(["-framerate", "12", "-pattern_type", "glob"])
        .input(&format!("{app_path}/*.png"))
        .codec_video("libx264")
        .output(&format!("{}/output.mp4", out_path.display()))
        .spawn()
        .map_err(|_| CoreError::VideoEncodingError)?
        .wait()
        .map_err(|_| CoreError::VideoEncodingError)?;
    if !encoding.success() {
        return Err(CoreError::VideoEncodingError);
    }
    set_progress("Video encoded", 100);

    Ok(())
}
