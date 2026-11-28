// A tiny random number generator (xorshift)

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 { 0x9E3779B97F4A7C15 } else { seed };
        Rng { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_seed_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn test_zero_seed_is_remapped() {
        // xorshift with state 0 would return 0 forever; the constructor must avoid that.
        let mut rng = Rng::new(0);
        assert!((0..10).any(|_| rng.next_u64() != 0));
    }

    #[test]
    fn test_range_bounds() {
        let mut rng = Rng::new(7);
        for _ in 0..10_000 {
            let x = rng.range(-1.0, 1.0);
            assert!((-1.0..1.0).contains(&x));
        }
    }
}
