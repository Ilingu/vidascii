const BASE_FONT_SIZE: f32 = 20.0; // 20px
const BASE_CHAR_WIDTH: u32 = 10; // 10px
const BASE_CHAR_HEIGHT: u32 = 18; // 18px

#[allow(dead_code)]
pub struct Braille2ImgOptionsBuilder {
    /// rbg value
    background_color: [u8; 3],
    /// rbg value
    text_color: [u8; 3],
    /// in pixels
    font_size: f32,
}

#[allow(dead_code)]
impl Braille2ImgOptionsBuilder {
    pub fn set_bg_color(mut self, rgb: [u8; 3]) -> Self {
        self.background_color = rgb;
        self
    }
    pub fn set_text_color(mut self, rgb: [u8; 3]) -> Self {
        self.text_color = rgb;
        self
    }
    pub fn set_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    pub fn build(self) -> Braille2ImgOptions {
        let char_width = self.font_size * BASE_CHAR_WIDTH as f32 / BASE_FONT_SIZE;
        let char_height = self.font_size * BASE_CHAR_HEIGHT as f32 / BASE_FONT_SIZE;

        Braille2ImgOptions {
            background_color: self.background_color,
            text_color: self.text_color,
            font_size: self.font_size,
            char_width: char_width.round() as u32,
            char_height: char_height.round() as u32,
        }
    }
}

impl Default for Braille2ImgOptionsBuilder {
    fn default() -> Self {
        Self {
            background_color: [0, 0, 0],
            text_color: [255, 255, 255],
            font_size: 20.0,
        }
    }
}

/// use: `Braille2ImgOptions::builder`
pub struct Braille2ImgOptions {
    /// rbg value
    pub background_color: [u8; 3],
    /// rbg value
    pub text_color: [u8; 3],
    /// in pixels
    pub font_size: f32,
    /// use: `Braille2ImgOptions::builder` if you don't know what this is
    pub char_width: u32,
    /// use: `Braille2ImgOptions::builder` if you don't know what this is
    pub char_height: u32,
}

#[allow(dead_code)]
impl Braille2ImgOptions {
    pub fn builder() -> Braille2ImgOptionsBuilder {
        Braille2ImgOptionsBuilder::default()
    }
}

impl Default for Braille2ImgOptions {
    fn default() -> Self {
        Self {
            background_color: [0, 0, 0],
            text_color: [255, 255, 255],
            font_size: BASE_FONT_SIZE,
            char_width: BASE_CHAR_WIDTH,
            char_height: BASE_CHAR_HEIGHT,
        }
    }
}
