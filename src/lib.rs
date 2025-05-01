#![allow(missing_docs)]

mod gen_range;
mod gen_value;
mod rng;
mod sample;

pub use gen_range::*;
pub use gen_value::*;
pub use rng::*;
pub use sample::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rng;

    #[test]
    fn test_range_int() {}

    #[test]
    fn test_range_float() {}

    #[test]
    fn test_gen() {
        for _ in 0..2 {
            let rng = Rng::new();
            // let rand = rng();

            dbg!(rng.gen_value::<f32>());
        }
    }

    #[test]
    fn test_sample() {}

    #[test]
    fn test_sample_multi() {
        let mut rng = Rng::new();
        let slice = [1, 2, 3, 4, 5];

        // Test basic functionality
        let samples = slice.sample_multi(&mut rng, 3);

        assert_eq!(samples.len(), 3);

        // All sampled items exist in the original slice
        assert!(samples.iter().all(|&x| slice.contains(x)));

        // All unique
        assert_eq!(
            samples
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            3
        );

        // Test requesting all elements (no duplicates, full set)
        let all_samples = slice.sample_multi(&mut rng, slice.len());

        assert_eq!(all_samples.len(), slice.len());

        // All unique
        assert_eq!(
            all_samples
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            slice.len()
        );

        // Test edge cases
        assert!(Vec::<&i32>::new().is_empty()); // Empty slice
        assert!([1, 2, 3].sample_multi(&mut rng, 0).is_empty()); // amount = 0
    }
}
