/**
 * Qualitier: Adaptive Quality Control System
 * ==========================================
 *
 * Dynamic performance tier management that adjusts narrative analysis quality
 * based on system resources, memory pressure, and narrative stress indicators.
 *
 * Four quality tiers:
 * - Minimal: Obligation injection only (emergency mode)
 * - Standard: + Emotion tracking (default)
 * - Enhanced: + Spatial validation, CAPR depth (high performance)
 * - Premium: Full recursive intelligence (maximum quality)
 */

use crate::adaptive::adapt_iq::AdaptIQSettings;
use crate::telemetry::Pulse;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Quality levels for adaptive performance management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityLevel {
    /// Emergency mode: Only essential obligation injection
    Minimal,
    /// Default mode: Basic emotion tracking and analysis
    Standard,
    /// High performance: Spatial validation and deeper CAPR analysis
    Enhanced,
    /// Maximum quality: Full recursive intelligence with all features
    Premium,
}

impl QualityLevel {
    /// Get human-readable description of the quality level
    pub fn description(&self) -> &'static str {
        match self {
            QualityLevel::Minimal => "Minimal (obligation injection only)",
            QualityLevel::Standard => "Standard (basic emotion tracking)",
            QualityLevel::Enhanced => "Enhanced (spatial validation, CAPR depth)",
            QualityLevel::Premium => "Premium (full recursive intelligence)",
        }
    }

    /// Get the maximum recursion depth for this quality level
    pub fn max_recursion_depth(&self) -> usize {
        match self {
            QualityLevel::Minimal => 4,
            QualityLevel::Standard => 6,
            QualityLevel::Enhanced => 10,
            QualityLevel::Premium => 14,
        }
    }

    /// Get the pathogen sensitivity cap for this quality level
    pub fn pathogen_sensitivity_cap(&self) -> f32 {
        match self {
            QualityLevel::Minimal => 0.3,
            QualityLevel::Standard => 0.6,
            QualityLevel::Enhanced => 0.8,
            QualityLevel::Premium => 1.0,
        }
    }

    /// Get the affect assertiveness cap for this quality level
    pub fn affect_assertiveness_cap(&self) -> f32 {
        match self {
            QualityLevel::Minimal => 0.2,
            QualityLevel::Standard => 0.5,
            QualityLevel::Enhanced => 0.8,
            QualityLevel::Premium => 1.0,
        }
    }

    /// Get the beat sampling rate cap for this quality level
    pub fn beat_sampling_rate_cap(&self) -> f32 {
        match self {
            QualityLevel::Minimal => 0.4,
            QualityLevel::Standard => 0.7,
            QualityLevel::Enhanced => 0.9,
            QualityLevel::Premium => 1.0,
        }
    }

    /// Check if a feature is enabled at this quality level
    pub fn feature_enabled(&self, feature: QualityFeature) -> bool {
        let required_level = feature.required_level();
        (*self as u8) >= (required_level as u8)
    }
}

impl Default for QualityLevel {
    fn default() -> Self {
        QualityLevel::Standard
    }
}

/// Features that can be enabled/disabled based on quality level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityFeature {
    ObligationInjection,
    EmotionTracking,
    SpatialValidation,
    CAPRDepthAnalysis,
    CharacterConsistency,
    EngagementLoops,
    DriftStabilization,
    CacheOptimization,
    FullRecursion,
}

impl QualityFeature {
    /// Get the minimum quality level required for this feature
    pub fn required_level(&self) -> QualityLevel {
        match self {
            QualityFeature::ObligationInjection => QualityLevel::Minimal,
            QualityFeature::EmotionTracking => QualityLevel::Standard,
            QualityFeature::SpatialValidation => QualityLevel::Enhanced,
            QualityFeature::CAPRDepthAnalysis => QualityLevel::Enhanced,
            QualityFeature::CharacterConsistency => QualityLevel::Standard,
            QualityFeature::EngagementLoops => QualityLevel::Enhanced,
            QualityFeature::DriftStabilization => QualityLevel::Standard,
            QualityFeature::CacheOptimization => QualityLevel::Enhanced,
            QualityFeature::FullRecursion => QualityLevel::Premium,
        }
    }
}

