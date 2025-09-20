/**
 * AdaptIQ: Narrative Intelligence Modulator
 * =========================================
 *
 * A low-cost runtime system that adjusts narrative analysis depth and intelligence
 * effort based on content entropy, user preferences, and performance constraints.
 *
 * Modulates:
 * - ZC budget (recursion depth)
 * - Pathogen sensitivity
 * - EAT resolution (emotion injection)
 * - Emotion injection assertiveness
 * - Recursion step cutoff
 */

use crate::telemetry::{Pulse, PulseTrace};
use crate::cache::CacheMind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AdaptIQ configuration settings for downstream modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptIQSettings {
    /// Maximum recursion depth for ZC cycles
    pub recursion_depth: usize,
    /// Sensitivity threshold for pathogen detection (0.0-1.0)
    pub pathogen_sensitivity: f32,
    /// Assertiveness of emotion injection (0.0-1.0)
    pub affect_assertiveness: f32,
    /// Sampling rate for beat analysis (0.0-1.0)
    pub beat_sampling_rate: f32,
    /// Hysteresis margin for ZC gate activation
    pub zc_hysteresis_margin: usize,
    /// EAT (Emotion Analysis Threshold) resolution scaling
    pub eat_resolution_scale: f32,
    /// Cache utilization preference (0.0-1.0)
    pub cache_preference: f32,
}

impl Default for AdaptIQSettings {
    fn default() -> Self {
        Self {
            recursion_depth: 8,
            pathogen_sensitivity: 0.5,
            affect_assertiveness: 0.4,
            beat_sampling_rate: 0.7,
            zc_hysteresis_margin: 5,
            eat_resolution_scale: 1.0,
            cache_preference: 0.6,
        }
    }
}

impl AdaptIQSettings {
    /// Create conservative settings (favor stability and performance)
    pub fn conservative() -> Self {
        Self {
            recursion_depth: 4,
            pathogen_sensitivity: 0.3,
            affect_assertiveness: 0.2,
            beat_sampling_rate: 0.5,
            zc_hysteresis_margin: 3,
            eat_resolution_scale: 0.7,
            cache_preference: 0.8,
        }
    }

    /// Create aggressive settings (favor depth and sensitivity)
    pub fn aggressive() -> Self {
        Self {
            recursion_depth: 12,
            pathogen_sensitivity: 0.8,
            affect_assertiveness: 0.7,
            beat_sampling_rate: 0.9,
            zc_hysteresis_margin: 8,
            eat_resolution_scale: 1.3,
            cache_preference: 0.4,
        }
    }

    /// Create balanced settings (compromise between performance and depth)
    pub fn balanced() -> Self {
        Self::default()
    }

    /// Apply user taste preferences to the settings
    pub fn with_taste(&mut self, taste: &TasteLUT) -> &mut Self {
        // Curiosity increases recursion depth and affect assertiveness
        self.recursion_depth = ((self.recursion_depth as f32) * (1.0 + taste.curiosity * 0.5)) as usize;
        self.affect_assertiveness *= 1.0 + taste.curiosity * 0.3;

        // Coherence preference affects sampling rate and cache usage
        self.beat_sampling_rate *= 1.0 + taste.coherence_pleasure * 0.2;
        self.cache_preference *= 1.0 + taste.coherence_pleasure * 0.3;

        // Unease increases pathogen sensitivity
        self.pathogen_sensitivity *= 1.0 + taste.unease * 0.4;

        // Awe affects EAT resolution and hysteresis
        self.eat_resolution_scale *= 1.0 + taste.awe * 0.3;
        self.zc_hysteresis_margin = ((self.zc_hysteresis_margin as f32) * (1.0 + taste.awe * 0.2)) as usize;

        // Boredom reduces overall activity
        let boredom_dampening = 1.0 - taste.boredom * 0.3;
        self.recursion_depth = ((self.recursion_depth as f32) * boredom_dampening) as usize;
        self.affect_assertiveness *= boredom_dampening;
        self.beat_sampling_rate *= boredom_dampening;

        self
    }

    /// Clamp all values to safe ranges
    pub fn clamp(&mut self) -> &mut Self {
        self.recursion_depth = self.recursion_depth.clamp(2, 20);
        self.pathogen_sensitivity = self.pathogen_sensitivity.clamp(0.1, 1.0);
        self.affect_assertiveness = self.affect_assertiveness.clamp(0.0, 1.0);
        self.beat_sampling_rate = self.beat_sampling_rate.clamp(0.1, 1.0);
        self.zc_hysteresis_margin = self.zc_hysteresis_margin.clamp(1, 15);
        self.eat_resolution_scale = self.eat_resolution_scale.clamp(0.5, 2.0);
        self.cache_preference = self.cache_preference.clamp(0.0, 1.0);
        self
    }
}

