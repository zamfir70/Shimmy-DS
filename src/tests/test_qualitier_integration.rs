/**
 * Integration tests for Qualitier Adaptive Quality Control System
 * ============================================================
 *
 * Tests the integration between Qualitier and the narrative system components,
 * including quality level transitions, performance monitoring, and settings clamping.
 */

#[cfg(test)]
mod tests {
    use crate::adaptive::{
        Qualitier, QualityLevel, QualityFeature, PerformanceConfig,
        AdaptIQSettings
    };
    use crate::recursive_narrative_assistant::RecursiveNarrativeAssistant;
    use crate::telemetry::Pulse;

    #[test]
    fn test_quality_level_ordering() {
        assert!(QualityLevel::Minimal as u8 < QualityLevel::Standard as u8);
        assert!(QualityLevel::Standard as u8 < QualityLevel::Enhanced as u8);
        assert!(QualityLevel::Enhanced as u8 < QualityLevel::Premium as u8);
    }

    #[test]
    fn test_quality_level_descriptions() {
        assert_eq!(QualityLevel::Minimal.description(), "Minimal (obligation injection only)");
        assert_eq!(QualityLevel::Standard.description(), "Standard (basic emotion tracking)");
        assert_eq!(QualityLevel::Enhanced.description(), "Enhanced (spatial validation, CAPR depth)");
        assert_eq!(QualityLevel::Premium.description(), "Premium (full recursive intelligence)");
    }

    #[test]
    fn test_quality_level_constraints() {
        assert_eq!(QualityLevel::Minimal.max_recursion_depth(), 4);
        assert_eq!(QualityLevel::Standard.max_recursion_depth(), 6);
        assert_eq!(QualityLevel::Enhanced.max_recursion_depth(), 10);
        assert_eq!(QualityLevel::Premium.max_recursion_depth(), 14);

        assert!(QualityLevel::Minimal.pathogen_sensitivity_cap() < QualityLevel::Premium.pathogen_sensitivity_cap());
        assert!(QualityLevel::Standard.affect_assertiveness_cap() < QualityLevel::Premium.affect_assertiveness_cap());
    }

    #[test]
    fn test_quality_feature_enablement() {
        // Minimal level should only enable basic features
        assert!(QualityLevel::Minimal.feature_enabled(QualityFeature::ObligationInjection));
        assert!(!QualityLevel::Minimal.feature_enabled(QualityFeature::EmotionTracking));
        assert!(!QualityLevel::Minimal.feature_enabled(QualityFeature::SpatialValidation));

        // Standard level adds emotion tracking
        assert!(QualityLevel::Standard.feature_enabled(QualityFeature::ObligationInjection));
        assert!(QualityLevel::Standard.feature_enabled(QualityFeature::EmotionTracking));
        assert!(!QualityLevel::Standard.feature_enabled(QualityFeature::SpatialValidation));

        // Enhanced level adds spatial validation and CAPR depth
        assert!(QualityLevel::Enhanced.feature_enabled(QualityFeature::SpatialValidation));
        assert!(QualityLevel::Enhanced.feature_enabled(QualityFeature::CAPRDepthAnalysis));
        assert!(!QualityLevel::Enhanced.feature_enabled(QualityFeature::FullRecursion));

        // Premium level enables everything
        assert!(QualityLevel::Premium.feature_enabled(QualityFeature::FullRecursion));
        assert!(QualityLevel::Premium.feature_enabled(QualityFeature::CacheOptimization));
    }

    #[test]
    fn test_performance_config_defaults() {
        let config = PerformanceConfig::default();
        assert_eq!(config.max_memory_mb, 100);
        assert_eq!(config.max_analysis_time_ms, 50);
        assert!(config.adaptive_quality);
        assert_eq!(config.memory_pressure_threshold, 0.8);
        assert_eq!(config.cpu_threshold, 0.85);
        assert_eq!(config.quality_change_cooldown_ms, 5000);
    }

    #[test]
    fn test_qualitier_creation() {
        let qual = Qualitier::new(100, 50, true);
        assert_eq!(qual.current, QualityLevel::Standard);
        assert_eq!(qual.config.max_memory_mb, 100);
        assert_eq!(qual.config.max_analysis_time_ms, 50);
        assert!(qual.config.adaptive_quality);
        assert_eq!(qual.stats.quality_changes, 0);
    }