/// Performance constraints and monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum memory usage in MB before downgrading quality
    pub max_memory_mb: u64,
    /// Maximum analysis time in milliseconds per operation
    pub max_analysis_time_ms: u64,
    /// Enable adaptive quality adjustment
    pub adaptive_quality: bool,
    /// Memory pressure threshold for warnings (0.0-1.0)
    pub memory_pressure_threshold: f32,
    /// CPU usage threshold for degradation (0.0-1.0)
    pub cpu_threshold: f32,
    /// Minimum time between quality level changes (ms)
    pub quality_change_cooldown_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 100,
            max_analysis_time_ms: 50,
            adaptive_quality: true,
            memory_pressure_threshold: 0.8,
            cpu_threshold: 0.85,
            quality_change_cooldown_ms: 5000, // 5 second cooldown
        }
    }
}

/// Quality tier management and performance monitoring
#[derive(Debug, Clone, Default)]
pub struct Qualitier {
    /// Current quality level
    pub current: QualityLevel,
    /// Performance configuration
    pub config: PerformanceConfig,
    /// Statistics and monitoring
    pub stats: QualitierStats,
    /// Last quality level change timestamp
    last_change: Option<Instant>,
}

/// Statistics for Qualitier performance monitoring
#[derive(Debug, Clone, Default)]
pub struct QualitierStats {
    /// Number of quality level changes
    pub quality_changes: u64,
    /// Time spent in each quality level (in seconds)
    pub time_in_minimal: f64,
    pub time_in_standard: f64,
    pub time_in_enhanced: f64,
    pub time_in_premium: f64,
    /// Performance degradations due to memory pressure
    pub memory_degradations: u64,
    /// Performance degradations due to narrative stress
    pub narrative_stress_upgrades: u64,
    /// Total decision count
    pub decision_count: u64,
    /// Average decision time (ms)
    pub avg_decision_time_ms: f64,
    /// Last update timestamp
    pub last_update: Option<Instant>,
}

impl Qualitier {
    /// Create a new Qualitier with specified configuration
    pub fn new(max_memory_mb: u64, max_analysis_time_ms: u64, adaptive: bool) -> Self {
        Self {
            current: QualityLevel::Standard,
            config: PerformanceConfig {
                max_memory_mb,
                max_analysis_time_ms,
                adaptive_quality: adaptive,
                ..Default::default()
            },
            stats: QualitierStats::default(),
            last_change: None,
        }
    }

