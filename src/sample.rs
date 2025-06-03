use crate::Rng;

use core::{cmp::Ordering, ops::Range};

pub trait SampleSlice {
    /// The element type.
    type Item;

    /// Returns a reference to one random element of the slice, or `None` if the
    /// slice is empty.
    fn sample(&self, rng: &mut Rng) -> Option<&Self::Item>;

    /// Returns a mutable reference to one random element of the slice, or
    /// `None` if the slice is empty.
    fn sample_mut(&mut self, rng: &mut Rng) -> Option<&mut Self::Item>;

    /// Returns a Vec of references to `amount` random element of the slice
    ///
    /// Panics if the slice has fewer elements than `amount`
    fn sample_multi(&self, rng: &mut Rng, amount: usize) -> Vec<&Self::Item>;
}

impl<T> SampleSlice for [T] {
    type Item = T;

    #[inline]
    fn sample(&self, rng: &mut Rng) -> Option<&Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }

    #[inline]
    fn sample_mut(&mut self, rng: &mut Rng) -> Option<&mut Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self[rng.gen_range(0..self.len())])
        }
    }

    #[inline]
    fn sample_multi(&self, rng: &mut Rng, amount: usize) -> Vec<&Self::Item> {
        let len = self.len();

        // Ensure we don't attempt to sample more elements than available
        debug_assert!(amount <= len);

        // Generate indices and perform partial shuffle
        let mut indices: Vec<usize> = (0..len).collect();

        for i in 0..amount {
            let j = rng.gen_range(i..len);

            indices.swap(i, j);
        }

        // Collect the selected elements
        indices[..amount].iter().map(|&idx| &self[idx]).collect()
    }
}

/// Generic Cumulative distribution function sampler for efficient random sampling
/// according to given probabilities.
#[derive(Clone, Debug)]
pub struct CdfSampler<T> {
    // The cumulative distribution function values
    cdf: Vec<f32>,
    // The items to be sampled
    items: Vec<T>,
    // Precomputed maximum value for binary search optimization
    max_cdf: f32,
}

impl<T> CdfSampler<T> {
    /// Creates a new CdfSampler from items and their corresponding weights.
    ///
    /// # Panics
    /// - If items and weights have different lengths
    /// - If any weight is negative
    /// - If all weights are zero
    pub fn new(items: Vec<T>, weights: Vec<f32>) -> Self {
        debug_assert!(
            items.len() == weights.len(),
            "items and weights must have same length"
        );
        debug_assert!(
            !weights.is_empty(),
            "cannot create sampler with empty items"
        );

        // Compute cumulative distribution function
        let mut cdf = Vec::with_capacity(weights.len() + 1);
        cdf.push(0.0);

        let mut sum = 0.0;
        for &w in &weights {
            debug_assert!(w >= 0.0, "weights must be non-negative");

            sum += w;
            cdf.push(sum);
        }

        debug_assert!(sum > 0.0, "at least one weight must be positive");

        // Normalize the CDF to end at 1.0
        let max_cdf = cdf.last().copied().unwrap();

        if max_cdf != 1.0 {
            for x in &mut cdf[1..] {
                *x /= max_cdf;
            }
        }

        Self {
            cdf,
            items,
            max_cdf: 1.0,
        }
    }

    pub fn sample(&self, rng: &mut Rng) -> &T {
        let r = rng.gen_value();

        // Binary search to find the first cdf value >= r
        let index = match self
            .cdf
            .binary_search_by(|probe| probe.partial_cmp(&r).unwrap_or(Ordering::Equal))
        {
            Ok(i) => i - 1,
            Err(i) => i - 1,
        };

        self.items.get(index).unwrap()
    }

    /// Returns the number of items in the sampler
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sampling() {
//         let items = vec!["a", "b", "c"];
//         let weights = vec![1.0, 2.0, 3.0];
//         let sampler = CdfSampler::new(items, weights);

//         // Test edge cases
//         assert_eq!(sampler.sample(0.0), Some(&"a"));
//         assert_eq!(sampler.sample(0.99999), Some(&"c"));
//         assert_eq!(sampler.sample(1.0), None);
//         assert_eq!(sampler.sample(-0.1), None);

//         // Test sampling points
//         assert_eq!(sampler.sample(0.1666666), Some(&"a")); // < 1/6
//         assert_eq!(sampler.sample(0.1666667), Some(&"b")); // > 1/6
//         assert_eq!(sampler.sample(0.5), Some(&"b")); // < 0.5
//         assert_eq!(sampler.sample(0.500001), Some(&"c")); // > 0.5
//     }

//     #[test]
//     fn test_empty() {
//         let sampler: CdfSampler<i32> = CdfSampler::new(vec![], vec![]);
//         assert!(sampler.is_empty());
//         assert_eq!(sampler.sample(0.5), None);
//     }
// }
