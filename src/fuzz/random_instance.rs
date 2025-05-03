use crate::Rng;

#[macro_export]
macro_rules! generate_random_instance {
    ($ty:ty) => {{
        let estimated_size = mem::size_of::<$ty>() * 1; // Estimate based on type size.
        let rng = Rng::new();
        let mut raw = vec![0u8; estimated_size];
        rng.fill_bytes(&mut raw);

        let mut unstructured = Unstructured::new(&raw);

        <$ty as Arbitrary>::arbitrary(&mut unstructured)
            .expect("Failed to generate random instance. Increase buffer size if needed.")
    }};
}

pub trait RandomInstance {
    fn random_instance(rng: &mut Rng) -> Self;
}