    /// Create Qualitier with full configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        Self {
            current: QualityLevel::Standard,
            config,
            stats: QualitierStats::default(),
            last_change: None,
        }
    }

    /// Main decision function: analyze current state and adjust quality level
    pub fn decide(&mut self, pulse: &Pulse, adapt: &AdaptIQSettings) -> bool {
        let start_time = Instant::now();
        let mut quality_changed = false;

        if !self.config.adaptive_quality {
            self.update_stats(start_time, false);
            return false;
        }

        // Check cooldown period
        if let Some(last_change) = self.last_change {
            if last_change.elapsed().as_millis() < self.config.quality_change_cooldown_ms as u128 {
                self.update_stats(start_time, false);
                return false;
            }
        }

        let previous_level = self.current;

        // Get system metrics
        let memory_pressure = self.get_memory_pressure();
        let narrative_stress = self.calculate_narrative_stress(pulse);

        // Decision logic
        let new_level = self.calculate_optimal_quality_level(memory_pressure, narrative_stress, pulse, adapt);

        if new_level != self.current {
            self.current = new_level;
            self.last_change = Some(Instant::now());
            quality_changed = true;

            // Update statistics
            self.stats.quality_changes += 1;
            if memory_pressure > self.config.memory_pressure_threshold {
                self.stats.memory_degradations += 1;
            }
            if narrative_stress > 0.6 && new_level as u8 > previous_level as u8 {
                self.stats.narrative_stress_upgrades += 1;
            }
        }

        self.update_stats(start_time, quality_changed);
        quality_changed
    }

    /// Calculate optimal quality level based on current conditions
    fn calculate_optimal_quality_level(
        &self,
        memory_pressure: f32,
        narrative_stress: f32,
        pulse: &Pulse,
        _adapt: &AdaptIQSettings,
    ) -> QualityLevel {
        // Rule 1: Memory over budget → emergency downgrade
        if memory_pressure > 0.95 {
            return QualityLevel::Minimal;
        }

        // Rule 2: Severe memory pressure → downgrade to Standard
        if memory_pressure > self.config.memory_pressure_threshold {
            return match self.current {
                QualityLevel::Premium | QualityLevel::Enhanced => QualityLevel::Standard,
                _ => self.current,
            };
        }

        // Rule 3: High narrative strain → upgrade for better analysis
        if narrative_stress > 0.7 || pulse.pathogens_detected > 3 || pulse.adi_score < 0.3 {
            return match self.current {
                QualityLevel::Minimal => QualityLevel::Standard,
                QualityLevel::Standard => QualityLevel::Enhanced,
                _ => self.current,
            };
        }

        // Rule 4: Stable ADI + plenty of resources → upgrade
        if pulse.adi_score > 0.7 && memory_pressure < 0.5 && narrative_stress < 0.3 {
            return match self.current {
                QualityLevel::Minimal => QualityLevel::Standard,
                QualityLevel::Standard => QualityLevel::Enhanced,
                QualityLevel::Enhanced => QualityLevel::Premium,
                QualityLevel::Premium => QualityLevel::Premium,
            };
        }

        // Rule 5: Moderate stress with good resources → Enhanced
        if narrative_stress > 0.4 && memory_pressure < 0.6 {
            return match self.current {
                QualityLevel::Minimal => QualityLevel::Standard,
                QualityLevel::Standard => QualityLevel::Enhanced,
                _ => self.current,
            };
        }

        // Default: maintain current level
        self.current
    }

    /// Calculate narrative stress level from pulse data
    fn calculate_narrative_stress(&self, pulse: &Pulse) -> f32 {
        let pathogen_stress = (pulse.pathogens_detected as f32) / 10.0; // Normalize to 0-1
        let drift_stress = (pulse.drift_hits as f32) / 5.0; // Normalize to 0-1
        let adi_stress = 1.0 - pulse.adi_score.clamp(0.0, 1.0);
        let affect_stress = (1.0 - pulse.affect_coherence).abs();

        (pathogen_stress * 0.3 + drift_stress * 0.3 + adi_stress * 0.3 + affect_stress * 0.1).clamp(0.0, 1.0)
    }

    /// Get current memory pressure (0.0 = no pressure, 1.0 = critical)
    fn get_memory_pressure(&self) -> f32 {
        // Simplified memory pressure calculation
        // In a real implementation, you'd use system monitoring
        #[cfg(feature = "sysinfo")]
        {
            use sysinfo::{System, SystemExt};
            let mut sys = System::new_all();
            sys.refresh_memory();
            let used_mb = sys.used_memory() / 1024 / 1024;
            (used_mb as f32) / (self.config.max_memory_mb as f32)
        }

        #[cfg(not(feature = "sysinfo"))]
        {
            // Fallback: estimate based on pulse memory usage
            0.5 // Assume moderate pressure
        }
    }

    /// Apply quality level constraints to AdaptIQ settings
    pub fn clamp_settings(&self, adapt: &mut AdaptIQSettings) {
        // Apply recursion depth limits
        adapt.recursion_depth = adapt.recursion_depth.min(self.current.max_recursion_depth());

        // Apply sensitivity caps
        adapt.pathogen_sensitivity = adapt.pathogen_sensitivity.min(self.current.pathogen_sensitivity_cap());
        adapt.affect_assertiveness = adapt.affect_assertiveness.min(self.current.affect_assertiveness_cap());
        adapt.beat_sampling_rate = adapt.beat_sampling_rate.min(self.current.beat_sampling_rate_cap());

        // Quality-specific adjustments
        match self.current {
            QualityLevel::Minimal => {
                // Emergency mode: minimize all expensive operations
                adapt.recursion_depth = adapt.recursion_depth.min(4);
                adapt.pathogen_sensitivity = 0.3;
                adapt.affect_assertiveness = 0.2;
                adapt.beat_sampling_rate = 0.4;
                adapt.eat_resolution_scale = 0.7;
                adapt.cache_preference = 0.9; // Favor caching heavily
            }
            QualityLevel::Standard => {
                // Standard mode: balanced performance
                adapt.recursion_depth = adapt.recursion_depth.min(6);
                adapt.eat_resolution_scale = adapt.eat_resolution_scale.min(1.0);
            }
            QualityLevel::Enhanced => {
                // Enhanced mode: allow higher performance
                adapt.recursion_depth = adapt.recursion_depth.min(10);
                adapt.eat_resolution_scale = adapt.eat_resolution_scale.min(1.3);
            }
            QualityLevel::Premium => {
                // Premium mode: maximum quality (minimal constraints)
                adapt.recursion_depth = adapt.recursion_depth.min(14);
                // Allow full range for other parameters
            }
        }
    }

    /// Check if a specific feature is enabled at current quality level
    pub fn is_feature_enabled(&self, feature: QualityFeature) -> bool {
        self.current.feature_enabled(feature)
    }

    /// Get current quality level
    pub fn current_level(&self) -> QualityLevel {
        self.current
    }

    /// Force set quality level (bypasses adaptive logic)
    pub fn set_quality_level(&mut self, level: QualityLevel) {
        if level != self.current {
            self.current = level;
            self.last_change = Some(Instant::now());
            self.stats.quality_changes += 1;
        }
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> &QualitierStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = QualitierStats::default();
    }

    /// Update internal statistics
    fn update_stats(&mut self, start_time: Instant, quality_changed: bool) {
        let decision_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to ms

        self.stats.decision_count += 1;
        self.stats.avg_decision_time_ms = (self.stats.avg_decision_time_ms * ((self.stats.decision_count - 1) as f64) + decision_time) / (self.stats.decision_count as f64);

        // Update time spent in current quality level
        if let Some(last_update) = self.stats.last_update {
            let time_delta = last_update.elapsed().as_secs_f64();
            match self.current {
                QualityLevel::Minimal => self.stats.time_in_minimal += time_delta,
                QualityLevel::Standard => self.stats.time_in_standard += time_delta,
                QualityLevel::Enhanced => self.stats.time_in_enhanced += time_delta,
                QualityLevel::Premium => self.stats.time_in_premium += time_delta,
            }
        }

        self.stats.last_update = Some(Instant::now());
    }

    /// Get quality level distribution as percentages
    pub fn get_quality_distribution(&self) -> QualityDistribution {
        let total_time = self.stats.time_in_minimal + self.stats.time_in_standard + self.stats.time_in_enhanced + self.stats.time_in_premium;

        if total_time == 0.0 {
            return QualityDistribution {
                minimal_percent: 0.0,
                standard_percent: 100.0,
                enhanced_percent: 0.0,
                premium_percent: 0.0,
            };
        }

        QualityDistribution {
            minimal_percent: (self.stats.time_in_minimal / total_time) * 100.0,
            standard_percent: (self.stats.time_in_standard / total_time) * 100.0,
            enhanced_percent: (self.stats.time_in_enhanced / total_time) * 100.0,
            premium_percent: (self.stats.time_in_premium / total_time) * 100.0,
        }
    }

    /// Get comprehensive status report
    pub fn get_status_report(&self) -> QualitierStatusReport {
        let memory_pressure = self.get_memory_pressure();
        let distribution = self.get_quality_distribution();

        QualitierStatusReport {
            current_level: self.current,
            memory_pressure,
            adaptive_enabled: self.config.adaptive_quality,
            quality_changes: self.stats.quality_changes,
            memory_degradations: self.stats.memory_degradations,
            narrative_stress_upgrades: self.stats.narrative_stress_upgrades,
            avg_decision_time_ms: self.stats.avg_decision_time_ms,
            distribution,
            last_change_elapsed_ms: self.last_change.map(|t| t.elapsed().as_millis() as u64),
        }
    }
}