    #[test]
    fn test_qualitier_with_config() {
        let config = PerformanceConfig {
            max_memory_mb: 200,
            max_analysis_time_ms: 100,
            adaptive_quality: false,
            memory_pressure_threshold: 0.9,
            cpu_threshold: 0.9,
            quality_change_cooldown_ms: 10000,
        };

        let qual = Qualitier::with_config(config.clone());
        assert_eq!(qual.config.max_memory_mb, 200);
        assert_eq!(qual.config.max_analysis_time_ms, 100);
        assert!(!qual.config.adaptive_quality);
    }

    #[test]
    fn test_settings_clamping_minimal() {
        let qual = Qualitier::new(100, 50, true);
        // Manually set to minimal for testing
        let mut qual = qual;
        qual.current = QualityLevel::Minimal;

        let mut settings = AdaptIQSettings {
            recursion_depth: 20,
            pathogen_sensitivity: 1.0,
            affect_assertiveness: 1.0,
            beat_sampling_rate: 1.0,
            zc_hysteresis_margin: 10,
            eat_resolution_scale: 2.0,
            cache_preference: 0.5,
        };

        qual.clamp_settings(&mut settings);

        assert_eq!(settings.recursion_depth, 4);
        assert_eq!(settings.pathogen_sensitivity, 0.3);
        assert_eq!(settings.affect_assertiveness, 0.2);
        assert_eq!(settings.beat_sampling_rate, 0.4);
        assert_eq!(settings.cache_preference, 0.9);
    }

    #[test]
    fn test_settings_clamping_standard() {
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

        qual.clamp_settings(&mut settings);

        assert!(settings.recursion_depth <= 6);
        assert!(settings.pathogen_sensitivity <= 0.6);
        assert!(settings.eat_resolution_scale <= 1.0);
    }

    #[test]
    fn test_settings_clamping_premium() {
        let mut qual = Qualitier::new(100, 50, true);
        qual.current = QualityLevel::Premium;

        let mut settings = AdaptIQSettings {
            recursion_depth: 20,
            pathogen_sensitivity: 1.0,
            affect_assertiveness: 1.0,
            beat_sampling_rate: 1.0,
            zc_hysteresis_margin: 10,
            eat_resolution_scale: 2.0,
            cache_preference: 0.5,
        };

        qual.clamp_settings(&mut settings);

        assert!(settings.recursion_depth <= 14);
        // Premium should allow full range for most parameters
        assert_eq!(settings.pathogen_sensitivity, 1.0);
        assert_eq!(settings.affect_assertiveness, 1.0);
    }

    #[test]
    fn test_narrative_stress_calculation() {
        let qual = Qualitier::new(100, 50, true);

        let low_stress_pulse = Pulse::with_data(1, 0, 0, 0.9, 30.0, 0.8, 0.9, None);
        let high_stress_pulse = Pulse::with_data(2, 8, 4, 0.1, 80.0, -0.3, 0.2, None);

        let low_stress = qual.calculate_narrative_stress(&low_stress_pulse);
        let high_stress = qual.calculate_narrative_stress(&high_stress_pulse);

        assert!(high_stress > low_stress);
        assert!(low_stress >= 0.0 && low_stress <= 1.0);
        assert!(high_stress >= 0.0 && high_stress <= 1.0);
        assert!(high_stress > 0.7); // High stress pulse should register high stress
    }

    #[test]
    fn test_quality_level_decision_logic() {
        let mut qual = Qualitier::new(100, 50, true);
        let adapt = AdaptIQSettings::default();

        // Low stress, good resources should maintain or upgrade
        let good_pulse = Pulse::with_data(1, 1, 0, 0.8, 30.0, 0.5, 0.8, None);
        qual.decide(&good_pulse, &adapt);
        let good_level = qual.current;

        // High stress should upgrade quality
        let stress_pulse = Pulse::with_data(2, 5, 3, 0.2, 40.0, -0.2, 0.4, None);
        qual.decide(&stress_pulse, &adapt);
        let stress_level = qual.current;

        // Should upgrade under stress (if not already at max)
        if good_level != QualityLevel::Premium {
            assert!(stress_level as u8 >= good_level as u8);
        }
    }

    #[test]
    fn test_manual_quality_setting() {
        let mut qual = Qualitier::new(100, 50, true);
        assert_eq!(qual.current, QualityLevel::Standard);
        assert_eq!(qual.stats.quality_changes, 0);

        qual.set_quality_level(QualityLevel::Premium);
        assert_eq!(qual.current, QualityLevel::Premium);
        assert_eq!(qual.stats.quality_changes, 1);

        // Setting to same level shouldn't increment counter
        qual.set_quality_level(QualityLevel::Premium);
        assert_eq!(qual.stats.quality_changes, 1);
    }

