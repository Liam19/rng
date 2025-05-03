use crate::{Rng, fuzz::RandomInstance};

macro_rules! impl_random_instance {
    ($($t:ty),*) => {
        $(
            impl RandomInstance for $t {
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
    fn random_instance(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.01) {
            return 0.0;
        }

        (1.0 / ((rng.gen_value::<f32>() * 32.0) + (1.0 / 32.0)))
    }
}

impl RandomInstance for f64 {
    fn random_instance(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.01) {
            return 0.0;
        }

        (1.0 / (rng.gen_value::<f64>() * 64.0)) * (f64::MAX / 100.0)
    }
}

impl RandomInstance for bool {
    fn random_instance(rng: &mut Rng) -> Self {
        rng.gen_bool(0.5)
    }
}
