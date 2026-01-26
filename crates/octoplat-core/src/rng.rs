//! Centralized PCG-XSH-RR Random Number Generator
//!
//! A high-quality, deterministic RNG implementation using the PCG (Permuted Congruential
//! Generator) algorithm with XOR-Shift and Random Rotate output function.
//!
//! Key features:
//! - 64-bit state, 32-bit output (PCG-XSH-RR variant)
//! - Deterministic and reproducible given the same seed
//! - No modulo bias in bounded random generation (Lemire's method)
//! - Float generation uses upper bits for better quality
//! - Cumulative sum approach for weighted selection (avoids float accumulation errors)

/// PCG-XSH-RR random number generator.
///
/// This implementation provides high-quality random numbers with excellent
/// statistical properties while remaining fast and simple.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
    inc: u64,
}

// PCG multiplier and default increment (from the PCG paper)
const PCG_MULTIPLIER: u64 = 6364136223846793005;
const PCG_DEFAULT_INCREMENT: u64 = 1442695040888963407;

impl Rng {
    /// Create a new RNG with the given seed.
    ///
    /// The seed is used to initialize the internal state. Different seeds
    /// produce different random sequences.
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: PCG_DEFAULT_INCREMENT,
        };
        // Advance state once and add seed
        rng.state = rng.state.wrapping_add(seed);
        rng.advance();
        rng
    }

    /// Create a new RNG with a seed and stream (increment).
    ///
    /// Different streams produce independent random sequences even with the same seed.
    /// Useful for generating multiple independent random sequences.
    pub fn with_stream(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            // Increment must be odd
            inc: (stream << 1) | 1,
        };
        rng.state = rng.state.wrapping_add(seed);
        rng.advance();
        rng
    }

    /// Advance the internal state (LCG step)
    #[inline]
    fn advance(&mut self) {
        self.state = self.state.wrapping_mul(PCG_MULTIPLIER).wrapping_add(self.inc);
    }

    /// Generate the next 32-bit random value using XOR-Shift and Random Rotate.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.advance();

        // PCG-XSH-RR output function
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Generate the next 64-bit random value.
    ///
    /// Combines two 32-bit outputs for full 64-bit coverage.
    pub fn next_u64(&mut self) -> u64 {
        let high = self.next_u32() as u64;
        let low = self.next_u32() as u64;
        (high << 32) | low
    }

    /// Generate a random float in the range [0, 1).
    ///
    /// Uses the upper 24 bits of a 32-bit value for better distribution
    /// (floats only have 23 bits of mantissa precision).
    #[inline]
    pub fn next_float(&mut self) -> f32 {
        // Use upper 24 bits divided by 2^24
        (self.next_u32() >> 8) as f32 / 16777216.0
    }

    /// Generate a random float in the range [0, 1) with 64-bit precision.
    pub fn next_f64(&mut self) -> f64 {
        // Use upper 53 bits divided by 2^53
        (self.next_u64() >> 11) as f64 / 9007199254740992.0
    }

    /// Generate a bounded random number in [0, bound) without modulo bias.
    ///
    /// Uses Lemire's nearly divisionless method for unbiased generation.
    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        if bound == 0 {
            return 0;
        }

        // Lemire's method - reject only the few values that would cause bias
        let threshold = bound.wrapping_neg() % bound;

        loop {
            let r = self.next_u32();
            let m = (r as u64).wrapping_mul(bound as u64);

            if (m as u32) >= threshold {
                return (m >> 32) as u32;
            }
        }
    }

    /// Generate a random integer in the range [min, max] (inclusive).
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        if max <= min {
            return min;
        }
        let range = (max - min + 1) as u32;
        min + self.next_bounded(range) as i32
    }

    /// Generate a random usize in the range [min, max) (exclusive upper bound).
    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        if max <= min {
            return min;
        }
        let range = (max - min) as u32;
        min + self.next_bounded(range) as usize
    }

    /// Choose a random element from a slice.
    ///
    /// Returns `None` if the slice is empty.
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let idx = self.next_bounded(items.len() as u32) as usize;
            Some(&items[idx])
        }
    }

    /// Choose a random element from a slice, returning the index.
    ///
    /// Returns `None` if the slice is empty.
    pub fn choose_index<T>(&mut self, items: &[T]) -> Option<usize> {
        if items.is_empty() {
            None
        } else {
            Some(self.next_bounded(items.len() as u32) as usize)
        }
    }

    /// Choose a random element with weighted probabilities.
    ///
    /// Uses cumulative sum approach to avoid float accumulation errors.
    /// Weights should be non-negative. Zero-weight items will never be chosen.
    ///
    /// Returns `None` if items is empty or all weights are zero.
    pub fn weighted_choose<'a, T>(&mut self, items: &'a [(T, f32)]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }

        // Calculate total weight (using Kahan summation for precision)
        let mut total = 0.0f64;
        let mut compensation = 0.0f64;
        for (_, weight) in items {
            let w = *weight as f64;
            let y = w - compensation;
            let t = total + y;
            compensation = (t - total) - y;
            total = t;
        }

        if total <= 0.0 {
            return None;
        }

        // Generate random value in [0, total)
        let target = self.next_f64() * total;

        // Find the item using cumulative sum
        let mut cumulative = 0.0f64;
        for (item, weight) in items {
            cumulative += *weight as f64;
            if target < cumulative {
                return Some(item);
            }
        }

        // Fallback to last item (handles floating point edge cases)
        items.last().map(|(item, _)| item)
    }

    /// Shuffle a slice in-place using Fisher-Yates algorithm.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        let len = items.len();
        if len <= 1 {
            return;
        }

        for i in (1..len).rev() {
            let j = self.next_bounded((i + 1) as u32) as usize;
            items.swap(i, j);
        }
    }

    /// Return `true` with the given probability (0.0 to 1.0).
    ///
    /// `chance(0.0)` always returns `false`.
    /// `chance(1.0)` always returns `true`.
    #[inline]
    pub fn chance(&mut self, probability: f32) -> bool {
        self.next_float() < probability
    }

    /// Return `true` with probability 1/n (e.g., one_in(6) simulates a d6 roll of 1).
    #[inline]
    pub fn one_in(&mut self, n: u32) -> bool {
        n > 0 && self.next_bounded(n) == 0
    }

    /// Generate a random value following a roughly normal distribution.
    ///
    /// Uses Irwin-Hall approximation (sum of uniform values) for speed.
    /// Returns value roughly in range [0, 1] with mean 0.5.
    pub fn normal_approx(&mut self) -> f32 {
        // Sum of 4 uniform values, normalized
        (self.next_float() + self.next_float() + self.next_float() + self.next_float()) / 4.0
    }

    /// Fork the RNG into a new independent stream.
    ///
    /// Useful for creating separate RNG instances for subsystems while
    /// maintaining reproducibility.
    pub fn fork(&mut self) -> Rng {
        let seed = self.next_u64();
        let stream = self.next_u64();
        Rng::with_stream(seed, stream)
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u32(), rng2.next_u32());
        }
    }

    #[test]
    fn test_different_seeds() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(54321);

        // Should produce different sequences
        let vals1: Vec<u32> = (0..10).map(|_| rng1.next_u32()).collect();
        let vals2: Vec<u32> = (0..10).map(|_| rng2.next_u32()).collect();
        assert_ne!(vals1, vals2);
    }

    #[test]
    fn test_float_range() {
        let mut rng = Rng::new(42);
        for _ in 0..1000 {
            let f = rng.next_float();
            assert!(f >= 0.0 && f < 1.0, "Float {} out of range", f);
        }
    }

    #[test]
    fn test_bounded_no_bias() {
        let mut rng = Rng::new(42);
        let bound = 7u32;
        let mut counts = [0u32; 7];

        // Generate many samples
        for _ in 0..70000 {
            let val = rng.next_bounded(bound);
            assert!(val < bound);
            counts[val as usize] += 1;
        }

        // Check roughly uniform distribution (within 15% of expected)
        let expected = 10000.0;
        for (i, &count) in counts.iter().enumerate() {
            let ratio = count as f64 / expected;
            assert!(
                ratio > 0.85 && ratio < 1.15,
                "Bucket {} has {} samples, expected ~{}", i, count, expected
            );
        }
    }

    #[test]
    fn test_range() {
        let mut rng = Rng::new(42);
        for _ in 0..1000 {
            let val = rng.range(-10, 10);
            assert!(val >= -10 && val <= 10);
        }
    }

    #[test]
    fn test_choose() {
        let mut rng = Rng::new(42);
        let items = [1, 2, 3, 4, 5];

        for _ in 0..100 {
            let chosen = rng.choose(&items);
            assert!(chosen.is_some());
            assert!(items.contains(chosen.unwrap()));
        }

        let empty: [i32; 0] = [];
        assert!(rng.choose(&empty).is_none());
    }

    #[test]
    fn test_weighted_choose() {
        let mut rng = Rng::new(42);
        let items = [('a', 1.0), ('b', 3.0), ('c', 1.0)];

        let mut counts = std::collections::HashMap::new();
        for _ in 0..5000 {
            let chosen = rng.weighted_choose(&items).unwrap();
            *counts.entry(*chosen).or_insert(0) += 1;
        }

        // 'b' should be chosen about 3x as often as 'a' or 'c'
        let a = *counts.get(&'a').unwrap_or(&0) as f64;
        let b = *counts.get(&'b').unwrap_or(&0) as f64;
        let c = *counts.get(&'c').unwrap_or(&0) as f64;

        let b_to_a = b / a;
        let b_to_c = b / c;

        assert!(b_to_a > 2.5 && b_to_a < 3.5, "b/a ratio: {}", b_to_a);
        assert!(b_to_c > 2.5 && b_to_c < 3.5, "b/c ratio: {}", b_to_c);
    }

    #[test]
    fn test_shuffle() {
        let mut rng = Rng::new(42);
        let mut items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let original = items;

        rng.shuffle(&mut items);

        // Should contain same elements
        let mut sorted = items;
        sorted.sort();
        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        // Likely to be in different order (very small chance of being same)
        assert_ne!(items, original);
    }

    #[test]
    fn test_chance() {
        let mut rng = Rng::new(42);

        // chance(0) should always be false
        for _ in 0..100 {
            assert!(!rng.chance(0.0));
        }

        // chance(1) should always be true
        for _ in 0..100 {
            assert!(rng.chance(1.0));
        }

        // chance(0.5) should be roughly 50/50
        let mut count = 0;
        for _ in 0..10000 {
            if rng.chance(0.5) {
                count += 1;
            }
        }
        assert!(count > 4500 && count < 5500, "count: {}", count);
    }

    #[test]
    fn test_fork() {
        let mut rng = Rng::new(42);
        let mut fork1 = rng.fork();
        let mut fork2 = rng.fork();

        // Forks should produce different sequences
        let vals1: Vec<u32> = (0..10).map(|_| fork1.next_u32()).collect();
        let vals2: Vec<u32> = (0..10).map(|_| fork2.next_u32()).collect();
        assert_ne!(vals1, vals2);
    }
}
