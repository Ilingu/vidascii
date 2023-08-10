pub mod img2braille;
mod tests;
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
}
