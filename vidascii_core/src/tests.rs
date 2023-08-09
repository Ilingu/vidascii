#[cfg(test)]
mod core_tests {
    use std::{
        error::Error,
        fs::{self, OpenOptions},
        io::Write,
        path::Path,
        process::Command,
        time::Instant,
    };

    use headless_chrome::{protocol::cdp::Page, Browser};
    use scopeguard::defer;
    use uuid::Uuid;

    use crate::utils::to_braille;

    #[test]
    fn braille_pixels_to_string_bench_test() {
        fn extend(braille_pixels: Vec<Vec<char>>) -> String {
            let lines = braille_pixels.join(&'\n');

            let mut joined_string = String::new();
            joined_string.extend(lines.iter());

            joined_string
        }
        fn classic1(braille_pixels: Vec<Vec<char>>) -> String {
            braille_pixels
                .iter()
                .map(|c| c.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n")
        }
        fn classic2(braille_pixels: Vec<Vec<char>>) -> String {
            braille_pixels
                .join(&'\n')
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        }

        let avg_classic1_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000 {
                classic1(vec![vec!['⣿'; 100]; 100]);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_classic2_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000 {
                classic2(vec![vec!['⣿'; 100]; 100]);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_extend_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000 {
                extend(vec![vec!['⣿'; 100]; 100]);
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("avg_classic1_elapsed - {avg_classic1_elapsed}ms");
        println!("avg_classic2_elapsed - {avg_classic2_elapsed}ms");
        println!("avg_extend_elapsed - {avg_extend_elapsed}ms");
    }

    #[test]
    fn avg_bench_test() {
        fn avg_fold(rbg: [u8; 3]) -> u16 {
            rbg[0..=2].iter().fold(0_u16, |acc, pv| acc + *pv as u16) / 3
        }

        fn avg(rbg: [u8; 3]) -> u16 {
            (rbg[0] as u16 + rbg[1] as u16 + rbg[2] as u16) / 3
        }

        let avg_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000_000 {
                avg([255, 255, 255]);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_fold_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000_000 {
                avg_fold([255, 255, 255]);
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("avg - {avg_elapsed}ms");
        println!("avg_fold - {avg_fold_elapsed}ms");
        assert!(avg_elapsed < avg_fold_elapsed)
    }

    #[test]
    fn to_braille_bench_test() {
        let avg_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000_000 {
                to_braille(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("{avg_elapsed}ms");
        assert!(avg_elapsed < 1000)
    }

    #[test]
    fn braille_text_to_img_bench_test() {
        const HTML_TEMPLATE: &str = r#"<!DOCTYPE html><html><body>{%bt}</body></html>"#;

        fn braille_text_to_img_js(braille_text: String) -> Result<Vec<u8>, Box<dyn Error>> {
            let html_braille = HTML_TEMPLATE.replacen("{%bt}", &braille_text, 1);

            let browser = Browser::default()?;
            let tab = browser.new_tab()?;
            tab.evaluate(
                &format!(
                    r#"(()=>{{
            document.head = 
            document.body = 
        }})()"#
                ),
                false,
            )?;

            let png_data = tab
                .wait_for_element("body")?
                .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
            Ok(png_data)
        }
        fn braille_text_to_img_file(braille_text: String) -> Result<Vec<u8>, Box<dyn Error>> {
            let file_id = Uuid::new_v4().to_string();
            let file_path = format!("../tmp/{file_id}.html");

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&file_path)?;

            let html_braille = HTML_TEMPLATE.replacen("{%bt}", &braille_text, 1);
            file.write_all(html_braille.as_bytes())?;

            defer! {
               let _ = fs::remove_file(&file_path);
            };

            // load html page
            let full_path = Path::new(&file_path).canonicalize()?;
            let file_uri = format!("file://{}", full_path.to_str().ok_or("")?);

            let browser = Browser::default()?;
            let tab = browser.new_tab()?;
            tab.navigate_to(&file_uri)?;

            let png_data = tab
                .wait_for_element("body")?
                .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
            Ok(png_data)
        }

        // benchmark
        let avg_file_elapsed = (0..5).fold(0_u128, |acc, _| {
            let now = Instant::now();
            braille_text_to_img_file("sasaki and miyano".to_string()).unwrap();
            acc + now.elapsed().as_millis()
        }) / 5;
        let avg_js_elapsed = (0..5).fold(0_u128, |acc, _| {
            let now = Instant::now();
            braille_text_to_img_js("sasaki and miyano".to_string()).unwrap();
            acc + now.elapsed().as_millis()
        }) / 5;

        println!("avg_file_elapsed - {avg_file_elapsed}ms");
        println!("avg_js_elapsed - {avg_js_elapsed}ms");
    }
}