/// Quality level time distribution
#[derive(Debug, Clone)]
pub struct QualityDistribution {
    pub minimal_percent: f64,
    pub standard_percent: f64,
    pub enhanced_percent: f64,
    pub premium_percent: f64,
}

/// Comprehensive status report for Qualitier
#[derive(Debug, Clone)]
pub struct QualitierStatusReport {
    pub current_level: QualityLevel,
    pub memory_pressure: f32,
    pub adaptive_enabled: bool,
    pub quality_changes: u64,
    pub memory_degradations: u64,
    pub narrative_stress_upgrades: u64,
    pub avg_decision_time_ms: f64,
    pub distribution: QualityDistribution,
    pub last_change_elapsed_ms: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::Pulse;

    #[test]
    fn test_quality_level_ordering() {
        assert!(QualityLevel::Minimal as u8 < QualityLevel::Standard as u8);
        assert!(QualityLevel::Standard as u8 < QualityLevel::Enhanced as u8);
        assert!(QualityLevel::Enhanced as u8 < QualityLevel::Premium as u8);
    }

    #[test]
    fn test_quality_level_constraints() {
        assert_eq!(QualityLevel::Minimal.max_recursion_depth(), 4);
        assert_eq!(QualityLevel::Standard.max_recursion_depth(), 6);
        assert_eq!(QualityLevel::Enhanced.max_recursion_depth(), 10);
        assert_eq!(QualityLevel::Premium.max_recursion_depth(), 14);

        assert!(QualityLevel::Minimal.pathogen_sensitivity_cap() < QualityLevel::Premium.pathogen_sensitivity_cap());
    }

