use std::fs;

use uuid::Uuid;

use crate::CoreError;

pub fn open_app_path() -> Result<String, CoreError> {
    let temp_dir = std::env::temp_dir();
    let session_id = Uuid::new_v4().to_string();
    let app_path = format!(
        "{}/vidascii_tmp/{session_id}",
        temp_dir.display().to_string().trim_end_matches('/')
    );
    fs::create_dir_all(&app_path).map_err(|_| CoreError::FailedToOpenAppPath)?;
    Ok(app_path)
}
