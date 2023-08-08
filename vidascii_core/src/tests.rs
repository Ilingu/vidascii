#[cfg(test)]
mod core_tests {
    use std::time::Instant;

    use crate::to_braille;

    #[test]
    fn to_braille_bench_test() {
        let avg_elapsed = (0..10).fold(0_u128, |acc, _| {
            let now = Instant::now();
            for _ in 0..1_000_000 {
                assert_eq!(to_braille(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap(), '⣿');
            }
            acc + now.elapsed().as_millis()
        }) / 10;

        println!("{avg_elapsed}ms");
        assert!(avg_elapsed < 1000)
    }
}
