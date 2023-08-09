use std::{path::Path, thread};

use vid2img::FileSource;

use crate::{img2braille::image_to_braille, CoreError};

pub fn video_to_braille(file_path: &Path) -> Result<(), CoreError> {
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

            convert_tasks.push(thread::spawn(move || image_to_braille(&png_img_data, 100)));
        }
    }

    for tasks in convert_tasks {
        tasks.join().map_err(|_| CoreError::FailedToConvert)??;
    }

    Ok(())
}
