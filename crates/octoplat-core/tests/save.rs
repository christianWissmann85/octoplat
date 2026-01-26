//! Integration tests for save system

use octoplat_core::save::{EndlessRun, SaveData, SaveManager};

// =============================================================================
// SaveData Tests
// =============================================================================

#[test]
fn test_save_data_new() {
    let data = SaveData::new();

    // Should have sensible defaults
    assert!(data.sfx_volume > 0.0 && data.sfx_volume <= 1.0);
    assert!(data.music_volume >= 0.0 && data.music_volume <= 1.0);
    assert!(data.screen_shake_enabled);
    assert!(data.levels_completed.is_empty());
    assert!(data.endless_runs.is_empty());
}

#[test]
fn test_save_data_default() {
    let data = SaveData::default();

    // Default should be usable
    assert!(data.total_deaths == 0);
    assert!(data.total_gems == 0);
}

#[test]
fn test_save_data_complete_level() {
    let mut data = SaveData::new();

    data.complete_level("test_level", 30.0, 5);

    assert!(data.levels_completed.contains("test_level"));
    assert_eq!(data.get_best_time("test_level"), Some(30.0));
    assert_eq!(data.get_best_gems("test_level"), Some(5));
}

#[test]
fn test_save_data_complete_level_updates_best() {
    let mut data = SaveData::new();

    // First completion
    data.complete_level("test_level", 30.0, 5);

    // Better time, same gems
    data.complete_level("test_level", 25.0, 5);
    assert_eq!(data.get_best_time("test_level"), Some(25.0));

    // Worse time, better gems
    data.complete_level("test_level", 40.0, 10);
    assert_eq!(data.get_best_time("test_level"), Some(25.0)); // Still best time
    assert_eq!(data.get_best_gems("test_level"), Some(10)); // Updated gems
}

#[test]
fn test_save_data_get_best_time_missing() {
    let data = SaveData::new();
    assert!(data.get_best_time("nonexistent").is_none());
}

#[test]
fn test_save_data_get_best_gems_missing() {
    let data = SaveData::new();
    assert!(data.get_best_gems("nonexistent").is_none());
}

#[test]
fn test_save_data_record_endless_run() {
    let mut data = SaveData::new();

    let run = EndlessRun {
        seed: 12345,
        levels_completed: 5,
        gems_collected: 50,
        deaths: 2,
        time: 120.0,
        timestamp: 1000,
    };

    data.record_endless_run(run);

    assert_eq!(data.endless_best_levels, 5);
    assert_eq!(data.endless_best_gems, 50);
    assert_eq!(data.endless_runs.len(), 1);
}

#[test]
fn test_save_data_record_endless_run_updates_bests() {
    let mut data = SaveData::new();

    // First run
    let run1 = EndlessRun {
        seed: 12345,
        levels_completed: 5,
        gems_collected: 50,
        deaths: 2,
        time: 120.0,
        timestamp: 1000,
    };
    data.record_endless_run(run1);

    // Better run
    let run2 = EndlessRun {
        seed: 54321,
        levels_completed: 10,
        gems_collected: 100,
        deaths: 3,
        time: 200.0,
        timestamp: 2000,
    };
    data.record_endless_run(run2);

    assert_eq!(data.endless_best_levels, 10);
    assert_eq!(data.endless_best_gems, 100);
    assert_eq!(data.endless_runs.len(), 2);
}

#[test]
fn test_save_data_endless_runs_sorted() {
    let mut data = SaveData::new();

    // Add runs in random order
    for i in 0..5 {
        let run = EndlessRun {
            seed: i as u64,
            levels_completed: i,
            gems_collected: i * 10,
            deaths: 1,
            time: 60.0,
            timestamp: i as u64 * 1000,
        };
        data.record_endless_run(run);
    }

    // Should be sorted by levels_completed (descending)
    for i in 1..data.endless_runs.len() {
        assert!(
            data.endless_runs[i - 1].levels_completed >= data.endless_runs[i].levels_completed,
            "Runs should be sorted by levels completed"
        );
    }
}

