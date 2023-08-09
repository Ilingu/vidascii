use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use headless_chrome::{protocol::cdp::Page, Browser};
use scopeguard::defer;
use uuid::Uuid;

use crate::CoreError;

pub fn braille_pixels_to_string(braille_pixels: Vec<Vec<char>>) -> String {
    let lines = braille_pixels.join(&'\n');

    let mut img_text = String::new();
    img_text.extend(lines.iter());

    img_text
}

/* --> Sum of 2**digit == offset
0 3
1 4
2 5
6 7
*/

pub fn to_braille(dots: &[u8]) -> Result<char, CoreError> {
    let offset = dots.iter().fold(0_u32, |acc, &dot| {
        let all_combination = 2_u32.pow(dot as u32);
        acc + all_combination
    });

    if offset > 255 {
        return Err(CoreError::FailedToConvertToBraille);
    }
    char::from_u32(0x2800 + offset).ok_or(CoreError::FailedToConvertToBraille)
}

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html><html><head><style>*{margin: 0;padding: 0;font-family: sans-serif;}body{background:#000000;color:#ffffff;width:min-content;height:min-content}</style></head><body>{%bt}</body></html>"#;

pub fn braille_text_to_img(braille_text: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let app_path = open_app_path()?;
    let file_id = Uuid::new_v4().to_string();

    let file_path = format!("{}/{file_id}.html", app_path.trim_end_matches('/'));
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&file_path)?;

    let html_braille = HTML_TEMPLATE.replacen("{%bt}", &braille_text, 1);
    file.write_all(html_braille.as_bytes())?;

    defer! {
    //    let _ = fs::remove_dir_all(&app_path);
    };

    // load html page
    let full_path = Path::new(&file_path).canonicalize()?;
    println!("{}", full_path.display());
    let file_uri = format!("file://{}", full_path.to_str().ok_or("")?);

    // browser config
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(&file_uri)?;

    let png_data = tab
        .wait_for_element("body")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    Ok(png_data)
}

fn get_home_dir() -> Result<String, Box<dyn Error>> {
    Ok(home::home_dir()
        .ok_or("home dir not found")?
        .display()
        .to_string())
}

fn open_app_path() -> Result<String, Box<dyn Error>> {
    let home_dir = get_home_dir()?;
    let app_path = format!("{}/vidascii_tmp", home_dir.trim_end_matches('/'));
    fs::create_dir_all(&app_path)?;
    Ok(app_path)
}