/// User taste profile lookup table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteLUT {
    /// Preference for exploration vs exploitation (0.0-1.0)
    pub curiosity: f32,
    /// Preference for narrative coherence and flow (0.0-1.0)
    pub coherence_pleasure: f32,
    /// Tolerance for tension and conflict (0.0-1.0)
    pub unease: f32,
    /// Appreciation for sublime/transcendent moments (0.0-1.0)
    pub awe: f32,
    /// Resistance to repetitive content (0.0-1.0)
    pub boredom: f32,
}

impl TasteLUT {
    /// Returns a balanced taste profile with moderate preferences
    pub fn Balanced() -> Self {
        Self {
            curiosity: 0.5,
            coherence_pleasure: 0.5,
            unease: 0.5,
            awe: 0.5,
            boredom: 0.5,
        }
    }
}

impl Default for TasteLUT {
    fn default() -> Self {
        Self {
            curiosity: 0.5,
            coherence_pleasure: 0.6,
            unease: 0.4,
            awe: 0.5,
            boredom: 0.3,
        }
    }
}

impl TasteLUT {
    /// Create a taste profile favoring exploration and depth
    pub fn curious() -> Self {
        Self {
            curiosity: 0.8,
            coherence_pleasure: 0.4,
            unease: 0.6,
            awe: 0.7,
            boredom: 0.5,
        }
    }

    /// Create a taste profile favoring safety and stability
    pub fn safe() -> Self {
        Self {
            curiosity: 0.3,
            coherence_pleasure: 0.8,
            unease: 0.2,
            awe: 0.4,
            boredom: 0.2,
        }
    }

    /// Create a taste profile for balanced exploration
    pub fn balanced() -> Self {
        Self::default()
    }

    /// Create a taste profile for experimental/artistic content
    pub fn experimental() -> Self {
        Self {
            curiosity: 0.9,
            coherence_pleasure: 0.3,
            unease: 0.7,
            awe: 0.8,
            boredom: 0.8,
        }
    }

    /// Validate and clamp all taste values to valid ranges
    pub fn clamp(&mut self) -> &mut Self {
        self.curiosity = self.curiosity.clamp(0.0, 1.0);
        self.coherence_pleasure = self.coherence_pleasure.clamp(0.0, 1.0);
        self.unease = self.unease.clamp(0.0, 1.0);
        self.awe = self.awe.clamp(0.0, 1.0);
        self.boredom = self.boredom.clamp(0.0, 1.0);
        self
    }
}

/// AdaptIQ decision engine
#[derive(Debug, Clone)]
pub struct AdaptIQEngine {
    /// Base entropy level of content
    pub base_entropy: f32,
    /// User taste preferences
    pub taste_lut: TasteLUT,
    /// Performance constraints
    pub performance_constraints: PerformanceConstraints,
    /// Runtime statistics
    pub stats: AdaptIQStats,
}

/// Performance constraints for the AdaptIQ engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConstraints {
    /// Maximum allowed analysis time per cycle (ms)
    pub max_analysis_time_ms: u64,
    /// Maximum memory usage threshold (MB)
    pub max_memory_mb: f32,
    /// CPU utilization threshold (0.0-1.0)
    pub cpu_threshold: f32,
    /// Cache hit ratio target (0.0-1.0)
    pub target_cache_hit_ratio: f32,
}

impl Default for PerformanceConstraints {
    fn default() -> Self {
        Self {
            max_analysis_time_ms: 500,
            max_memory_mb: 100.0,
            cpu_threshold: 0.8,
            target_cache_hit_ratio: 0.7,
        }
    }
}

/// Runtime statistics for AdaptIQ decisions
#[derive(Debug, Clone, Default)]
pub struct AdaptIQStats {
    /// Number of decisions made
    pub decision_count: u64,
    /// Average decision time (ms)
    pub avg_decision_time_ms: f64,
    /// Cache utilization rate
    pub cache_utilization: f32,
    /// Performance adjustment count
    pub performance_adjustments: u64,
    /// Last decision timestamp
    pub last_decision_timestamp: Option<std::time::Instant>,
}

