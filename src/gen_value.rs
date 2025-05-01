use crate::Rng;

pub trait RandomValue {
    fn gen_value(rng: &mut Rng) -> Self;
}

impl Rng {
    #[inline]
    pub fn gen_value<T: RandomValue>(&mut self) -> T {
        T::gen_value(self)
    }
}

macro_rules! impl_random_value_int {
    ($($t:ty),*) => {
        $(
            impl RandomValue for $t {
                #[inline]
                fn gen_value(rng: &mut Rng) -> Self {
                    rng.next_u64() as $t
                }
            }
        )*
    };
}

impl_random_value_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl RandomValue for f32 {
    #[inline]
    fn gen_value(rng: &mut Rng) -> Self {
        rng.next_f64() as f32
    }
}

impl RandomValue for f64 {
    #[inline]
    fn gen_value(rng: &mut Rng) -> Self {
        rng.next_f64()
    }
}
