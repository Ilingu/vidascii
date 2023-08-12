#[cfg(test)]
mod core_tests {
    use std::{fs, thread, time::Instant};

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
        fn to_braille(dots: &[u8]) -> Result<char, ()> {
            let offset = dots.iter().fold(0_u32, |acc, &dot| {
                let all_combination = 2_u32.pow(dot as u32);
                acc + all_combination
            });

            if offset > 255 {
                return Err(());
            }
            char::from_u32(0x2800 + offset).ok_or(())
        }

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
    fn get_frame_count_bench_test() {
        const REAL_FRAME_COUNT: usize = 2001;
        const APP_PATH: &str = "/tmp/vidascii_tmp/tests";

        fn concise() -> usize {
            fs::read_dir(APP_PATH)
                .unwrap()
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
                .unwrap()
        }

        fn flatmapped() -> usize {
            fs::read_dir(APP_PATH)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let (filename, filetype) = (
                        entry.file_name().to_str().map(|s| s.to_string()),
                        entry.file_type().ok(),
                    );
                    match (filename, filetype) {
                        (Some(filename), Some(filetype)) => Some((filename, filetype)),
                        _ => None,
                    }
                })
                .fold(0_usize, |acc, (filename, filetype)| {
                    if filetype.is_file() && filename.ends_with(".png") {
                        acc + 1
                    } else {
                        acc
                    }
                })
        }

        let avg_concise_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000 {
                let count = concise();
                assert_eq!(count, REAL_FRAME_COUNT);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_flatmapped_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000 {
                let count = flatmapped();
                assert_eq!(count, REAL_FRAME_COUNT);
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("avg_concise_elapsed - {avg_concise_elapsed}ms");
        println!("avg_flatmapped_elapsed - {avg_flatmapped_elapsed}ms");
    }

    #[test]
    fn extract_frames_bench_test() {
        const REAL_FRAME_COUNT: usize = 2001;
        const APP_PATH: &str = "/tmp/vidascii_tmp/tests";
        let frame_count = fs::read_dir(APP_PATH)
            .unwrap()
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
            .unwrap();
        assert_eq!(frame_count, REAL_FRAME_COUNT);

        fn single_threaded_while(frame_count: usize) {
            let mut i = 1;
            let mut frames = vec![];
            while let Ok(img_data) = fs::read(format!("{APP_PATH}/{}.png", i)) {
                i += 1;
                frames.push(img_data);
            }
            if frames.len() != frame_count {
                panic!("frames.len() != frame_count")
            }
        }

        fn single_threaded_map(frame_count: usize) {
            let frames = (1..=frame_count)
                .map(|i| fs::read(format!("{APP_PATH}/{}.png", i)))
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            if frames.len() != frame_count {
                panic!("frames.len() != frame_count")
            }
        }

        fn parallel(frame_count: usize) {
            // retreive the png frames
            let mut frames_task = vec![];
            for fid in 1..=frame_count {
                frames_task.push(thread::spawn(move || {
                    fs::read(format!("{APP_PATH}/{}.png", fid))
                }));
            }

            let mut frames = vec![];
            for task in frames_task {
                let frame_data = task.join().unwrap().unwrap();
                frames.push(frame_data);
            }

            if frames.len() != frame_count {
                panic!("frames.len() != frame_count")
            }
        }

        let avg_parallel_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..10 {
                parallel(frame_count);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_single_threaded_map_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..10 {
                single_threaded_map(frame_count);
            }
            acc + now.elapsed().as_millis()
        }) / 10;
        let avg_single_threaded_while_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..10 {
                single_threaded_while(frame_count);
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("avg_single_threaded_map_elapsed - {avg_single_threaded_map_elapsed}ms");
        println!("avg_single_threaded_while_elapsed - {avg_single_threaded_while_elapsed}ms");
        println!("avg_parallel_elapsed - {avg_parallel_elapsed}ms");
    }
}
