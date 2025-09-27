use crate::*;

use rng_derive::RandomVariant;

pub trait RandomVariant {
    fn random_variant(rng: &mut Rng) -> Self;
}

#[derive(Debug, RandomVariant)]
enum TestEnum {
    One,
    Two,
    Three,
}

#[test]
fn test_random_variant() {
    let rng = Rng::new();

    for _ in 0..16 {
        dbg!(TestEnum::random_variant(rng));
    }
}
