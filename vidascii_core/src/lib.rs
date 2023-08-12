mod braille2img;
pub mod img2braille;
mod tests;
mod utils;
pub mod vid2braille;

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
    FailedToConvertToImage,
    DitheringFailed,
    FailedToOpenAppPath,
    FailedToSave,
    FFmpegAutoDownloadFailed,
    OutputNotFound,
    VideoEncodingError,
    VideoDecodingError,
}