impl AdaptIQEngine {
    /// Create a new AdaptIQ engine with specified entropy and taste profile
    pub fn new(entropy: f32, taste: TasteLUT) -> Self {
        Self {
            base_entropy: entropy.clamp(0.0, 1.0),
            taste_lut: taste,
            performance_constraints: PerformanceConstraints::default(),
            stats: AdaptIQStats::default(),
        }
    }

    /// Create engine with custom performance constraints
    pub fn with_constraints(entropy: f32, taste: TasteLUT, constraints: PerformanceConstraints) -> Self {
        Self {
            base_entropy: entropy.clamp(0.0, 1.0),
            taste_lut: taste,
            performance_constraints: constraints,
            stats: AdaptIQStats::default(),
        }
    }

    /// Main decision function: analyze context and return adaptive settings
    pub fn decide(&mut self, pulse: &Pulse, cache: &CacheMind) -> AdaptIQSettings {
        let start_time = std::time::Instant::now();

        // Calculate context factors
        let tension = self.calculate_tension_factor(pulse);
        let freshness_bonus = self.calculate_freshness_bonus(cache);
        let drift_factor = self.calculate_drift_factor(pulse);
        let performance_factor = self.calculate_performance_factor(pulse);

        // Generate base settings
        let mut settings = self.generate_base_settings(tension, freshness_bonus, drift_factor, performance_factor);

        // Apply taste preferences
        settings.with_taste(&self.taste_lut);

        // Apply performance constraints
        self.apply_performance_constraints(&mut settings, performance_factor);

        // Clamp to safe ranges
        settings.clamp();

        // Update statistics
        self.update_stats(start_time, cache);

        settings
    }

    /// Calculate tension factor from pulse metrics
    fn calculate_tension_factor(&self, pulse: &Pulse) -> f32 {
        let pathogen_tension = (pulse.pathogens_detected as f32) / 10.0; // Normalize assuming max ~10 pathogens
        let drift_tension = (pulse.drift_hits as f32) / 5.0; // Normalize assuming max ~5 drift hits
        let affect_tension = (1.0 - pulse.affect_pleasure).abs() + (1.0 - pulse.affect_coherence).abs();

        (pathogen_tension + drift_tension + affect_tension * 0.5).clamp(0.0, 2.0)
    }

    /// Calculate freshness bonus from cache state
    fn calculate_freshness_bonus(&self, cache: &CacheMind) -> f32 {
        // Check cache freshness - more entries suggest more context
        let constraint_freshness = cache.constraint_cache.len() as f32 / cache.constraint_cache.capacity() as f32;
        let capr_freshness = cache.capr_cache.len() as f32 / cache.capr_cache.capacity() as f32;
        let character_freshness = cache.emotion_cache.len() as f32 / cache.emotion_cache.capacity() as f32;

        let avg_freshness = (constraint_freshness + capr_freshness + character_freshness) / 3.0;

        // Bonus for having cached context, but diminishing returns
        (avg_freshness * 0.3).clamp(0.0, 0.5)
    }

    /// Calculate drift factor from ADI score
    fn calculate_drift_factor(&self, pulse: &Pulse) -> f32 {
        1.0 - pulse.adi_score.clamp(0.0, 1.0)
    }

    /// Calculate performance factor based on system state
    fn calculate_performance_factor(&self, pulse: &Pulse) -> f32 {
        // Simple performance assessment based on memory usage
        let memory_pressure = (pulse.memory_usage_mb / self.performance_constraints.max_memory_mb).clamp(0.0, 1.0);

        // CPU pressure could be inferred from pulse timing (simplified)
        let timing_pressure = if pulse.timestamp.elapsed().as_millis() > self.performance_constraints.max_analysis_time_ms as u128 {
            0.3
        } else {
            0.0
        };

        (memory_pressure + timing_pressure).clamp(0.0, 1.0)
    }

