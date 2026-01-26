//! Integration tests for random number generation

use octoplat_core::rng::Rng;

// =============================================================================
// Rng Tests
// =============================================================================

#[test]
fn test_rng_creation() {
    let rng = Rng::new(12345);
    // Should create without panic
    let _ = rng;
}

#[test]
fn test_rng_deterministic() {
    let mut rng1 = Rng::new(12345);
    let mut rng2 = Rng::new(12345);

    // Same seed should produce same sequence
    for _ in 0..10 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn test_rng_different_seeds() {
    let mut rng1 = Rng::new(12345);
    let mut rng2 = Rng::new(54321);

    // Different seeds should (almost certainly) produce different values
    let mut same_count = 0;
    for _ in 0..100 {
        if rng1.next_u64() == rng2.next_u64() {
            same_count += 1;
        }
    }

    // It's extremely unlikely to have many collisions
    assert!(same_count < 5, "Different seeds produced too many same values");
}

#[test]
fn test_rng_next_float() {
    let mut rng = Rng::new(12345);

    for _ in 0..100 {
        let val = rng.next_float();
        assert!(val >= 0.0 && val < 1.0, "next_float should be in [0, 1): {}", val);
    }
}

#[test]
fn test_rng_range() {
    let mut rng = Rng::new(12345);

    for _ in 0..100 {
        let val = rng.range(10, 20);
        assert!(val >= 10 && val <= 20, "range should be in [10, 20]: {}", val);
    }
}

#[test]
fn test_rng_range_usize() {
    let mut rng = Rng::new(12345);

    for _ in 0..100 {
        let val = rng.range_usize(0, 10);
        assert!(val < 10, "range_usize should be in [0, 10): {}", val);
    }
}

#[test]
fn test_rng_chance() {
    let mut rng = Rng::new(12345);

    // 0% chance should always return false
    let mut any_true = false;
    for _ in 0..100 {
        if rng.chance(0.0) {
            any_true = true;
        }
    }
    assert!(!any_true, "0% chance should never succeed");

    // 100% chance should always return true
    let mut any_false = false;
    for _ in 0..100 {
        if !rng.chance(1.0) {
            any_false = true;
        }
    }
    assert!(!any_false, "100% chance should always succeed");
}

#[test]
fn test_rng_chance_distribution() {
    let mut rng = Rng::new(12345);

    // 50% chance should be roughly balanced
    let mut true_count = 0;
    let iterations = 1000;
    for _ in 0..iterations {
        if rng.chance(0.5) {
            true_count += 1;
        }
    }

    // Should be roughly 50% (within 10%)
    let ratio = true_count as f32 / iterations as f32;
    assert!(
        ratio > 0.4 && ratio < 0.6,
        "50% chance ratio should be near 0.5, got {}",
        ratio
    );
}

#[test]
fn test_rng_choose() {
    let mut rng = Rng::new(12345);
    let items = vec!["a", "b", "c", "d"];

    for _ in 0..100 {
        let chosen = rng.choose(&items);
        assert!(chosen.is_some());
        assert!(items.contains(chosen.unwrap()));
    }
}

#[test]
fn test_rng_choose_empty() {
    let mut rng = Rng::new(12345);
    let items: Vec<&str> = vec![];

    let chosen = rng.choose(&items);
    assert!(chosen.is_none());
}

#[test]
fn test_rng_choose_distribution() {
    let mut rng = Rng::new(12345);
    let items = vec![0, 1, 2, 3];
    let mut counts = [0; 4];

    let iterations = 1000;
    for _ in 0..iterations {
        let chosen = *rng.choose(&items).unwrap();
        counts[chosen] += 1;
    }

    // Each item should be chosen roughly 25% of the time
    for count in counts {
        let ratio = count as f32 / iterations as f32;
        assert!(
            ratio > 0.15 && ratio < 0.35,
            "Distribution should be roughly uniform, got {}",
            ratio
        );
    }
}

#[test]
fn test_rng_shuffle() {
    let mut rng = Rng::new(12345);
    let mut items = vec![1, 2, 3, 4, 5];
    let original = items.clone();

    rng.shuffle(&mut items);

    // Shuffled array should have same elements
    items.sort();
    assert_eq!(items, vec![1, 2, 3, 4, 5]);

    // Re-shuffle from fresh state
    let mut items = original.clone();
    let mut rng = Rng::new(12345);
    rng.shuffle(&mut items);

    // Should be different from original (extremely likely for 5 elements)
    // Note: There's a 1/120 chance this could fail
    assert_ne!(items, original, "Shuffle should change order (usually)");
}

#[test]
fn test_rng_weighted_choose() {
    let mut rng = Rng::new(12345);
    let items = vec![
        ("rare", 1.0),
        ("common", 99.0),
    ];

    let mut common_count = 0;
    let iterations = 1000;
    for _ in 0..iterations {
        let name = *rng.weighted_choose(&items).unwrap();
        if name == "common" {
            common_count += 1;
        }
    }

    // "common" should be chosen most of the time
    let ratio = common_count as f32 / iterations as f32;
    assert!(ratio > 0.9, "Weighted choice should favor common item, got {}", ratio);
}

#[test]
fn test_rng_weighted_choose_empty() {
    let mut rng = Rng::new(12345);
    let items: Vec<(&str, f32)> = vec![];

    let chosen = rng.weighted_choose(&items);
    assert!(chosen.is_none());
}

#[test]
fn test_rng_reproducible_sequence() {
    let seed = 98765u64;

    // Generate a sequence
    let mut rng = Rng::new(seed);
    let sequence: Vec<u64> = (0..20).map(|_| rng.next_u64()).collect();

    // Regenerate with same seed
    let mut rng2 = Rng::new(seed);
    let sequence2: Vec<u64> = (0..20).map(|_| rng2.next_u64()).collect();

    assert_eq!(sequence, sequence2, "Same seed should produce same sequence");
}

#[test]
fn test_rng_next_u32() {
    let mut rng = Rng::new(12345);
    // Just verify it doesn't panic and returns values
    let v1 = rng.next_u32();
    let v2 = rng.next_u32();
    assert_ne!(v1, v2, "Consecutive u32 values should differ");
}

#[test]
fn test_rng_next_bounded() {
    let mut rng = Rng::new(12345);

    for _ in 0..100 {
        let val = rng.next_bounded(10);
        assert!(val < 10, "next_bounded(10) should be < 10, got {}", val);
    }
}

#[test]
fn test_rng_one_in() {
    let mut rng = Rng::new(12345);

    // one_in(1) should always return true
    for _ in 0..100 {
        assert!(rng.one_in(1), "one_in(1) should always be true");
    }

    // one_in(0) should return false (edge case)
    assert!(!rng.one_in(0), "one_in(0) should be false");
}

#[test]
fn test_rng_with_stream() {
    let rng1 = Rng::with_stream(12345, 1);
    let rng2 = Rng::with_stream(12345, 2);

    // Different streams should produce different sequences
    let mut rng1 = rng1;
    let mut rng2 = rng2;

    let mut same_count = 0;
    for _ in 0..100 {
        if rng1.next_u64() == rng2.next_u64() {
            same_count += 1;
        }
    }

    assert!(same_count < 5, "Different streams should produce different values");
}

#[test]
fn test_rng_fork() {
    let mut rng = Rng::new(12345);
    let mut forked = rng.fork();

    // Forked RNG should be independent
    let v1 = rng.next_u64();
    let v2 = forked.next_u64();

    // Values will likely be different (not guaranteed but very probable)
    // The forked RNG has a different seed/stream
    let _ = v1;
    let _ = v2;
}

#[test]
fn test_rng_normal_approx() {
    let mut rng = Rng::new(12345);

    for _ in 0..100 {
        let val = rng.normal_approx();
        assert!(val >= 0.0 && val <= 1.0, "normal_approx should be in [0, 1]: {}", val);
    }
}