    #[test]
    fn test_feature_enablement() {
        assert!(QualityLevel::Minimal.feature_enabled(QualityFeature::ObligationInjection));
        assert!(!QualityLevel::Minimal.feature_enabled(QualityFeature::EmotionTracking));

        assert!(QualityLevel::Standard.feature_enabled(QualityFeature::EmotionTracking));
        assert!(!QualityLevel::Standard.feature_enabled(QualityFeature::SpatialValidation));

        assert!(QualityLevel::Enhanced.feature_enabled(QualityFeature::SpatialValidation));
        assert!(!QualityLevel::Enhanced.feature_enabled(QualityFeature::FullRecursion));

        assert!(QualityLevel::Premium.feature_enabled(QualityFeature::FullRecursion));
    }

    #[test]
    fn test_qualitier_creation() {
        let qual = Qualitier::new(100, 50, true);
        assert_eq!(qual.current, QualityLevel::Standard);
        assert_eq!(qual.config.max_memory_mb, 100);
        assert_eq!(qual.config.max_analysis_time_ms, 50);
        assert!(qual.config.adaptive_quality);
    }

    #[test]
    fn test_settings_clamping() {
        let qual = Qualitier::new(100, 50, true);
        let mut settings = AdaptIQSettings {
            recursion_depth: 20,
            pathogen_sensitivity: 1.0,
            affect_assertiveness: 1.0,
            beat_sampling_rate: 1.0,
            zc_hysteresis_margin: 10,
            eat_resolution_scale: 2.0,
            cache_preference: 0.5,
        };

        // Test Standard level clamping
        qual.clamp_settings(&mut settings);
        assert!(settings.recursion_depth <= 6);
        assert!(settings.pathogen_sensitivity <= 0.6);
    }

    #[test]
    fn test_narrative_stress_calculation() {
        let qual = Qualitier::new(100, 50, true);

        let low_stress_pulse = Pulse::with_data(1, 0, 0, 0.9, 30.0, 0.8, 0.9, None);
        let high_stress_pulse = Pulse::with_data(2, 5, 3, 0.2, 80.0, -0.2, 0.3, None);

        let low_stress = qual.calculate_narrative_stress(&low_stress_pulse);
        let high_stress = qual.calculate_narrative_stress(&high_stress_pulse);

        assert!(high_stress > low_stress);
        assert!(low_stress >= 0.0 && low_stress <= 1.0);
        assert!(high_stress >= 0.0 && high_stress <= 1.0);
    }

    #[test]
    fn test_quality_level_decision() {
        let mut qual = Qualitier::new(100, 50, true);
        let pulse = Pulse::with_data(1, 2, 1, 0.7, 40.0, 0.5, 0.8, None);
        let adapt = AdaptIQSettings::default();

        // Test that decision updates stats
        let initial_count = qual.stats.decision_count;
        qual.decide(&pulse, &adapt);
        assert!(qual.stats.decision_count > initial_count);
    }

    #[test]
    fn test_manual_quality_setting() {
        let mut qual = Qualitier::new(100, 50, true);
        assert_eq!(qual.current, QualityLevel::Standard);

        qual.set_quality_level(QualityLevel::Premium);
        assert_eq!(qual.current, QualityLevel::Premium);
        assert_eq!(qual.stats.quality_changes, 1);
    }

    #[test]
    fn test_stats_tracking() {
        let mut qual = Qualitier::new(100, 50, true);
        let pulse = Pulse::with_data(1, 1, 1, 0.7, 40.0, 0.5, 0.8, None);
        let adapt = AdaptIQSettings::default();

        // Make several decisions
        for _ in 0..5 {
            qual.decide(&pulse, &adapt);
        }

        assert_eq!(qual.stats.decision_count, 5);
        assert!(qual.stats.avg_decision_time_ms >= 0.0);
    }

    #[test]
    fn test_adaptive_disable() {
        let mut qual = Qualitier::new(100, 50, false); // Adaptive disabled
        let pulse = Pulse::with_data(1, 10, 5, 0.1, 200.0, -0.5, 0.2, None); // High stress
        let adapt = AdaptIQSettings::default();

        let initial_level = qual.current;
        qual.decide(&pulse, &adapt);

        // Should not change level when adaptive is disabled
        assert_eq!(qual.current, initial_level);
    }
}