    /// Generate base settings before taste and performance adjustments
    fn generate_base_settings(&self, tension: f32, freshness_bonus: f32, drift_factor: f32, performance_factor: f32) -> AdaptIQSettings {
        // Base recursion depth influenced by entropy and tension
        let base_depth = ((self.base_entropy + tension + freshness_bonus) * 4.0 + 4.0).clamp(4.0, 12.0) as usize;

        // Pathogen sensitivity based on drift and tension
        let base_sensitivity = (0.5 + drift_factor * 0.3 + tension * 0.2).clamp(0.1, 1.0);

        // Affect assertiveness based on entropy and user preferences baseline
        let base_assertiveness = (0.3 + self.base_entropy * 0.4).clamp(0.1, 0.8);

        // Beat sampling rate influenced by freshness and entropy
        let base_sampling = (0.5 + self.base_entropy * 0.3 + freshness_bonus).clamp(0.3, 0.9);

        // ZC hysteresis margin based on tension and drift
        let base_hysteresis = (3.0 + tension * 2.0 + drift_factor * 3.0).clamp(2.0, 10.0) as usize;

        // EAT resolution scaling based on entropy and tension
        let base_eat_scale = (1.0 + self.base_entropy * 0.3 + tension * 0.2).clamp(0.7, 1.5);

        // Cache preference inversely related to performance pressure
        let base_cache_pref = (0.7 - performance_factor * 0.3).clamp(0.2, 0.9);

        AdaptIQSettings {
            recursion_depth: base_depth,
            pathogen_sensitivity: base_sensitivity,
            affect_assertiveness: base_assertiveness,
            beat_sampling_rate: base_sampling,
            zc_hysteresis_margin: base_hysteresis,
            eat_resolution_scale: base_eat_scale,
            cache_preference: base_cache_pref,
        }
    }

    /// Apply performance constraints to settings
    fn apply_performance_constraints(&mut self, settings: &mut AdaptIQSettings, performance_factor: f32) {
        if performance_factor > 0.5 {
            // Under performance pressure, reduce computational load
            settings.recursion_depth = (settings.recursion_depth as f32 * (1.0 - performance_factor * 0.4)) as usize;
            settings.beat_sampling_rate *= 1.0 - performance_factor * 0.3;
            settings.eat_resolution_scale *= 1.0 - performance_factor * 0.2;
            settings.cache_preference *= 1.0 + performance_factor * 0.3; // Favor cache under pressure

            self.stats.performance_adjustments += 1;
        }
    }

    /// Update runtime statistics
    fn update_stats(&mut self, start_time: std::time::Instant, cache: &CacheMind) {
        let decision_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to ms

        self.stats.decision_count += 1;
        self.stats.avg_decision_time_ms = (self.stats.avg_decision_time_ms * ((self.stats.decision_count - 1) as f64) + decision_time) / (self.stats.decision_count as f64);

        // Update cache utilization
        let total_entries = cache.constraint_cache.len() + cache.capr_cache.len() + cache.emotion_cache.len();
        let total_capacity = cache.constraint_cache.capacity() * 3; // Three caches
        self.stats.cache_utilization = total_entries as f32 / total_capacity as f32;

        self.stats.last_decision_timestamp = Some(std::time::Instant::now());
    }

