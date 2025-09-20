/// 🧠 Card 8: Recursive Drift Stabilizer (RDS)
///
/// A system that monitors and prevents slow narrative drift across recursive chapter generations.
/// This is a meta-stability layer that watches the long tail of recursion and warns when
/// structural decay begins.

use crate::obligation_pressure::Obligation;
use crate::emotion_resonance::EmotionalState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Represents the stability state for drift monitoring across recursive generations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftStabilityState {
    /// Number of unresolved obligations
    pub unresolved_obligation_count: usize,
    /// Number of obligations that have been stale (unaddressed) for multiple chapters
    pub stale_obligations: usize,
    /// Sum of emotional decay across all tracked emotional states
    pub emotional_decay_sum: f32,
    /// Score representing theme drift from original narrative goals
    pub theme_drift_score: f32,
    /// Whether spatial return pressure has been lost for important locations
    pub spatial_return_pressure_lost: bool,
    /// Current chapter number
    pub current_chapter: u32,
    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
    /// Additional metadata for tracking
    pub metadata: HashMap<String, f32>,
}

impl DriftStabilityState {
    /// Creates a new drift stability state
    pub fn new(chapter: u32) -> Self {
        Self {
            unresolved_obligation_count: 0,
            stale_obligations: 0,
            emotional_decay_sum: 0.0,
            theme_drift_score: 0.0,
            spatial_return_pressure_lost: false,
            current_chapter: chapter,
            last_updated: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Updates the stability state with new metrics
    pub fn update_metrics(
        &mut self,
        obligations: &[Obligation],
        emotional_states: &[EmotionalState],
        theme_coherence: f32,
        spatial_return_pressure: bool,
    ) {
        self.unresolved_obligation_count = obligations.len();
        self.stale_obligations = self.count_stale_obligations(obligations);
        self.emotional_decay_sum = self.calculate_emotional_decay(emotional_states);
        self.theme_drift_score = (1.0 - theme_coherence).max(0.0);
        self.spatial_return_pressure_lost = !spatial_return_pressure;
        self.last_updated = Utc::now();
    }

    /// Counts obligations that have been stale for multiple chapters
    fn count_stale_obligations(&self, obligations: &[Obligation]) -> usize {
        obligations
            .iter()
            .filter(|o| o.age >= 3) // Stale if unresolved for 3+ chapters
            .count()
    }

    /// Calculates emotional decay across all emotional states
    fn calculate_emotional_decay(&self, emotional_states: &[EmotionalState]) -> f32 {
        emotional_states
            .iter()
            .map(|state| {
                // Decay increases as intensity decreases over time
                let base_decay = 1.0 - state.total_intensity();
                // Amplify decay for negative emotions that should resolve
                match state.dominant_emotion() {
                    "guilt" | "sadness" | "fear" | "anger" => base_decay * 1.5,
                    _ => base_decay,
                }
            })
            .sum()
    }

    /// Advances to the next chapter
    pub fn advance_chapter(&mut self) {
        self.current_chapter += 1;
        self.last_updated = Utc::now();
    }

    /// Gets the number of chapters since the last update
    pub fn chapters_since_update(&self) -> u32 {
        // This would typically be calculated from persistent state
        // For now, we'll use a simple heuristic
        0
    }

    /// Adds custom metadata for tracking specific drift indicators
    pub fn set_metadata(&mut self, key: impl Into<String>, value: f32) {
        self.metadata.insert(key.into(), value);
    }

    /// Gets custom metadata value
    pub fn get_metadata(&self, key: &str) -> Option<f32> {
        self.metadata.get(key).copied()
    }

    /// Get drift indicators as a vector of key issues
    pub fn get_drift_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.stale_obligations > 0 {
            indicators.push(format!("Stale obligations: {}", self.stale_obligations));
        }
        if self.emotional_decay_sum > 0.5 {
            indicators.push(format!("Emotional decay: {:.2}", self.emotional_decay_sum));
        }
        if self.theme_drift_score > 0.3 {
            indicators.push(format!("Theme drift: {:.2}", self.theme_drift_score));
        }
        if self.spatial_return_pressure_lost {
            indicators.push("Spatial return pressure lost".to_string());
        }

        indicators
    }
}

/// Configuration for drift stabilizer thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftStabilizerConfig {
    pub enabled: bool,
    pub stale_obligation_threshold: usize,
    pub emotional_decay_limit: f32,
    pub theme_threshold: f32,
    pub spatial_pressure_chapter_limit: u32,
}

