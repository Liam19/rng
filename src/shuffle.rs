use crate::Rng;

pub trait ShuffleSlice {
    /// The element type.
    type Item;

    /// Shuffles the elements in a random order
    fn shuffle(&mut self, rng: &mut Rng);
}

impl<T> ShuffleSlice for [T] {
    type Item = T;

    #[inline]
    fn shuffle(&mut self, rng: &mut Rng) {
        for idx in (1..self.len()).rev() {
            self.swap(idx, rng.gen_range(0..=idx));
        }
    }
}

#[test]
fn test_shuffle() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];

    vec.shuffle(Rng::new());

    dbg!(vec);
}
