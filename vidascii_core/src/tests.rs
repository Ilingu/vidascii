#[cfg(test)]
mod core_tests {
    use std::time::Instant;

    use crate::to_braille;

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
}