impl Default for DriftStabilizerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stale_obligation_threshold: 5,
            emotional_decay_limit: 2.5,
            theme_threshold: 1.0,
            spatial_pressure_chapter_limit: 3,
        }
    }
}

/// Checks for recursive drift and returns warnings if any are detected
///
/// # Arguments
/// * `state` - The current drift stability state
/// * `config` - Configuration for drift detection thresholds
///
/// # Returns
/// An optional string containing warnings, or None if no drift is detected
pub fn check_recursive_drift(
    state: &DriftStabilityState,
    config: &DriftStabilizerConfig,
) -> Option<String> {
    if !config.enabled {
        return None;
    }

    let mut warnings = Vec::new();

    // Check for stale obligations
    if state.stale_obligations > config.stale_obligation_threshold {
        warnings.push(format!(
            "⚠️ Multiple unresolved obligations ({}) remain unaddressed across recent chapters.",
            state.stale_obligations
        ));
    }

    // Check for emotional decay
    if state.emotional_decay_sum > config.emotional_decay_limit {
        warnings.push(format!(
            "⚠️ Emotional field decay detected — resolution pressure falling (decay: {:.2}).",
            state.emotional_decay_sum
        ));
    }

    // Check for theme drift
    if state.theme_drift_score > config.theme_threshold {
        warnings.push(format!(
            "⚠️ Theme resonance mismatch — tonal drift possible (score: {:.2}).",
            state.theme_drift_score
        ));
    }

    // Check for spatial return pressure loss
    if state.spatial_return_pressure_lost {
        warnings.push(
            "⚠️ Gravity-decay: high-gravity location not revisited in >3 chapters.".to_string(),
        );
    }

    // Check custom metadata thresholds
    if let Some(character_drift) = state.get_metadata("character_consistency") {
        if character_drift > 0.7 {
            warnings.push(format!(
                "⚠️ Character consistency drift detected (score: {:.2}).",
                character_drift
            ));
        }
    }

    if let Some(pacing_drift) = state.get_metadata("pacing_consistency") {
        if pacing_drift > 0.8 {
            warnings.push(format!(
                "⚠️ Narrative pacing inconsistency detected (score: {:.2}).",
                pacing_drift
            ));
        }
    }

    if warnings.is_empty() {
        None
    } else {
        Some(warnings.join("\n"))
    }
}

/// Generates a narrative drift injection prompt based on detected issues
///
/// # Arguments
/// * `state` - The current drift stability state
/// * `warnings` - The warnings generated by drift detection
///
/// # Returns
/// A formatted string for prompt injection to address drift issues
pub fn generate_drift_injection_prompt(
    state: &DriftStabilityState,
    warnings: &str,
) -> String {
    let mut injection = String::new();

    injection.push_str("🧠 Narrative Drift Warning:\n");
    injection.push_str(warnings);
    injection.push('\n');

    // Add specific guidance based on drift type
    if warnings.contains("unresolved obligations") {
        injection.push_str("→ Consider resolving or advancing at least one major obligation\n");
    }

    if warnings.contains("Emotional field decay") {
        injection.push_str("→ Consider intensifying emotional stakes or providing resolution\n");
    }

    if warnings.contains("Theme resonance mismatch") {
        injection.push_str("→ Consider reinforcing core themes or narrative focus\n");
    }

    if warnings.contains("Gravity-decay") {
        injection.push_str("→ Consider reintroducing high-gravity location or explaining absence\n");
    }

    injection.push_str(&format!("\nCurrent Chapter: {}\n", state.current_chapter));
    injection.push_str("Please address these drift concerns in the upcoming narrative.\n");

    injection
}

/// Analyzes historical drift patterns to predict future stability issues
///
/// # Arguments
/// * `history` - A slice of historical drift states
///
/// # Returns
/// A tuple containing (trend_score, prediction_confidence)
pub fn analyze_drift_trends(history: &[DriftStabilityState]) -> (f32, f32) {
    if history.len() < 3 {
        return (0.0, 0.0);
    }

    let recent = &history[history.len().saturating_sub(3)..];

    // Calculate trends in key metrics
    let obligation_trend = calculate_trend(
        &recent
            .iter()
            .map(|s| s.stale_obligations as f32)
            .collect::<Vec<_>>(),
    );

    let emotional_trend = calculate_trend(
        &recent
            .iter()
            .map(|s| s.emotional_decay_sum)
            .collect::<Vec<_>>(),
    );

    let theme_trend = calculate_trend(
        &recent
            .iter()
            .map(|s| s.theme_drift_score)
            .collect::<Vec<_>>(),
    );

    // Combine trends (higher is worse)
    let combined_trend = (obligation_trend + emotional_trend + theme_trend) / 3.0;

    // Confidence based on consistency of trends
    let variance = calculate_variance(&[obligation_trend, emotional_trend, theme_trend]);
    let confidence = (1.0 - variance).max(0.0).min(1.0);

    (combined_trend, confidence)
}

