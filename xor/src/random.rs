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
