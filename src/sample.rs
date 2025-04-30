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

    fn sample(&self, rng: &mut Rng) -> Option<&Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }

    fn sample_mut(&mut self, rng: &mut Rng) -> Option<&mut Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self[rng.gen_range(0..self.len())])
        }
    }

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
