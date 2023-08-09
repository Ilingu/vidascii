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
}

// fn braille_text_to_img(braille_text: String) -> Result<Vec<u8>, CoreError> {}
