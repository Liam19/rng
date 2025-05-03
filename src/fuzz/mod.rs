use crate::Rng;

mod impl_collections;
mod impl_primitives;

#[allow(unused_imports)]
pub use impl_collections::*;
#[allow(unused_imports)]
pub use impl_primitives::*;

pub trait RandomInstance {
    fn random_instance(rng: &mut Rng) -> Self;
}