/// Helper function to calculate linear trend from a series of values
fn calculate_trend(values: &[f32]) -> f32 {
    if values.len() < 2 {
        return 0.0;
    }

    let n = values.len() as f32;
    let sum_x: f32 = (0..values.len()).map(|i| i as f32).sum();
    let sum_y: f32 = values.iter().sum();
    let sum_xy: f32 = values
        .iter()
        .enumerate()
        .map(|(i, &y)| i as f32 * y)
        .sum();
    let sum_x2: f32 = (0..values.len()).map(|i| (i as f32).powi(2)).sum();

    // Linear regression slope
    (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2))
}

/// Helper function to calculate variance
fn calculate_variance(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let variance = values
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>()
        / values.len() as f32;

    variance
}

/// Creates a comprehensive drift stability report
///
/// # Arguments
/// * `state` - The current drift stability state
/// * `config` - The drift stabilizer configuration
/// * `history` - Optional historical data for trend analysis
///
/// # Returns
/// A formatted report string
pub fn generate_drift_stability_report(
    state: &DriftStabilityState,
    config: &DriftStabilizerConfig,
    history: Option<&[DriftStabilityState]>,
) -> String {
    let mut report = String::new();

    report.push_str("🧠 RECURSIVE DRIFT STABILITY REPORT\n");
    report.push_str("====================================\n\n");

    report.push_str(&format!("Current Chapter: {}\n", state.current_chapter));
    report.push_str(&format!("Last Updated: {}\n", state.last_updated.format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str(&format!("Stabilizer Status: {}\n\n", if config.enabled { "✅ ENABLED" } else { "❌ DISABLED" }));

    // Current metrics
    report.push_str("📊 Current Metrics:\n");
    report.push_str(&format!("  • Unresolved Obligations: {}\n", state.unresolved_obligation_count));
    report.push_str(&format!("  • Stale Obligations: {} (threshold: {})\n",
        state.stale_obligations, config.stale_obligation_threshold));
    report.push_str(&format!("  • Emotional Decay Sum: {:.2} (limit: {:.2})\n",
        state.emotional_decay_sum, config.emotional_decay_limit));
    report.push_str(&format!("  • Theme Drift Score: {:.2} (threshold: {:.2})\n",
        state.theme_drift_score, config.theme_threshold));
    report.push_str(&format!("  • Spatial Pressure Lost: {}\n\n",
        if state.spatial_return_pressure_lost { "⚠️ YES" } else { "✅ NO" }));

    // Warnings
    if let Some(warnings) = check_recursive_drift(state, config) {
        report.push_str("⚠️ ACTIVE WARNINGS:\n");
        report.push_str(&warnings);
        report.push_str("\n\n");
    } else {
        report.push_str("✅ NO DRIFT WARNINGS\n\n");
    }

    // Custom metadata
    if !state.metadata.is_empty() {
        report.push_str("🎯 Custom Tracking:\n");
        for (key, value) in &state.metadata {
            report.push_str(&format!("  • {}: {:.2}\n", key, value));
        }
        report.push('\n');
    }

    // Trend analysis
    if let Some(hist) = history {
        let (trend, confidence) = analyze_drift_trends(hist);
        report.push_str("📈 Trend Analysis:\n");
        report.push_str(&format!("  • Drift Trend: {:.2} (higher = more drift)\n", trend));
        report.push_str(&format!("  • Confidence: {:.2} (0.0-1.0)\n", confidence));

        let trend_status = match trend {
            t if t > 0.5 => "🔴 DETERIORATING",
            t if t > 0.2 => "🟡 SLIGHT DRIFT",
            t if t > -0.2 => "🔵 STABLE",
            _ => "🟢 IMPROVING",
        };
        report.push_str(&format!("  • Status: {}\n\n", trend_status));
    }

    // Configuration
    report.push_str("⚙️ Configuration:\n");
    report.push_str(&format!("  • Stale Obligation Threshold: {}\n", config.stale_obligation_threshold));
    report.push_str(&format!("  • Emotional Decay Limit: {:.2}\n", config.emotional_decay_limit));
    report.push_str(&format!("  • Theme Threshold: {:.2}\n", config.theme_threshold));
    report.push_str(&format!("  • Spatial Pressure Chapter Limit: {}\n", config.spatial_pressure_chapter_limit));

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obligation_pressure::Obligation;
    use crate::emotion_resonance::EmotionalState;

    #[test]
    fn test_drift_stability_state_creation() {
        let state = DriftStabilityState::new(5);
        assert_eq!(state.current_chapter, 5);
        assert_eq!(state.unresolved_obligation_count, 0);
        assert_eq!(state.stale_obligations, 0);
        assert_eq!(state.emotional_decay_sum, 0.0);
        assert_eq!(state.theme_drift_score, 0.0);
        assert!(!state.spatial_return_pressure_lost);
    }

    #[test]
    fn test_count_stale_obligations() {
        let state = DriftStabilityState::new(1);
        let obligations = vec![
            Obligation::new("fresh", 0.8, 1),
            Obligation::new("stale1", 0.6, 3),
            Obligation::new("stale2", 0.7, 5),
            Obligation::new("fresh2", 0.5, 2),
        ];

        let stale_count = state.count_stale_obligations(&obligations);
        assert_eq!(stale_count, 2); // obligations with age >= 3
    }

    #[test]
    fn test_emotional_decay_calculation() {
        let state = DriftStabilityState::new(1);
        let emotional_states = vec![
            EmotionalState::new("joy", 0.8),     // decay = 0.2
            EmotionalState::new("guilt", 0.3),   // decay = 0.7 * 1.5 = 1.05
            EmotionalState::new("fear", 0.5),    // decay = 0.5 * 1.5 = 0.75
        ];

        let decay = state.calculate_emotional_decay(&emotional_states);
        assert!((decay - 2.0).abs() < 0.1); // 0.2 + 1.05 + 0.75 = 2.0
    }

    #[test]
    fn test_check_recursive_drift_no_warnings() {
        let state = DriftStabilityState::new(1);
        let config = DriftStabilizerConfig::default();

        let result = check_recursive_drift(&state, &config);
        assert!(result.is_none());
    }

    #[test]
    fn test_check_recursive_drift_with_warnings() {
        let mut state = DriftStabilityState::new(1);
        state.stale_obligations = 6; // Above threshold of 5
        state.emotional_decay_sum = 3.0; // Above limit of 2.5
        state.theme_drift_score = 1.5; // Above threshold of 1.0
        state.spatial_return_pressure_lost = true;

        let config = DriftStabilizerConfig::default();
        let result = check_recursive_drift(&state, &config);

        assert!(result.is_some());
        let warnings = result.unwrap();
        assert!(warnings.contains("Multiple unresolved obligations"));
        assert!(warnings.contains("Emotional field decay"));
        assert!(warnings.contains("Theme resonance mismatch"));
        assert!(warnings.contains("Gravity-decay"));
    }

    #[test]
    fn test_check_recursive_drift_disabled() {
        let mut state = DriftStabilityState::new(1);
        state.stale_obligations = 10; // Above threshold

        let mut config = DriftStabilizerConfig::default();
        config.enabled = false;

        let result = check_recursive_drift(&state, &config);
        assert!(result.is_none());
    }

    #[test]
    fn test_generate_drift_injection_prompt() {
        let state = DriftStabilityState::new(5);
        let warnings = "⚠️ Multiple unresolved obligations\n⚠️ Emotional field decay detected";

        let prompt = generate_drift_injection_prompt(&state, warnings);

        assert!(prompt.contains("Narrative Drift Warning"));
        assert!(prompt.contains("unresolved obligations"));
        assert!(prompt.contains("Emotional field decay"));
        assert!(prompt.contains("Current Chapter: 5"));
        assert!(prompt.contains("resolving or advancing"));
        assert!(prompt.contains("intensifying emotional stakes"));
    }

    #[test]
    fn test_metadata_operations() {
        let mut state = DriftStabilityState::new(1);

        state.set_metadata("character_consistency", 0.8);
        state.set_metadata("pacing_consistency", 0.6);

        assert_eq!(state.get_metadata("character_consistency"), Some(0.8));
        assert_eq!(state.get_metadata("pacing_consistency"), Some(0.6));
        assert_eq!(state.get_metadata("nonexistent"), None);
    }

    #[test]
    fn test_advance_chapter() {
        let mut state = DriftStabilityState::new(3);
        assert_eq!(state.current_chapter, 3);

        state.advance_chapter();
        assert_eq!(state.current_chapter, 4);
    }

    #[test]
    fn test_update_metrics() {
        let mut state = DriftStabilityState::new(1);

        let obligations = vec![
            Obligation::new("test1", 0.8, 4),
            Obligation::new("test2", 0.6, 2),
        ];

        let emotional_states = vec![
            EmotionalState::new("guilt", 0.4),
        ];

        state.update_metrics(&obligations, &emotional_states, 0.3, false);

        assert_eq!(state.unresolved_obligation_count, 2);
        assert_eq!(state.stale_obligations, 1); // One obligation with age >= 3
        assert!((state.theme_drift_score - 0.7).abs() < 0.1); // 1.0 - 0.3
        assert!(state.spatial_return_pressure_lost);
    }

    #[test]
    fn test_calculate_trend() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let trend = calculate_trend(&values);
        assert!((trend - 1.0).abs() < 0.1); // Linear increase of 1.0 per step

        let flat_values = vec![5.0, 5.0, 5.0];
        let flat_trend = calculate_trend(&flat_values);
        assert!(flat_trend.abs() < 0.1); // Should be close to 0

        let empty_values = vec![];
        let empty_trend = calculate_trend(&empty_values);
        assert_eq!(empty_trend, 0.0);
    }

    #[test]
    fn test_calculate_variance() {
        let values = vec![2.0, 4.0, 6.0];
        let variance = calculate_variance(&values);
        assert!((variance - 2.67).abs() < 0.1); // Approximate variance

        let identical_values = vec![5.0, 5.0, 5.0];
        let zero_variance = calculate_variance(&identical_values);
        assert!(zero_variance.abs() < 0.1);

        let empty_values = vec![];
        let empty_variance = calculate_variance(&empty_values);
        assert_eq!(empty_variance, 0.0);
    }

    #[test]
    fn test_analyze_drift_trends() {
        // Insufficient history
        let short_history = vec![DriftStabilityState::new(1)];
        let (trend, confidence) = analyze_drift_trends(&short_history);
        assert_eq!(trend, 0.0);
        assert_eq!(confidence, 0.0);

        // Create trend history
        let mut history = Vec::new();
        for i in 0..5 {
            let mut state = DriftStabilityState::new(i);
            state.stale_obligations = i as usize; // Increasing trend
            state.emotional_decay_sum = i as f32 * 0.5; // Increasing trend
            state.theme_drift_score = i as f32 * 0.3; // Increasing trend
            history.push(state);
        }

        let (trend, confidence) = analyze_drift_trends(&history);
        assert!(trend > 0.0); // Should detect increasing trend
        assert!(confidence > 0.5); // Should have reasonable confidence
    }

    #[test]
    fn test_generate_drift_stability_report() {
        let mut state = DriftStabilityState::new(3);
        state.stale_obligations = 2;
        state.emotional_decay_sum = 1.5;
        state.set_metadata("character_consistency", 0.7);

        let config = DriftStabilizerConfig::default();
        let report = generate_drift_stability_report(&state, &config, None);

        assert!(report.contains("RECURSIVE DRIFT STABILITY REPORT"));
        assert!(report.contains("Current Chapter: 3"));
        assert!(report.contains("Stale Obligations: 2"));
        assert!(report.contains("Emotional Decay Sum: 1.50"));
        assert!(report.contains("character_consistency: 0.70"));
        assert!(report.contains("NO DRIFT WARNINGS"));
    }

    #[test]
    fn test_custom_metadata_warnings() {
        let mut state = DriftStabilityState::new(1);
        state.set_metadata("character_consistency", 0.8); // Above 0.7 threshold
        state.set_metadata("pacing_consistency", 0.9); // Above 0.8 threshold

        let config = DriftStabilizerConfig::default();
        let result = check_recursive_drift(&state, &config);

        assert!(result.is_some());
        let warnings = result.unwrap();
        assert!(warnings.contains("Character consistency drift"));
        assert!(warnings.contains("pacing inconsistency"));
    }
}