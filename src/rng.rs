use core::cell::RefCell;

/// A fast, non-cryptographic random number generator (RNG) based on the WyRand algorithm.
///
/// This RNG is suitable for simulations, games, and randomized algorithms
/// where cryptographic security is not required.
#[derive(Copy, Clone, Debug)]
pub struct Rng {
    seed: u64,
}

thread_local! {
    static THREAD_RNG: RefCell<Rng> = RefCell::new({
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("Failed to seed RNG");
        Rng::from_seed(u64::from_le_bytes(buf))
    });
}

impl Rng {
    /// Returns a mutable reference to the thread-local RNG.
    #[allow(unsafe_code)]
    #[inline]
    pub fn new() -> &'static mut Rng {
        THREAD_RNG.with(|rng| unsafe { &mut *rng.as_ptr() })
    }

    /// Creates a new RNG instance with a fixed seed.
    ///
    /// Useful for deterministic testing or reproducible simulations.
    #[inline]
    pub fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    /// Generates the next random `u64` using the WyRand algorithm.
    ///
    /// This is the core method that drives all other random number generation.
    #[inline]
    pub(crate) fn next_u64(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
        let t = self.seed;
        let s = t ^ t >> 32;
        s.wrapping_mul(t) ^ s >> 32
    }

    /// Generates a random `f64` in the range `[0.0, 1.0)`.
    ///
    /// The value is uniformly distributed across the possible range.
    #[inline]
    pub(crate) fn next_f64(&mut self) -> f64 {
        const F64_DENOM: f64 = 1.0 / (1u64 << 53) as f64;

        (self.next_u64() >> 11) as f64 * F64_DENOM
    }

    /// Returns `true` with a given probability.
    ///
    /// # Arguments
    /// * `probability` - A value between `0.0` (always `false`) and `1.0` (always `true`).
    ///
    /// # Panics
    /// Does not panic, but values outside `[0.0, 1.0]` will clamp to `false` or `true`.
    #[inline]
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }
}
