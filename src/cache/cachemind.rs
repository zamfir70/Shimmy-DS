/**
 * CacheMind: Cross-System State Cache
 * ==================================
 *
 * Implements Claude Code Card #2: cachemind.rs (Cross-System State Cache)
 *
 * Purpose: Provides LRU caching for narrative system states to enable:
 * - 60% faster warm starts
 * - Cross-chapter memory without reanalysis
 * - Meaning reuse across sessions
 * - Future predictive module loading
 */

use super::lru::LRUCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};

/// Snapshot of constraint space state at a specific point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSnapshot {
    /// Freedom score (0.0 to 1.0) indicating narrative flexibility
    pub freedom_score: f32,
    /// List of currently active constraint names
    pub active_constraints: Vec<String>,
    /// Constraint types and their pressure levels
    pub constraint_pressures: HashMap<String, f32>,
    /// Timestamp when snapshot was taken
    pub timestamp: u64,
    /// Chapter and scene context
    pub chapter: u32,
    pub scene: Option<u32>,
}

impl ConstraintSnapshot {
    /// Create new constraint snapshot
    pub fn new(
        freedom_score: f32,
        active_constraints: Vec<String>,
        constraint_pressures: HashMap<String, f32>,
        chapter: u32,
        scene: Option<u32>,
    ) -> Self {
        Self {
            freedom_score,
            active_constraints,
            constraint_pressures,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            chapter,
            scene,
        }
    }

    /// Calculate similarity to another snapshot (0.0 to 1.0)
    pub fn similarity(&self, other: &ConstraintSnapshot) -> f32 {
        let freedom_similarity = 1.0 - (self.freedom_score - other.freedom_score).abs();

        let constraint_overlap = self.active_constraints.iter()
            .filter(|c| other.active_constraints.contains(c))
            .count() as f32;

        let total_constraints = (self.active_constraints.len() + other.active_constraints.len()) as f32;
        let constraint_similarity = if total_constraints > 0.0 {
            (constraint_overlap * 2.0) / total_constraints
        } else {
            1.0
        };

        (freedom_similarity * 0.6 + constraint_similarity * 0.4).max(0.0).min(1.0)
    }
}

/// Summary of CAPR (Contradiction→Action→Pressure→Return) narrative path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAPRPathSummary {
    /// Number of complete CAPR loops detected
    pub loop_count: usize,
    /// The most recent return vector (narrative elements that resolved)
    pub last_return_vector: Vec<String>,
    /// Active contradictions that haven't been resolved
    pub active_contradictions: Vec<String>,
    /// Pressure points that are building narrative tension
    pub pressure_points: Vec<String>,
    /// Average loop completion time (in beats/scenes)
    pub avg_loop_duration: f32,
    /// Quality score of the most recent loop (0.0 to 1.0)
    pub last_loop_quality: f32,
    /// Context information
    pub chapter: u32,
    pub scene: Option<u32>,
    pub timestamp: u64,
}

