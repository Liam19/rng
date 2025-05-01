use crate::Rng;

use core::ops::Bound::{Excluded, Included, Unbounded};
use core::ops::RangeBounds;

pub trait RandomRange {
    fn gen_range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self;
}

impl Rng {
    #[inline]
    pub fn gen_range<T: RandomRange>(&mut self, range: impl RangeBounds<T>) -> T {
        T::gen_range(self, range)
    }
}

macro_rules! impl_random_range_int {
    ($($t:ty),*) => {
        $(
            impl RandomRange for $t {
                #[inline]
                fn gen_range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
                    let start = match range.start_bound() {
                        Included(&n) => n,
                        Excluded(&n) => n + 1,
                        Unbounded => <$t>::MIN,
                    };
                    let end = match range.end_bound() {
                        Included(&n) => n + 1,
                        Excluded(&n) => n,
                        Unbounded => <$t>::MAX,
                    };

                    if start >= end {
                        panic!("Empty or invalid range: [{start}, {end})");
                    }

                    let span = (end - start) as u64;

                    (rng.next_u64() % span) as $t + start
                }
            }
        )*
    };
}

impl_random_range_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl RandomRange for f32 {
    #[inline]
    fn gen_range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => next_after_f32(n, f32::INFINITY),
            Unbounded => f32::MIN,
        };
        let end = match range.end_bound() {
            Included(&n) => n,
            Excluded(&n) => n,
            Unbounded => f32::MAX,
        };

        assert!((start < end), "Invalid float range: [{start}, {end})");

        (rng.next_f64() as f32).mul_add(end - start, start)
    }
}

impl RandomRange for f64 {
    #[inline]
    fn gen_range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => next_after_f64(n, f64::INFINITY),
            Unbounded => f64::MIN,
        };
        let end = match range.end_bound() {
            Included(&n) => n,
            Excluded(&n) => n,
            Unbounded => f64::MAX,
        };

        assert!(start < end, "Invalid float range: [{start}, {end})");

        rng.next_f64().mul_add(end - start, start)
    }
}

/// Returns the next representable floating-point value after self in the direction of other.
///
/// This is a low-level operation used to step through floating-point values with maximum precision.
fn next_after_f32(x: f32, toward: f32) -> f32 {
    if x < toward {
        f32::from_bits(x.to_bits() + 1)
    } else if x > toward {
        f32::from_bits(x.to_bits() - 1)
    } else {
        x
    }
}

/// Returns the next representable floating-point value after self in the direction of other.
///
/// This is a low-level operation used to step through floating-point values with maximum precision.
fn next_after_f64(x: f64, toward: f64) -> f64 {
    if x < toward {
        f64::from_bits(x.to_bits() + 1)
    } else if x > toward {
        f64::from_bits(x.to_bits() - 1)
    } else {
        x
    }
}