    /// Get current statistics
    pub fn get_stats(&self) -> &AdaptIQStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = AdaptIQStats::default();
    }

    /// Update taste preferences
    pub fn update_taste(&mut self, new_taste: TasteLUT) {
        self.taste_lut = new_taste;
    }

    /// Update base entropy (e.g., when analyzing different content)
    pub fn update_entropy(&mut self, new_entropy: f32) {
        self.base_entropy = new_entropy.clamp(0.0, 1.0);
    }

    /// Create a quick decision with minimal computation (for performance-critical paths)
    pub fn quick_decide(&mut self, adi_score: f32, memory_usage: f32) -> AdaptIQSettings {
        let mut settings = AdaptIQSettings::default();

        // Quick heuristics
        if adi_score < 0.3 {
            settings.recursion_depth = 6;
            settings.pathogen_sensitivity = 0.7;
        } else if adi_score > 0.7 {
            settings.recursion_depth = 4;
            settings.pathogen_sensitivity = 0.4;
        }

        if memory_usage > 80.0 {
            settings.recursion_depth = (settings.recursion_depth / 2).max(2);
            settings.cache_preference = 0.9;
        }

        settings.with_taste(&self.taste_lut);
        settings.clamp();

        self.stats.decision_count += 1;
        settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::Pulse;
    use crate::cache::CacheMind;

    #[test]
    fn test_adapt_iq_settings_default() {
        let settings = AdaptIQSettings::default();
        assert_eq!(settings.recursion_depth, 8);
        assert_eq!(settings.pathogen_sensitivity, 0.5);
        assert_eq!(settings.affect_assertiveness, 0.4);
        assert_eq!(settings.beat_sampling_rate, 0.7);
        assert_eq!(settings.zc_hysteresis_margin, 5);
    }

    #[test]
    fn test_adapt_iq_settings_presets() {
        let conservative = AdaptIQSettings::conservative();
        assert_eq!(conservative.recursion_depth, 4);
        assert!(conservative.pathogen_sensitivity < 0.5);

        let aggressive = AdaptIQSettings::aggressive();
        assert_eq!(aggressive.recursion_depth, 12);
        assert!(aggressive.pathogen_sensitivity > 0.5);
    }

    #[test]
    fn test_taste_lut_presets() {
        let curious = TasteLUT::curious();
        assert!(curious.curiosity > 0.7);
        assert!(curious.awe > 0.6);

        let safe = TasteLUT::safe();
        assert!(safe.coherence_pleasure > 0.7);
        assert!(safe.unease < 0.3);
    }

    #[test]
    fn test_adapt_iq_engine_creation() {
        let taste = TasteLUT::balanced();
        let engine = AdaptIQEngine::new(0.6, taste);

        assert_eq!(engine.base_entropy, 0.6);
        assert_eq!(engine.stats.decision_count, 0);
    }

    #[test]
    fn test_adapt_iq_decision_basic() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);
        let cache = CacheMind::new();

        let pulse = Pulse::with_data(1, 2, 1, 0.7, 50.0, 0.3, 0.8, None);
        let settings = engine.decide(&pulse, &cache);

        assert!(settings.recursion_depth >= 2);
        assert!(settings.recursion_depth <= 20);
        assert!(settings.pathogen_sensitivity >= 0.1);
        assert!(settings.pathogen_sensitivity <= 1.0);
    }

    #[test]
    fn test_taste_influence_on_settings() {
        let curious_taste = TasteLUT::curious();
        let safe_taste = TasteLUT::safe();

        let mut curious_settings = AdaptIQSettings::default();
        let mut safe_settings = AdaptIQSettings::default();

        curious_settings.with_taste(&curious_taste);
        safe_settings.with_taste(&safe_taste);

        // Curious should have higher recursion depth and affect assertiveness
        assert!(curious_settings.recursion_depth >= safe_settings.recursion_depth);
        assert!(curious_settings.affect_assertiveness >= safe_settings.affect_assertiveness);
    }

    #[test]
    fn test_performance_constraints() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.8, taste);
        let cache = CacheMind::new();

        // High-pressure pulse (high memory usage)
        let high_pressure_pulse = Pulse::with_data(1, 5, 3, 0.3, 150.0, -0.2, 0.4, None);
        let settings = engine.decide(&high_pressure_pulse, &cache);

        // Should reduce computational load under pressure
        assert!(settings.recursion_depth < 12); // Should be reduced from high entropy base
        assert!(settings.cache_preference > 0.5); // Should favor caching
    }

    #[test]
    fn test_quick_decide() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);

        let settings1 = engine.quick_decide(0.2, 40.0); // Low ADI, low memory
        let settings2 = engine.quick_decide(0.8, 90.0); // High ADI, high memory

        assert!(settings1.pathogen_sensitivity > settings2.pathogen_sensitivity);
        assert!(settings1.recursion_depth > settings2.recursion_depth);
        assert!(settings2.cache_preference > settings1.cache_preference);
    }

    #[test]
    fn test_settings_clamping() {
        let mut settings = AdaptIQSettings {
            recursion_depth: 100, // Too high
            pathogen_sensitivity: 2.0, // Too high
            affect_assertiveness: -0.5, // Too low
            beat_sampling_rate: 1.5, // Too high
            zc_hysteresis_margin: 0, // Too low
            eat_resolution_scale: 3.0, // Too high
            cache_preference: -1.0, // Too low
        };

        settings.clamp();

        assert_eq!(settings.recursion_depth, 20);
        assert_eq!(settings.pathogen_sensitivity, 1.0);
        assert_eq!(settings.affect_assertiveness, 0.0);
        assert_eq!(settings.beat_sampling_rate, 1.0);
        assert_eq!(settings.zc_hysteresis_margin, 1);
        assert_eq!(settings.eat_resolution_scale, 2.0);
        assert_eq!(settings.cache_preference, 0.0);
    }

    #[test]
    fn test_stats_tracking() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);
        let cache = CacheMind::new();
        let pulse = Pulse::with_data(1, 1, 1, 0.7, 50.0, 0.3, 0.8, None);

        assert_eq!(engine.get_stats().decision_count, 0);

        let _settings = engine.decide(&pulse, &cache);
        assert_eq!(engine.get_stats().decision_count, 1);
        assert!(engine.get_stats().avg_decision_time_ms >= 0.0);

        let _settings2 = engine.quick_decide(0.5, 50.0);
        assert_eq!(engine.get_stats().decision_count, 2);
    }
}