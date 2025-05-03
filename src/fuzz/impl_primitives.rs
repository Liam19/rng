use crate::{Rng, fuzz::RandomInstance};

macro_rules! impl_random_instance {
    ($($t:ty),*) => {
        $(
            impl RandomInstance for $t {
                #[inline]
                fn random_instance(rng: &mut Rng) -> Self {
                    rng.gen_value()
                }
            }
        )*
    };
}

// Implement for all primitives
impl_random_instance!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char, String
);

impl RandomInstance for f32 {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.08) {
            return 0.0;
        }

        if rng.gen_bool(0.3) {
            return rng.gen_value::<f32>();
        }

        rng.gen_value::<f32>() * f32::MAX
    }
}

impl RandomInstance for f64 {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.08) {
            return 0.0;
        }

        if rng.gen_bool(0.3) {
            return rng.gen_value::<f64>();
        }

        rng.gen_value::<f64>() * f64::MAX
    }
}

impl RandomInstance for bool {
    #[inline]
    fn random_instance(rng: &mut Rng) -> Self {
        rng.gen_bool(0.5)
    }
}
