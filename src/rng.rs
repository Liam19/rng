use getrandom::fill;

pub struct Rng {
    seed: u64,
}

impl Rng {
    //TODO - look into pre-initialisation like Rand::rng()
    /// Creates a new Rng with a random seed from the OS entropy source.
    ///
    /// Panics if entropy is unavailable (unlikely on modern systems).
    pub fn new() -> Self {
        // 8 bytes = 64 bits
        let mut buf = [0u8; 8];

        fill(&mut buf).expect("Failed to generate random seed");

        Self {
            seed: u64::from_le_bytes(buf),
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    /// Generates the next random `u64`.
    pub(crate) fn next_u64(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);

        let mut result = self.seed;

        result ^= result >> 32;
        result = result.wrapping_mul(self.seed);
        result ^= result >> 32;
        result
    }

    /// Generates a float in the range [0.0, 1.0)
    pub(crate) fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }
}