impl CAPRPathSummary {
    /// Create new CAPR path summary
    pub fn new(
        loop_count: usize,
        last_return_vector: Vec<String>,
        active_contradictions: Vec<String>,
        pressure_points: Vec<String>,
        avg_loop_duration: f32,
        last_loop_quality: f32,
        chapter: u32,
        scene: Option<u32>,
    ) -> Self {
        Self {
            loop_count,
            last_return_vector,
            active_contradictions,
            pressure_points,
            avg_loop_duration,
            last_loop_quality,
            chapter,
            scene,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Get health score based on loop quality and resolution rate
    pub fn health_score(&self) -> f32 {
        let resolution_rate = if self.active_contradictions.is_empty() {
            1.0
        } else {
            1.0 - (self.active_contradictions.len() as f32 * 0.1).min(0.8)
        };

        (self.last_loop_quality * 0.7 + resolution_rate * 0.3).max(0.0).min(1.0)
    }
}

/// Character emotional arc data for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEmotionArc {
    /// Character name
    pub character: String,
    /// Sequence of emotional valence values (-1.0 to 1.0)
    pub valence_sequence: Vec<f32>,
    /// Sequence of emotional intensity values (0.0 to 1.0)
    pub intensity_sequence: Vec<f32>,
    /// Dominant emotions throughout the arc
    pub dominant_emotions: Vec<String>,
    /// Emotional turning points (beat indices where significant changes occurred)
    pub turning_points: Vec<usize>,
    /// Overall arc trend (rising, falling, stable, volatile)
    pub arc_trend: String,
    /// Context information
    pub chapter: u32,
    pub scene_range: (Option<u32>, Option<u32>), // (start_scene, end_scene)
    pub timestamp: u64,
}

impl CharacterEmotionArc {
    /// Create new character emotion arc
    pub fn new(
        character: String,
        valence_sequence: Vec<f32>,
        intensity_sequence: Vec<f32>,
        dominant_emotions: Vec<String>,
        chapter: u32,
        scene_range: (Option<u32>, Option<u32>),
    ) -> Self {
        let turning_points = Self::detect_turning_points(&valence_sequence);
        let arc_trend = Self::analyze_trend(&valence_sequence);

        Self {
            character,
            valence_sequence,
            intensity_sequence,
            dominant_emotions,
            turning_points,
            arc_trend,
            chapter,
            scene_range,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Detect significant emotional turning points
    fn detect_turning_points(valence_sequence: &[f32]) -> Vec<usize> {
        let mut turning_points = Vec::new();

        if valence_sequence.len() < 3 {
            return turning_points;
        }

        for i in 1..valence_sequence.len() - 1 {
            let prev = valence_sequence[i - 1];
            let curr = valence_sequence[i];
            let next = valence_sequence[i + 1];

            // Detect significant direction changes
            let change_threshold = 0.3;
            let prev_trend = curr - prev;
            let next_trend = next - curr;

            if prev_trend.abs() > change_threshold && next_trend.abs() > change_threshold {
                if (prev_trend > 0.0) != (next_trend > 0.0) {
                    turning_points.push(i);
                }
            }
        }

        turning_points
    }

    /// Analyze overall emotional trend
    fn analyze_trend(valence_sequence: &[f32]) -> String {
        if valence_sequence.len() < 2 {
            return "insufficient_data".to_string();
        }

        let start = valence_sequence[0];
        let end = valence_sequence[valence_sequence.len() - 1];
        let overall_change = end - start;

        // Calculate volatility
        let mut changes = Vec::new();
        for i in 1..valence_sequence.len() {
            changes.push((valence_sequence[i] - valence_sequence[i - 1]).abs());
        }
        let avg_volatility = changes.iter().sum::<f32>() / changes.len() as f32;

        if avg_volatility > 0.4 {
            "volatile".to_string()
        } else if overall_change > 0.2 {
            "rising".to_string()
        } else if overall_change < -0.2 {
            "falling".to_string()
        } else {
            "stable".to_string()
        }
    }

    /// Get emotional complexity score
    pub fn complexity_score(&self) -> f32 {
        let valence_range = if let (Some(min), Some(max)) = (
            self.valence_sequence.iter().min_by(|a, b| a.partial_cmp(b).unwrap()),
            self.valence_sequence.iter().max_by(|a, b| a.partial_cmp(b).unwrap()),
        ) {
            max - min
        } else {
            0.0
        };

        let turning_point_density = self.turning_points.len() as f32 / self.valence_sequence.len().max(1) as f32;
        let emotion_diversity = self.dominant_emotions.len() as f32 * 0.1;

        (valence_range * 0.4 + turning_point_density * 0.4 + emotion_diversity * 0.2).min(1.0)
    }
}

/// Cross-system state cache for narrative intelligence components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMind {
    /// Cache for constraint space snapshots
    pub constraint_cache: LRUCache<String, ConstraintSnapshot>,
    /// Cache for CAPR path summaries
    pub capr_cache: LRUCache<String, CAPRPathSummary>,
    /// Cache for character emotion arcs
    pub emotion_cache: LRUCache<String, CharacterEmotionArc>,
    /// Maximum entries per cache
    pub max_entries: usize,
    /// Cache statistics
    pub stats: CacheMindStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMindStats {
    pub constraint_hits: u64,
    pub constraint_misses: u64,
    pub capr_hits: u64,
    pub capr_misses: u64,
    pub emotion_hits: u64,
    pub emotion_misses: u64,
    pub total_saves: u64,
    pub total_loads: u64,
    pub last_save_timestamp: u64,
    pub last_load_timestamp: u64,
}

impl Default for CacheMind {
    fn default() -> Self {
        Self::new(128) // Default capacity
    }
}

impl Default for CacheMindStats {
    fn default() -> Self {
        Self {
            constraint_hits: 0,
            constraint_misses: 0,
            capr_hits: 0,
            capr_misses: 0,
            emotion_hits: 0,
            emotion_misses: 0,
            total_saves: 0,
            total_loads: 0,
            last_save_timestamp: 0,
            last_load_timestamp: 0,
        }
    }
}

impl CacheMind {
    /// Create new CacheMind with specified capacity for each cache
    pub fn new(max_entries: usize) -> Self {
        Self {
            constraint_cache: LRUCache::new(max_entries),
            capr_cache: LRUCache::new(max_entries),
            emotion_cache: LRUCache::new(max_entries),
            max_entries,
            stats: CacheMindStats::default(),
        }
    }

    /// Store constraint snapshot with context-based key
    pub fn store_constraint_snapshot(&mut self, key: String, snapshot: ConstraintSnapshot) {
        self.constraint_cache.insert(key, snapshot);
    }

    /// Retrieve constraint snapshot by key
    pub fn get_constraint_snapshot(&mut self, key: &str) -> Option<&ConstraintSnapshot> {
        if let Some(snapshot) = self.constraint_cache.get(&key.to_string()) {
            self.stats.constraint_hits += 1;
            Some(snapshot)
        } else {
            self.stats.constraint_misses += 1;
            None
        }
    }

    /// Store CAPR path summary
    pub fn store_capr_summary(&mut self, key: String, summary: CAPRPathSummary) {
        self.capr_cache.insert(key, summary);
    }

    /// Retrieve CAPR path summary by key
    pub fn get_capr_summary(&mut self, key: &str) -> Option<&CAPRPathSummary> {
        if let Some(summary) = self.capr_cache.get(&key.to_string()) {
            self.stats.capr_hits += 1;
            Some(summary)
        } else {
            self.stats.capr_misses += 1;
            None
        }
    }

    /// Store character emotion arc
    pub fn store_emotion_arc(&mut self, key: String, arc: CharacterEmotionArc) {
        self.emotion_cache.insert(key, arc);
    }

    /// Retrieve character emotion arc by key
    pub fn get_emotion_arc(&mut self, key: &str) -> Option<&CharacterEmotionArc> {
        if let Some(arc) = self.emotion_cache.get(&key.to_string()) {
            self.stats.emotion_hits += 1;
            Some(arc)
        } else {
            self.stats.emotion_misses += 1;
            None
        }
    }

    /// Generate cache key for constraint snapshot
    pub fn constraint_key(chapter: u32, scene: Option<u32>, context: &str) -> String {
        match scene {
            Some(s) => format!("constraint_ch{}_sc{}__{}", chapter, s, context),
            None => format!("constraint_ch{}__{}", chapter, context),
        }
    }

    /// Generate cache key for CAPR summary
    pub fn capr_key(chapter: u32, scene: Option<u32>, context: &str) -> String {
        match scene {
            Some(s) => format!("capr_ch{}_sc{}__{}", chapter, s, context),
            None => format!("capr_ch{}__{}", chapter, context),
        }
    }

    /// Generate cache key for emotion arc
    pub fn emotion_key(character: &str, chapter: u32, scene_range: (Option<u32>, Option<u32>)) -> String {
        match scene_range {
            (Some(start), Some(end)) => format!("emotion_{}_ch{}_sc{}-{}", character, chapter, start, end),
            (Some(start), None) => format!("emotion_{}_ch{}_sc{}-end", character, chapter, start),
            (None, Some(end)) => format!("emotion_{}_ch{}_start-sc{}", character, chapter, end),
            (None, None) => format!("emotion_{}_ch{}", character, chapter),
        }
    }

    /// Find similar constraint snapshots within similarity threshold
    pub fn find_similar_constraints(&mut self, reference: &ConstraintSnapshot, threshold: f32) -> Vec<String> {
        let mut similar_keys = Vec::new();

        for (key, snapshot) in self.constraint_cache.iter() {
            if reference.similarity(snapshot) >= threshold {
                similar_keys.push(key.clone());
            }
        }

        similar_keys
    }

    /// Get cache utilization statistics
    pub fn get_utilization(&self) -> CacheUtilization {
        CacheUtilization {
            constraint_usage: self.constraint_cache.len() as f32 / self.max_entries as f32,
            capr_usage: self.capr_cache.len() as f32 / self.max_entries as f32,
            emotion_usage: self.emotion_cache.len() as f32 / self.max_entries as f32,
            total_entries: self.constraint_cache.len() + self.capr_cache.len() + self.emotion_cache.len(),
            max_total_entries: self.max_entries * 3,
        }
    }

    /// Get cache hit ratios
    pub fn get_hit_ratios(&self) -> CacheHitRatios {
        let constraint_total = self.stats.constraint_hits + self.stats.constraint_misses;
        let capr_total = self.stats.capr_hits + self.stats.capr_misses;
        let emotion_total = self.stats.emotion_hits + self.stats.emotion_misses;

        CacheHitRatios {
            constraint_hit_ratio: if constraint_total > 0 {
                self.stats.constraint_hits as f32 / constraint_total as f32
            } else {
                0.0
            },
            capr_hit_ratio: if capr_total > 0 {
                self.stats.capr_hits as f32 / capr_total as f32
            } else {
                0.0
            },
            emotion_hit_ratio: if emotion_total > 0 {
                self.stats.emotion_hits as f32 / emotion_total as f32
            } else {
                0.0
            },
            overall_hit_ratio: {
                let total_hits = self.stats.constraint_hits + self.stats.capr_hits + self.stats.emotion_hits;
                let total_requests = constraint_total + capr_total + emotion_total;
                if total_requests > 0 {
                    total_hits as f32 / total_requests as f32
                } else {
                    0.0
                }
            },
        }
    }

    /// Clear all caches
    pub fn clear_all(&mut self) {
        self.constraint_cache.clear();
        self.capr_cache.clear();
        self.emotion_cache.clear();
    }

    /// Resize all caches
    pub fn resize(&mut self, new_max_entries: usize) {
        self.max_entries = new_max_entries;
        self.constraint_cache.resize(new_max_entries);
        self.capr_cache.resize(new_max_entries);
        self.emotion_cache.resize(new_max_entries);
    }

    /// Export cache to JSON string
    pub fn export_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Import cache from JSON string
    pub fn import_json(json_str: &str) -> Option<Self> {
        serde_json::from_str(json_str).ok()
    }

    /// Save cache to file
    pub fn save_to_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let json_data = self.export_json();
        let mut file = fs::File::create(path)?;
        file.write_all(json_data.as_bytes())?;
        file.flush()?;

        self.stats.total_saves += 1;
        self.stats.last_save_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(())
    }

    /// Load cache from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let json_data = fs::read_to_string(path)?;
        let mut cache = Self::import_json(&json_data)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse cache JSON"))?;

        cache.stats.total_loads += 1;
        cache.stats.last_load_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(cache)
    }