    #[test]
    fn test_adaptive_disable() {
        let mut qual = Qualitier::new(100, 50, false); // Adaptive disabled
        let adapt = AdaptIQSettings::default();
        let stress_pulse = Pulse::with_data(1, 10, 5, 0.1, 200.0, -0.5, 0.2, None);

        let initial_level = qual.current;
        qual.decide(&stress_pulse, &adapt);

        // Should not change level when adaptive is disabled
        assert_eq!(qual.current, initial_level);
    }

    #[test]
    fn test_cooldown_period() {
        let mut qual = Qualitier::new(100, 50, true);
        qual.config.quality_change_cooldown_ms = 10000; // 10 second cooldown

        let adapt = AdaptIQSettings::default();
        let stress_pulse = Pulse::with_data(1, 5, 3, 0.2, 80.0, -0.2, 0.4, None);

        // First decision should be allowed
        let changed1 = qual.decide(&stress_pulse, &adapt);

        // Immediate second decision should be blocked by cooldown
        let changed2 = qual.decide(&stress_pulse, &adapt);

        // At least one should be false (second one blocked)
        if changed1 {
            assert!(!changed2);
        }
    }

    #[test]
    fn test_stats_tracking() {
        let mut qual = Qualitier::new(100, 50, true);
        let adapt = AdaptIQSettings::default();
        let pulse = Pulse::with_data(1, 1, 1, 0.7, 40.0, 0.5, 0.8, None);

        // Make several decisions
        for _ in 0..5 {
            qual.decide(&pulse, &adapt);
        }

        assert_eq!(qual.stats.decision_count, 5);
        assert!(qual.stats.avg_decision_time_ms >= 0.0);
    }

    #[test]
    fn test_quality_distribution() {
        let mut qual = Qualitier::new(100, 50, true);

        // Simulate some time in different quality levels
        qual.stats.time_in_minimal = 10.0;
        qual.stats.time_in_standard = 20.0;
        qual.stats.time_in_enhanced = 30.0;
        qual.stats.time_in_premium = 40.0;

        let distribution = qual.get_quality_distribution();

        assert_eq!(distribution.minimal_percent, 10.0);
        assert_eq!(distribution.standard_percent, 20.0);
        assert_eq!(distribution.enhanced_percent, 30.0);
        assert_eq!(distribution.premium_percent, 40.0);
    }

    #[test]
    fn test_status_report() {
        let qual = Qualitier::new(100, 50, true);
        let status = qual.get_status_report();

        assert_eq!(status.current_level, QualityLevel::Standard);
        assert!(status.adaptive_enabled);
        assert_eq!(status.quality_changes, 0);
        assert!(status.memory_pressure >= 0.0);
        assert!(status.memory_pressure <= 1.0);
    }

    #[test]
    fn test_recursive_narrative_assistant_qualitier_integration() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Test initial state
        assert_eq!(assistant.get_quality_level(), QualityLevel::Standard);
        assert!(assistant.qualitier.config.adaptive_quality);

        // Test manual quality setting
        assistant.set_quality_level(QualityLevel::Enhanced);
        assert_eq!(assistant.get_quality_level(), QualityLevel::Enhanced);

        // Test feature enablement
        assert!(assistant.is_feature_enabled(QualityFeature::ObligationInjection));
        assert!(assistant.is_feature_enabled(QualityFeature::EmotionTracking));
        assert!(assistant.is_feature_enabled(QualityFeature::SpatialValidation));
        assert!(!assistant.is_feature_enabled(QualityFeature::FullRecursion));

