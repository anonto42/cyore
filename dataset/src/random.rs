// The xorshift RNG from `mnist`, plus helpers for sampling. Seeded, so the same seed
// always produces the same dataset.

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        // xorshift with state 0 would output 0 forever, so swap in a fixed non-zero constant.
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

    /// Uniform in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform integer in [0, n). `n` must be > 0.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    /// A uniformly chosen element. `items` must not be empty.
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    /// True with probability `p`.
    pub fn chance(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// Fisher–Yates shuffle.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
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
        let mut rng = Rng::new(0);
        assert!((0..10).any(|_| rng.next_u64() != 0));
    }

    #[test]
    fn test_chance_extremes() {
        let mut rng = Rng::new(3);
        for _ in 0..1000 {
            assert!(!rng.chance(0.0));
            assert!(rng.chance(1.0));
        }
    }

    #[test]
    fn test_choose_and_below_in_range() {
        let mut rng = Rng::new(5);
        let items = ["a", "b", "c"];
        for _ in 0..1000 {
            assert!(rng.below(7) < 7);
            assert!(items.contains(rng.choose(&items)));
        }
    }

    #[test]
    fn test_shuffle_is_permutation() {
        let original: Vec<usize> = (0..50).collect();
        let mut shuffled = original.clone();
        Rng::new(1).shuffle(&mut shuffled);
        assert_ne!(shuffled, original);
        shuffled.sort_unstable();
        assert_eq!(shuffled, original);
    }
}
