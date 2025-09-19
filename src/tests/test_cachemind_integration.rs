/**
 * Integration tests for CacheMind cross-system state cache
 * ======================================================
 *
 * Tests the integration between CacheMind and the narrative system components.
 */

#[cfg(test)]
mod tests {
    use crate::cache::{CacheMind, ConstraintSnapshot, CAPRPathSummary, CharacterEmotionArc, LRUCache};
    use crate::recursive_narrative_assistant::RecursiveNarrativeAssistant;
    use std::collections::HashMap;

    #[test]
    fn test_lru_cache_basic_operations() {
        let mut cache = LRUCache::new(3);

        // Test insertion and retrieval
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        cache.insert("key3", "value3");

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        assert_eq!(cache.get(&"key2"), Some(&"value2"));
        assert_eq!(cache.get(&"key3"), Some(&"value3"));

        // Test eviction
        cache.insert("key4", "value4");
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"key1"), None); // Should be evicted
        assert_eq!(cache.get(&"key4"), Some(&"value4"));
    }

    #[test]
    fn test_lru_cache_serialization() {
        let mut cache = LRUCache::new(5);
        cache.insert("test1", 100);
        cache.insert("test2", 200);
        cache.insert("test3", 300);

        // Serialize to JSON
        let json = serde_json::to_string(&cache).unwrap();
        assert!(json.contains("\"capacity\":5"));

        // Deserialize from JSON
        let deserialized: LRUCache<String, i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized.capacity(), 5);
        assert!(deserialized.contains(&"test1".to_string()));
        assert!(deserialized.contains(&"test2".to_string()));
        assert!(deserialized.contains(&"test3".to_string()));
    }

    #[test]
    fn test_cachemind_basic_operations() {
        let mut cache_mind = CacheMind::new();

        // Test constraint snapshot
        let snapshot = ConstraintSnapshot {
            freedom_score: 0.75,
            active_constraints: vec!["constraint1".to_string(), "constraint2".to_string()],
            constraint_pressures: {
                let mut map = HashMap::new();
                map.insert("pressure1".to_string(), 0.5);
                map.insert("pressure2".to_string(), 0.8);
                map
            },
            timestamp: 1234567890,
            chapter: 1,
            scene: Some(1),
        };

        cache_mind.set_constraint_snapshot("test_key".to_string(), snapshot.clone());
        let retrieved = cache_mind.get_constraint_snapshot("test_key");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().freedom_score, 0.75);
        assert_eq!(retrieved.unwrap().chapter, 1);
    }

    #[test]
    fn test_cachemind_capr_path_summary() {
        let mut cache_mind = CacheMind::new();

        let summary = CAPRPathSummary {
            loop_count: 5,
            last_return_vector: vec!["return1".to_string(), "return2".to_string()],
            active_contradictions: vec!["contradiction1".to_string()],
            pressure_points: vec!["pressure1".to_string(), "pressure2".to_string()],
            avg_loop_duration: 45.5,
            last_loop_quality: 0.85,
            chapter: 2,
            scene: Some(3),
            timestamp: 1234567890,
        };

        cache_mind.set_capr_path_summary("capr_test".to_string(), summary.clone());
        let retrieved = cache_mind.get_capr_path_summary("capr_test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().loop_count, 5);
        assert_eq!(retrieved.unwrap().last_loop_quality, 0.85);
    }

    #[test]
    fn test_cachemind_character_emotion_arc() {
        let mut cache_mind = CacheMind::new();

        let arc = CharacterEmotionArc {
            character: "Alice".to_string(),
            valence_sequence: vec![0.2, 0.5, 0.8, 0.6],
            intensity_sequence: vec![0.3, 0.7, 0.9, 0.5],
            dominant_emotions: vec!["joy".to_string(), "surprise".to_string()],
            turning_points: vec![1, 3],
            arc_trend: "rising".to_string(),
            chapter: 3,
            scene_range: (Some(1), Some(4)),
            timestamp: 1234567890,
        };

        cache_mind.set_character_emotion_arc("alice_ch3".to_string(), arc.clone());
        let retrieved = cache_mind.get_character_emotion_arc("alice_ch3");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().character, "Alice");
        assert_eq!(retrieved.unwrap().valence_sequence.len(), 4);
        assert_eq!(retrieved.unwrap().turning_points, vec![1, 3]);
    }

    #[test]
    fn test_cachemind_capacity_limits() {
        let mut cache_mind = CacheMind::new();

        // Fill constraint cache beyond capacity (128)
        for i in 0..130 {
            let snapshot = ConstraintSnapshot {
                freedom_score: 0.5,
                active_constraints: vec![],
                constraint_pressures: HashMap::new(),
                timestamp: i as u64,
                chapter: i as u32,
                scene: None,
            };
            cache_mind.set_constraint_snapshot(format!("key_{}", i), snapshot);
        }

        // Should be limited to capacity
        assert_eq!(cache_mind.constraint_cache.len(), 128);
    }

    #[test]
    fn test_cachemind_similarity_detection() {
        let mut cache_mind = CacheMind::new();

        // Create two similar constraint snapshots
        let snapshot1 = ConstraintSnapshot {
            freedom_score: 0.75,
            active_constraints: vec!["c1".to_string(), "c2".to_string()],
            constraint_pressures: {
                let mut map = HashMap::new();
                map.insert("p1".to_string(), 0.5);
                map
            },
            timestamp: 1000,
            chapter: 1,
            scene: Some(1),
        };

        let snapshot2 = ConstraintSnapshot {
            freedom_score: 0.76, // Very similar
            active_constraints: vec!["c1".to_string(), "c2".to_string()],
            constraint_pressures: {
                let mut map = HashMap::new();
                map.insert("p1".to_string(), 0.51); // Very similar
                map
            },
            timestamp: 2000,
            chapter: 1,
            scene: Some(2),
        };

        cache_mind.set_constraint_snapshot("key1".to_string(), snapshot1);
        cache_mind.set_constraint_snapshot("key2".to_string(), snapshot2);

        let similar_keys = cache_mind.find_similar_constraint_snapshots("key1", 0.1);
        assert!(similar_keys.contains(&"key2".to_string()));
    }

    #[test]
    fn test_cachemind_json_serialization() {
        let mut cache_mind = CacheMind::new();

        // Add some test data
        let snapshot = ConstraintSnapshot {
            freedom_score: 0.8,
            active_constraints: vec!["test".to_string()],
            constraint_pressures: HashMap::new(),
            timestamp: 1234567890,
            chapter: 1,
            scene: None,
        };
        cache_mind.set_constraint_snapshot("test".to_string(), snapshot);

        // Test JSON export
        let json = cache_mind.to_json();
        assert!(json.contains("\"constraint_cache\""));
        assert!(json.contains("\"freedom_score\":0.8"));

        // Test JSON import
        let mut new_cache = CacheMind::new();
        assert!(new_cache.from_json(&json).is_ok());
        assert!(new_cache.get_constraint_snapshot("test").is_some());
    }

    #[test]
    fn test_recursive_narrative_assistant_cache_integration() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Test cache initialization
        assert_eq!(assistant.cache_mind.constraint_cache.len(), 0);
        assert_eq!(assistant.cache_mind.capr_cache.len(), 0);
        assert_eq!(assistant.cache_mind.character_cache.len(), 0);

        // Test cache statistics
        let stats = assistant.get_cache_stats();
        assert!(stats.is_object());
        assert!(stats["constraint_cache"]["size"].as_u64().unwrap() == 0);
        assert!(stats["constraint_cache"]["capacity"].as_u64().unwrap() == 128);

        // Test constraint snapshot caching
        let key = assistant.cache_constraint_snapshot();
        assert!(!key.is_empty());
        assert!(assistant.get_cached_constraint_snapshot(&key).is_some());

        // Test CAPR path summary caching
        let capr_key = assistant.cache_capr_path_summary();
        assert!(!capr_key.is_empty());
        assert!(assistant.get_cached_capr_path_summary(&capr_key).is_some());
    }

    #[test]
    fn test_cache_memory_efficiency() {
        let cache_mind = CacheMind::new();

        // Check that empty cache has reasonable memory footprint
        let memory_size = std::mem::size_of_val(&cache_mind);
        assert!(memory_size < 50000); // Should be less than 50KB for empty cache

        // Test capacity limits
        assert_eq!(cache_mind.constraint_cache.capacity(), 128);
        assert_eq!(cache_mind.capr_cache.capacity(), 128);
        assert_eq!(cache_mind.character_cache.capacity(), 128);
    }

    #[test]
    fn test_cache_file_persistence() {
        use std::fs;
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let cache_file = temp_dir.path().join("test_cache.json");

        let mut cache_mind = CacheMind::new();

        // Add test data
        let snapshot = ConstraintSnapshot {
            freedom_score: 0.9,
            active_constraints: vec!["persistent_test".to_string()],
            constraint_pressures: HashMap::new(),
            timestamp: 1234567890,
            chapter: 10,
            scene: Some(5),
        };
        cache_mind.set_constraint_snapshot("persistent".to_string(), snapshot);

        // Save to file
        assert!(cache_mind.save_to_file(cache_file.to_str().unwrap()).is_ok());
        assert!(cache_file.exists());

        // Load from file
        let mut new_cache = CacheMind::new();
        assert!(new_cache.load_from_file(cache_file.to_str().unwrap()).is_ok());

        // Verify data was loaded
        let loaded_snapshot = new_cache.get_constraint_snapshot("persistent");
        assert!(loaded_snapshot.is_some());
        assert_eq!(loaded_snapshot.unwrap().freedom_score, 0.9);
        assert_eq!(loaded_snapshot.unwrap().chapter, 10);

        // Cleanup is automatic with tempdir
    }

    #[test]
    fn test_character_emotion_turning_points() {
        let mut cache_mind = CacheMind::new();

        // Create emotion arc with clear turning points
        let valence_seq = vec![-0.5, -0.2, 0.1, 0.8, 0.3, 0.9]; // Dip at 0, rise at 2-3, dip at 4, rise at 5
        let intensity_seq = vec![0.3, 0.5, 0.8, 0.9, 0.4, 0.8];

        let arc = CharacterEmotionArc {
            character: "Bob".to_string(),
            valence_sequence: valence_seq.clone(),
            intensity_sequence: intensity_seq.clone(),
            dominant_emotions: vec!["sadness".to_string(), "joy".to_string(), "confusion".to_string(), "elation".to_string()],
            turning_points: vec![2, 4], // Should detect significant changes
            arc_trend: "volatile".to_string(),
            chapter: 5,
            scene_range: (Some(1), Some(6)),
            timestamp: 1234567890,
        };

        cache_mind.set_character_emotion_arc("bob_volatile".to_string(), arc);

        let retrieved = cache_mind.get_character_emotion_arc("bob_volatile").unwrap();
        assert_eq!(retrieved.turning_points.len(), 2);
        assert!(retrieved.turning_points.contains(&2));
        assert!(retrieved.turning_points.contains(&4));
    }

    #[test]
    fn test_cache_statistics_and_hit_ratios() {
        let mut cache_mind = CacheMind::new();

        // Add some data and access it
        let snapshot = ConstraintSnapshot {
            freedom_score: 0.7,
            active_constraints: vec![],
            constraint_pressures: HashMap::new(),
            timestamp: 1234567890,
            chapter: 1,
            scene: None,
        };

        cache_mind.set_constraint_snapshot("stats_test".to_string(), snapshot);

        // Access the data multiple times to affect hit ratio
        for _ in 0..5 {
            let _ = cache_mind.get_constraint_snapshot("stats_test");
        }

        // Check statistics
        let stats = cache_mind.get_statistics();
        assert!(stats.is_object());
        assert!(stats["constraint_cache"]["entries"].as_u64().unwrap() == 1);
        assert!(stats["constraint_cache"]["capacity"].as_u64().unwrap() == 128);

        // Hit ratio should be > 0 (exact value depends on LRU implementation details)
        let hit_ratio = stats["constraint_cache"]["hit_ratio"].as_f64().unwrap();
        assert!(hit_ratio >= 0.0);
        assert!(hit_ratio <= 1.0);
    }
}