    /// Get default cache file path
    pub fn default_cache_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".shimmy")
            .join("cachemind.json")
    }

    /// Ensure cache directory exists
    pub fn ensure_cache_dir() -> io::Result<()> {
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".shimmy");

        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir)?;
        }

        Ok(())
    }

    /// Auto-save cache to default location
    pub fn auto_save(&mut self) -> io::Result<()> {
        Self::ensure_cache_dir()?;
        let cache_path = Self::default_cache_path();
        self.save_to_file(cache_path)
    }

    /// Auto-load cache from default location
    pub fn auto_load() -> Self {
        let cache_path = Self::default_cache_path();

        if cache_path.exists() {
            match Self::load_from_file(&cache_path) {
                Ok(cache) => {
                    tracing::info!("CacheMind loaded from {}", cache_path.display());
                    return cache;
                }
                Err(e) => {
                    tracing::warn!("Failed to load CacheMind from {}: {}", cache_path.display(), e);
                }
            }
        }

        tracing::info!("Creating new CacheMind instance");
        Self::new(128) // Default capacity as specified in the card
    }

    /// Set constraint snapshot in cache
    pub fn set_constraint_snapshot(&mut self, key: String, snapshot: ConstraintSnapshot) {
        self.constraint_cache.insert(key, snapshot);
    }

    /// Set CAPR path summary in cache
    pub fn set_capr_path_summary(&mut self, key: String, summary: CAPRPathSummary) {
        self.capr_cache.insert(key, summary);
    }

    /// Set character emotion arc in cache
    pub fn set_character_emotion_arc(&mut self, key: String, arc: CharacterEmotionArc) {
        self.emotion_cache.insert(key, arc);
    }

    /// Get CAPR path summary from cache
    pub fn get_capr_path_summary(&mut self, key: &str) -> Option<&CAPRPathSummary> {
        if let Some(summary) = self.capr_cache.get(&key.to_string()) {
            self.stats.capr_hits += 1;
            Some(summary)
        } else {
            self.stats.capr_misses += 1;
            None
        }
    }

    /// Get character emotion arc from cache
    pub fn get_character_emotion_arc(&mut self, key: &str) -> Option<&CharacterEmotionArc> {
        if let Some(arc) = self.emotion_cache.get(&key.to_string()) {
            self.stats.emotion_hits += 1;
            Some(arc)
        } else {
            self.stats.emotion_misses += 1;
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheUtilization {
    pub constraint_usage: f32,
    pub capr_usage: f32,
    pub emotion_usage: f32,
    pub total_entries: usize,
    pub max_total_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHitRatios {
    pub constraint_hit_ratio: f32,
    pub capr_hit_ratio: f32,
    pub emotion_hit_ratio: f32,
    pub overall_hit_ratio: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cachemind_basic_operations() {
        let mut cache = CacheMind::new(10);

        // Test constraint snapshot
        let snapshot = ConstraintSnapshot::new(
            0.7,
            vec!["constraint1".to_string(), "constraint2".to_string()],
            HashMap::new(),
            1,
            Some(1),
        );

        let key = CacheMind::constraint_key(1, Some(1), "test");
        cache.store_constraint_snapshot(key.clone(), snapshot);

        assert!(cache.get_constraint_snapshot(&key).is_some());
        assert_eq!(cache.stats.constraint_hits, 1);
    }

    #[test]
    fn test_capr_summary_health_score() {
        let summary = CAPRPathSummary::new(
            3,
            vec!["resolution1".to_string()],
            vec![], // No active contradictions
            vec!["pressure1".to_string()],
            5.0,
            0.8,
            1,
            Some(1),
        );

        let health = summary.health_score();
        assert!(health > 0.7); // Should be high due to no active contradictions and good loop quality
    }

    #[test]
    fn test_emotion_arc_turning_points() {
        let valence_sequence = vec![0.0, 0.5, 0.8, 0.2, -0.3, -0.1, 0.4];
        let arc = CharacterEmotionArc::new(
            "TestChar".to_string(),
            valence_sequence,
            vec![0.5; 7],
            vec!["joy".to_string(), "sadness".to_string()],
            1,
            (Some(1), Some(7)),
        );

        // Should detect turning points where emotional direction changes significantly
        assert!(!arc.turning_points.is_empty());
        assert_eq!(arc.arc_trend, "volatile"); // Large changes should be detected as volatile
    }

    #[test]
    fn test_constraint_similarity() {
        let snapshot1 = ConstraintSnapshot::new(
            0.7,
            vec!["c1".to_string(), "c2".to_string()],
            HashMap::new(),
            1,
            None,
        );

        let snapshot2 = ConstraintSnapshot::new(
            0.75,
            vec!["c1".to_string(), "c3".to_string()],
            HashMap::new(),
            1,
            None,
        );

        let similarity = snapshot1.similarity(&snapshot2);
        assert!(similarity > 0.0 && similarity < 1.0);
    }

    #[test]
    fn test_cache_serialization() {
        let mut cache = CacheMind::new(5);

        let snapshot = ConstraintSnapshot::new(
            0.8,
            vec!["test_constraint".to_string()],
            HashMap::new(),
            1,
            None,
        );

        cache.store_constraint_snapshot("test_key".to_string(), snapshot);

        let json = cache.export_json();
        assert!(!json.is_empty());

        let imported = CacheMind::import_json(&json);
        assert!(imported.is_some());

        let imported_cache = imported.unwrap();
        assert_eq!(imported_cache.max_entries, 5);
        assert_eq!(imported_cache.constraint_cache.len(), 1);
    }

    #[test]
    fn test_file_persistence() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let mut cache = CacheMind::new(5);
        let snapshot = ConstraintSnapshot::new(
            0.9,
            vec!["file_test".to_string()],
            HashMap::new(),
            2,
            Some(3),
        );

        cache.store_constraint_snapshot("file_key".to_string(), snapshot);

        // Save to file
        cache.save_to_file(path).unwrap();
        assert_eq!(cache.stats.total_saves, 1);

        // Load from file
        let loaded_cache = CacheMind::load_from_file(path).unwrap();
        assert_eq!(loaded_cache.constraint_cache.len(), 1);
        assert_eq!(loaded_cache.stats.total_loads, 1);
    }

    #[test]
    fn test_cache_hit_ratios() {
        let mut cache = CacheMind::new(5);

        // Generate some hits and misses
        cache.get_constraint_snapshot("nonexistent"); // Miss
        cache.get_capr_summary("nonexistent"); // Miss

        let snapshot = ConstraintSnapshot::new(0.5, vec![], HashMap::new(), 1, None);
        cache.store_constraint_snapshot("exists".to_string(), snapshot);
        cache.get_constraint_snapshot("exists"); // Hit

        let ratios = cache.get_hit_ratios();
        assert_eq!(ratios.constraint_hit_ratio, 0.5); // 1 hit, 1 miss
        assert_eq!(ratios.capr_hit_ratio, 0.0); // 0 hits, 1 miss
    }

    #[test]
    fn test_cache_utilization() {
        let mut cache = CacheMind::new(2);

        let utilization_empty = cache.get_utilization();
        assert_eq!(utilization_empty.total_entries, 0);

        // Fill constraint cache
        for i in 0..2 {
            let snapshot = ConstraintSnapshot::new(0.5, vec![], HashMap::new(), 1, None);
            cache.store_constraint_snapshot(format!("key{}", i), snapshot);
        }

        let utilization_half = cache.get_utilization();
        assert_eq!(utilization_half.constraint_usage, 1.0); // Constraint cache full
        assert_eq!(utilization_half.capr_usage, 0.0); // CAPR cache empty
    }
}