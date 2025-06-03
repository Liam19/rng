use crate::Rng;

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

/// Cumulative distribution function sampling
pub struct CdfSampler<T> {
    cdf: Vec<f32>,
    items: Vec<T>,
}

impl<T> CdfSampler<T> {
    /// Creates a new CdfSampler from items and their corresponding weights.
    ///
    /// # Panics
    /// - If items and weights have different lengths
    /// - If any weight is negative
    /// - If all weights are zero
    #[inline]
    pub fn new(items: Vec<T>, weights: Vec<f32>) -> Self {
        debug_assert!(
            items.len() == weights.len(),
            "items and weights must have same length"
        );
        debug_assert!(
            !weights.is_empty(),
            "cannot create sampler with empty items"
        );

        let mut cdf = Vec::with_capacity(items.len());
        let mut items_inner = Vec::with_capacity(items.len());

        let mut total = 0.0;

        for (weight, item) in weights.into_iter().zip(items.into_iter()) {
            if weight > 0.0 {
                total += weight;
                items_inner.push(item);
                cdf.push(total);
            }
        }

        debug_assert!(total > 0.0, "at least one weight must be positive");

        // Normalize the CDF to end at 1.0
        for val in &mut cdf {
            *val /= total;
        }

        Self {
            cdf,
            items: items_inner,
        }
    }

    #[inline]
    pub fn sample(&self, rng: &mut Rng) -> &T {
        let r = rng.gen_value::<f32>();

        // Binary search to find the first cdf value >= r
        let i = match self.cdf.binary_search_by(|v| v.partial_cmp(&r).unwrap()) {
            Ok(i) | Err(i) => i,
        };

        &self.items[i]
    }
}