#[test]
fn test_save_data_endless_runs_truncated() {
    let mut data = SaveData::new();

    // Add more than 10 runs
    for i in 0..15 {
        let run = EndlessRun {
            seed: i as u64,
            levels_completed: i,
            gems_collected: i * 10,
            deaths: 1,
            time: 60.0,
            timestamp: i as u64 * 1000,
        };
        data.record_endless_run(run);
    }

    // Should only keep top 10
    assert_eq!(data.endless_runs.len(), 10);

    // Should keep the best runs (highest levels_completed)
    assert!(data.endless_runs.iter().all(|r| r.levels_completed >= 5));
}

#[test]
fn test_save_data_clone() {
    let mut data = SaveData::new();
    data.complete_level("test", 30.0, 5);
    data.total_deaths = 10;

    let cloned = data.clone();

    assert_eq!(data.total_deaths, cloned.total_deaths);
    assert!(cloned.levels_completed.contains("test"));
}

#[test]
fn test_save_data_serialization() {
    let mut data = SaveData::new();
    data.complete_level("test_level", 30.0, 5);
    data.total_deaths = 10;
    data.sfx_volume = 0.8;

    let json = serde_json::to_string(&data).expect("serialize");
    let loaded: SaveData = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(data.total_deaths, loaded.total_deaths);
    assert_eq!(data.sfx_volume, loaded.sfx_volume);
    assert!(loaded.levels_completed.contains("test_level"));
}

// =============================================================================
// EndlessRun Tests
// =============================================================================

#[test]
fn test_endless_run_creation() {
    let run = EndlessRun {
        seed: 12345,
        levels_completed: 5,
        gems_collected: 50,
        deaths: 2,
        time: 120.0,
        timestamp: 1000,
    };

    assert_eq!(run.seed, 12345);
    assert_eq!(run.levels_completed, 5);
    assert_eq!(run.gems_collected, 50);
    assert_eq!(run.deaths, 2);
    assert_eq!(run.time, 120.0);
    assert_eq!(run.timestamp, 1000);
}

#[test]
fn test_endless_run_clone() {
    let run = EndlessRun {
        seed: 12345,
        levels_completed: 5,
        gems_collected: 50,
        deaths: 2,
        time: 120.0,
        timestamp: 1000,
    };

    let cloned = run.clone();

    assert_eq!(run.seed, cloned.seed);
    assert_eq!(run.levels_completed, cloned.levels_completed);
}

#[test]
fn test_endless_run_serialization() {
    let run = EndlessRun {
        seed: 12345,
        levels_completed: 5,
        gems_collected: 50,
        deaths: 2,
        time: 120.0,
        timestamp: 1000,
    };

    let json = serde_json::to_string(&run).expect("serialize");
    let loaded: EndlessRun = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(run.seed, loaded.seed);
    assert_eq!(run.levels_completed, loaded.levels_completed);
    assert_eq!(run.gems_collected, loaded.gems_collected);
}

// =============================================================================
// SaveManager Tests
// =============================================================================

#[test]
fn test_save_manager_creation() {
    // Note: This test might create a save file in the default location
    // In a CI environment, this might need special handling
    let manager = SaveManager::new();
    let _ = manager;
}

#[test]
fn test_save_manager_data_access() {
    let manager = SaveManager::new();

    // Should be able to access data
    let _levels = &manager.data.levels_completed;
    let _volume = manager.data.sfx_volume;
}

#[test]
fn test_save_manager_data_mut() {
    let mut manager = SaveManager::new();

    // Modify data
    let data = manager.data_mut();
    data.total_deaths += 1;
    let deaths = data.total_deaths;

    // Should persist in manager
    assert_eq!(manager.data.total_deaths, deaths);
}

#[test]
fn test_save_manager_default() {
    let manager = SaveManager::default();

    // Default should work same as new()
    let _ = manager.data.sfx_volume;
}