        // Set to premium and test full recursion
        assistant.set_quality_level(QualityLevel::Premium);
        assert!(assistant.is_feature_enabled(QualityFeature::FullRecursion));
    }

    #[test]
    fn test_qualitier_stats_json() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        let stats_json = assistant.get_qualitier_stats();
        assert!(stats_json.is_object());
        assert!(stats_json["current_quality_level"].is_string());
        assert!(stats_json["statistics"].is_object());
        assert!(stats_json["time_distribution"].is_object());
        assert!(stats_json["feature_enablement"].is_object());
        assert!(stats_json["quality_level_info"].is_object());

        // Check that all quality levels are described
        assert!(stats_json["quality_level_info"]["minimal"]["description"].is_string());
        assert!(stats_json["quality_level_info"]["standard"]["description"].is_string());
        assert!(stats_json["quality_level_info"]["enhanced"]["description"].is_string());
        assert!(stats_json["quality_level_info"]["premium"]["description"].is_string());
    }

    #[test]
    fn test_quality_assessment_reassessment() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Initialize pulse trace with a pulse
        let pulse = Pulse::with_data(1, 2, 1, 0.7, 50.0, 0.3, 0.8, None);
        assistant.pulse_trace.record(pulse);

        // Test reassessment
        let initial_level = assistant.get_quality_level();
        assistant.reassess_quality();

        // Level might change or stay the same depending on pulse data
        let new_level = assistant.get_quality_level();
        // Just verify the method doesn't crash and produces valid levels
        assert!(matches!(new_level, QualityLevel::Minimal | QualityLevel::Standard | QualityLevel::Enhanced | QualityLevel::Premium));
    }

    #[test]
    fn test_quality_filtered_insights() {
        use crate::recursive_narrative_assistant::{NarrativeInsight, InsightPriority, InsightType, Priority};

        let mut assistant = RecursiveNarrativeAssistant::new();

        // Create test insights of different priorities
        let insights = vec![
            NarrativeInsight {
                insight_type: InsightType::DNAPattern,
                priority: Priority::High, // Maps to Critical
                title: "Critical Issue".to_string(),
                description: "Critical problem".to_string(),
                questions: vec!["Critical question?".to_string()],
                suggestion: "Fix critically".to_string(),
                related_obligations: vec![],
            },
            NarrativeInsight {
                insight_type: InsightType::ConstraintPressure,
                priority: Priority::Medium, // Maps to Important
                title: "Important Issue".to_string(),
                description: "Important problem".to_string(),
                questions: vec!["Important question?".to_string()],
                suggestion: "Fix importantly".to_string(),
                related_obligations: vec![],
            },
            NarrativeInsight {
                insight_type: InsightType::RecursionPattern,
                priority: Priority::Low, // Maps to Minor
                title: "Minor Issue".to_string(),
                description: "Minor problem".to_string(),
                questions: vec!["Minor question?".to_string()],
                suggestion: "Fix minorly".to_string(),
                related_obligations: vec![],
            },
        ];

        // Test filtering at Minimal level
        assistant.set_quality_level(QualityLevel::Minimal);
        let minimal_filtered = assistant.get_quality_filtered_insights(insights.clone());
        // Should only have critical insights and be limited to 3
        assert!(minimal_filtered.len() <= 3);
        if !minimal_filtered.is_empty() {
            assert!(minimal_filtered.iter().all(|i| matches!(i.priority, Priority::High)));
        }

        // Test filtering at Standard level
        assistant.set_quality_level(QualityLevel::Standard);
        let standard_filtered = assistant.get_quality_filtered_insights(insights.clone());
        // Should have critical and important, limited to 5
        assert!(standard_filtered.len() <= 5);

        // Test filtering at Premium level
        assistant.set_quality_level(QualityLevel::Premium);
        let premium_filtered = assistant.get_quality_filtered_insights(insights.clone());
        // Should have all insights
        assert_eq!(premium_filtered.len(), insights.len());
    }

    #[test]
    fn test_performance_config_update() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        let new_config = PerformanceConfig {
            max_memory_mb: 200,
            max_analysis_time_ms: 100,
            adaptive_quality: false,
            memory_pressure_threshold: 0.9,
            cpu_threshold: 0.9,
            quality_change_cooldown_ms: 10000,
        };

        assistant.update_performance_config(new_config.clone());

        // Check that configuration was updated
        assert_eq!(assistant.config.performance_config.max_memory_mb, 200);
        assert!(!assistant.config.performance_config.adaptive_quality);
        assert_eq!(assistant.qualitier.config.max_memory_mb, 200);
    }

    #[test]
    fn test_stats_reset() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Make a decision to generate some stats
        let pulse = Pulse::with_data(1, 1, 1, 0.7, 40.0, 0.5, 0.8, None);
        assistant.pulse_trace.record(pulse);
        assistant.reassess_quality();

        // Verify stats exist
        assert!(assistant.qualitier.get_stats().decision_count > 0);

        // Reset stats
        assistant.reset_qualitier_stats();

        // Verify stats were reset
        assert_eq!(assistant.qualitier.get_stats().decision_count, 0);
        assert_eq!(assistant.qualitier.get_stats().quality_changes, 0);
    }
}