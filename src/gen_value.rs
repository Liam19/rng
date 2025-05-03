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

impl_random_value_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

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

impl RandomValue for char {
    #[inline]
    fn gen_value(rng: &mut Rng) -> Self {
        (0x20u8 + (rng.gen_value::<f32>() * 96.0) as u8) as char
    }
}
impl RandomValue for String {
    #[inline]
    fn gen_value(rng: &mut Rng) -> Self {
        // Arbitrary length limit of 256
        let len = rng.gen_range(0..256);
        let mut s = String::new();

        for _ in 0..len {
            s.push(rng.gen_value());
        }

        s
    }
}
