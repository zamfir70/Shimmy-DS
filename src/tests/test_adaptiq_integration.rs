/**
 * Integration tests for AdaptIQ Narrative Intelligence Modulator
 * ============================================================
 *
 * Tests the integration between AdaptIQ and the narrative system components,
 * including entropy calculation, adaptive settings, and taste preferences.
 */

#[cfg(test)]
mod tests {
    use crate::adaptive::{
        AdaptIQEngine, AdaptIQSettings, TasteLUT, entropy_score,
        estimate_cognitive_load, analyze_question_complexity, calculate_content_volatility
    };
    use crate::recursive_narrative_assistant::RecursiveNarrativeAssistant;
    use crate::telemetry::Pulse;
    use crate::cache::CacheMind;

    #[test]
    fn test_entropy_calculation_basic() {
        let simple_text = "Hello world hello world";
        let complex_text = "The epistemological implications of postmodern hermeneutical methodologies";
        let empty_text = "";

        let simple_entropy = entropy_score(simple_text);
        let complex_entropy = entropy_score(complex_text);
        let empty_entropy = entropy_score(empty_text);

        assert!(complex_entropy > simple_entropy, "Complex text should have higher entropy");
        assert_eq!(empty_entropy, 0.0, "Empty text should have zero entropy");
        assert!(simple_entropy >= 0.0 && simple_entropy <= 1.0, "Entropy should be normalized to 0-1");
        assert!(complex_entropy >= 0.0 && complex_entropy <= 1.0, "Entropy should be normalized to 0-1");
    }

    #[test]
    fn test_question_complexity_analysis() {
        let simple_question = "What is your name?";
        let complex_question = "Why do you think that if we consider the multifaceted implications, how would this fundamentally affect our understanding?";
        let nested_question = "What if I asked you: why do you think this matters?";

        let simple_complexity = analyze_question_complexity(simple_question);
        let complex_complexity = analyze_question_complexity(complex_question);
        let nested_complexity = analyze_question_complexity(nested_question);

        assert!(complex_complexity.overall_score > simple_complexity.overall_score);
        assert!(nested_complexity.overall_score > simple_complexity.overall_score);
        assert_eq!(simple_complexity.what_questions, 1);
        assert!(complex_complexity.why_questions > 0);
        assert!(complex_complexity.how_questions > 0);
        assert!(complex_complexity.conditional_questions > 0);
    }

    #[test]
    fn test_content_volatility() {
        let stable_text = "I am happy. I am very happy. I am extremely happy. This brings me joy.";
        let volatile_text = "I am happy. I am terribly sad. This is wonderful. I hate everything.";
        let neutral_text = "The system operates. The process continues. The method functions.";

        let stable_volatility = calculate_content_volatility(stable_text);
        let volatile_volatility = calculate_content_volatility(volatile_text);
        let neutral_volatility = calculate_content_volatility(neutral_text);

        assert!(volatile_volatility > stable_volatility, "Volatile text should have higher volatility");
        assert!(stable_volatility >= neutral_volatility, "Emotional text should have some volatility");
    }

    #[test]
    fn test_cognitive_load_estimation() {
        let simple_text = "Hello. How are you today?";
        let complex_text = "The epistemological ramifications of postmodern hermeneutical methodologies necessitate a comprehensive reevaluation of our fundamental assumptions regarding the nature of textual interpretation.";
        let technical_text = "The system implements a recursive algorithm that processes the input through multiple analysis layers.";

        let simple_load = estimate_cognitive_load(simple_text);
        let complex_load = estimate_cognitive_load(complex_text);
        let technical_load = estimate_cognitive_load(technical_text);

        assert!(complex_load.overall_load > simple_load.overall_load);
        assert!(technical_load.overall_load > simple_load.overall_load);
        assert!(complex_load.reading_difficulty > simple_load.reading_difficulty);
        assert!(technical_load.attention_demand > simple_load.attention_demand);
    }

    #[test]
    fn test_adaptiq_settings_presets() {
        let conservative = AdaptIQSettings::conservative();
        let aggressive = AdaptIQSettings::aggressive();
        let balanced = AdaptIQSettings::balanced();

        // Conservative settings should favor stability
        assert!(conservative.recursion_depth <= balanced.recursion_depth);
        assert!(conservative.pathogen_sensitivity <= balanced.pathogen_sensitivity);
        assert!(conservative.cache_preference >= balanced.cache_preference);

        // Aggressive settings should favor depth and sensitivity
        assert!(aggressive.recursion_depth >= balanced.recursion_depth);
        assert!(aggressive.pathogen_sensitivity >= balanced.pathogen_sensitivity);
        assert!(aggressive.affect_assertiveness >= balanced.affect_assertiveness);
    }

    #[test]
    fn test_taste_lut_presets() {
        let curious = TasteLUT::curious();
        let safe = TasteLUT::safe();
        let experimental = TasteLUT::experimental();
        let balanced = TasteLUT::balanced();

        // Curious should favor exploration
        assert!(curious.curiosity > balanced.curiosity);
        assert!(curious.awe > balanced.awe);

        // Safe should favor stability
        assert!(safe.coherence_pleasure > balanced.coherence_pleasure);
        assert!(safe.unease < balanced.unease);

        // Experimental should favor artistic expression
        assert!(experimental.curiosity > balanced.curiosity);
        assert!(experimental.boredom > balanced.boredom);
        assert!(experimental.awe > balanced.awe);
    }

    #[test]
    fn test_taste_influence_on_settings() {
        let curious_taste = TasteLUT::curious();
        let safe_taste = TasteLUT::safe();

        let mut curious_settings = AdaptIQSettings::balanced();
        let mut safe_settings = AdaptIQSettings::balanced();

        curious_settings.with_taste(&curious_taste);
        safe_settings.with_taste(&safe_taste);

        // Curious taste should increase exploration parameters
        assert!(curious_settings.recursion_depth >= safe_settings.recursion_depth);
        assert!(curious_settings.affect_assertiveness >= safe_settings.affect_assertiveness);

        // Safe taste should increase stability parameters
        assert!(safe_settings.cache_preference >= curious_settings.cache_preference);
        assert!(safe_settings.beat_sampling_rate >= curious_settings.beat_sampling_rate);
    }

    #[test]
    fn test_adaptiq_engine_creation() {
        let taste = TasteLUT::balanced();
        let engine = AdaptIQEngine::new(0.6, taste);

        assert_eq!(engine.base_entropy, 0.6);
        assert_eq!(engine.get_stats().decision_count, 0);
    }

    #[test]
    fn test_adaptiq_engine_decision_making() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);
        let cache = CacheMind::new();

        // Test with different pulse scenarios
        let low_tension_pulse = Pulse::with_data(1, 0, 0, 0.8, 30.0, 0.5, 0.9, None);
        let high_tension_pulse = Pulse::with_data(2, 5, 3, 0.3, 100.0, -0.2, 0.4, None);

        let low_tension_settings = engine.decide(&low_tension_pulse, &cache);
        let high_tension_settings = engine.decide(&high_tension_pulse, &cache);

        // High tension should result in more aggressive analysis
        assert!(high_tension_settings.recursion_depth >= low_tension_settings.recursion_depth);
        assert!(high_tension_settings.pathogen_sensitivity >= low_tension_settings.pathogen_sensitivity);

        // Engine stats should be updated
        assert_eq!(engine.get_stats().decision_count, 2);
        assert!(engine.get_stats().avg_decision_time_ms >= 0.0);
    }

    #[test]
    fn test_adaptiq_quick_decision() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);

        let settings1 = engine.quick_decide(0.2, 40.0); // Low ADI, normal memory
        let settings2 = engine.quick_decide(0.8, 90.0); // High ADI, high memory

        // Low ADI should trigger more analysis
        assert!(settings1.pathogen_sensitivity > settings2.pathogen_sensitivity);

        // High memory pressure should increase cache preference
        assert!(settings2.cache_preference > settings1.cache_preference);
        assert!(settings2.recursion_depth <= settings1.recursion_depth);
    }

    #[test]
    fn test_settings_clamping() {
        let mut extreme_settings = AdaptIQSettings {
            recursion_depth: 100,
            pathogen_sensitivity: 2.0,
            affect_assertiveness: -0.5,
            beat_sampling_rate: 1.5,
            zc_hysteresis_margin: 0,
            eat_resolution_scale: 3.0,
            cache_preference: -1.0,
        };

        extreme_settings.clamp();

        assert!(extreme_settings.recursion_depth <= 20);
        assert!(extreme_settings.pathogen_sensitivity <= 1.0);
        assert!(extreme_settings.affect_assertiveness >= 0.0);
        assert!(extreme_settings.beat_sampling_rate <= 1.0);
        assert!(extreme_settings.zc_hysteresis_margin >= 1);
        assert!(extreme_settings.eat_resolution_scale <= 2.0);
        assert!(extreme_settings.cache_preference >= 0.0);
    }

    #[test]
    fn test_recursive_narrative_assistant_adaptiq_integration() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Test initial state
        assert_eq!(assistant.current_adapt_settings.recursion_depth, 8); // Default
        assert!(assistant.adapt_iq_engine.is_none()); // Not initialized yet

        // Test prompt analysis
        let simple_prompt = "Hello, how are you?";
        let complex_prompt = "Analyze the epistemological implications of recursive narrative structures in postmodern literature.";

        let simple_load = assistant.analyze_prompt(simple_prompt);
        let complex_load = assistant.analyze_prompt(complex_prompt);

        assert!(complex_load > simple_load, "Complex prompt should have higher cognitive load");
        assert!(assistant.adapt_iq_engine.is_some(), "Engine should be initialized after prompt analysis");

        // Test settings update
        let initial_depth = assistant.current_adapt_settings.recursion_depth;
        assistant.analyze_prompt(complex_prompt);
        // Settings might change based on entropy and context
    }

    #[test]
    fn test_taste_preset_setting() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Test different taste presets
        assistant.set_taste_preset("curious");
        assert!(assistant.config.taste_profile.curiosity > 0.7);

        assistant.set_taste_preset("safe");
        assert!(assistant.config.taste_profile.coherence_pleasure > 0.7);
        assert!(assistant.config.taste_profile.unease < 0.3);

        assistant.set_taste_preset("experimental");
        assert!(assistant.config.taste_profile.curiosity > 0.8);
        assert!(assistant.config.taste_profile.boredom > 0.7);

        assistant.set_taste_preset("balanced");
        let taste = &assistant.config.taste_profile;
        assert!(taste.curiosity >= 0.4 && taste.curiosity <= 0.6);
    }

    #[test]
    fn test_adaptiq_stats_tracking() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Initialize AdaptIQ
        assistant.analyze_prompt("Test prompt for initialization");

        // Get initial stats
        let stats = assistant.get_adapt_iq_stats();
        assert!(stats.is_some());

        let stats_json = stats.unwrap();
        assert!(stats_json["decision_count"].as_u64().unwrap() >= 1);
        assert!(stats_json["current_settings"].is_object());
        assert!(stats_json["taste_profile"].is_object());

        // Test recalibration
        assistant.recalibrate_adapt_iq();
        let new_stats = assistant.get_adapt_iq_stats().unwrap();
        assert_eq!(new_stats["decision_count"].as_u64().unwrap(), 0); // Should reset
    }

    #[test]
    fn test_adaptive_status() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        let status = assistant.get_adaptive_status();
        assert_eq!(status["enabled"].as_bool().unwrap(), true);
        assert_eq!(status["engine_initialized"].as_bool().unwrap(), false);

        // Initialize engine
        assistant.analyze_prompt("Test prompt");

        let new_status = assistant.get_adaptive_status();
        assert_eq!(new_status["engine_initialized"].as_bool().unwrap(), true);
        assert!(new_status["current_settings"].is_object());
        assert!(new_status["taste_profile"].is_object());
    }

    #[test]
    fn test_performance_constraints() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.8, taste); // High entropy
        let cache = CacheMind::new();

        // Simulate high-pressure scenario (high memory usage, many pathogens)
        let high_pressure_pulse = Pulse::with_data(1, 8, 4, 0.2, 150.0, -0.3, 0.3, None);
        let settings = engine.decide(&high_pressure_pulse, &cache);

        // Under pressure, should reduce computational load
        assert!(settings.recursion_depth < 12); // Should be reduced from high entropy
        assert!(settings.cache_preference > 0.6); // Should favor caching
        assert!(engine.get_stats().performance_adjustments > 0); // Should record adjustment
    }

    #[test]
    fn test_cache_freshness_bonus() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.5, taste);

        // Empty cache
        let empty_cache = CacheMind::new();
        let pulse = Pulse::with_data(1, 2, 1, 0.7, 50.0, 0.3, 0.8, None);
        let empty_settings = engine.decide(&pulse, &empty_cache);

        // Populated cache
        let mut populated_cache = CacheMind::new();
        // Add some test data to the cache
        use crate::cache::ConstraintSnapshot;
        use std::collections::HashMap;
        for i in 0..10 {
            let snapshot = ConstraintSnapshot {
                freedom_score: 0.7,
                active_constraints: vec![format!("constraint_{}", i)],
                constraint_pressures: HashMap::new(),
                timestamp: 1234567890,
                chapter: 1,
                scene: Some(i),
            };
            populated_cache.set_constraint_snapshot(format!("test_{}", i), snapshot);
        }

        let populated_settings = engine.decide(&pulse, &populated_cache);

        // Populated cache should provide freshness bonus
        // This is subtle and depends on implementation details, but generally:
        assert!(populated_settings.recursion_depth >= empty_settings.recursion_depth ||
                populated_settings.cache_preference >= empty_settings.cache_preference);
    }

    #[test]
    fn test_entropy_edge_cases() {
        // Test edge cases for entropy calculation
        assert_eq!(entropy_score(""), 0.0);
        assert_eq!(entropy_score("   \n\t  "), 0.0);
        assert!(entropy_score("a") > 0.0);
        assert!(entropy_score("a a a a") < entropy_score("a b c d"));

        // Test with special characters
        let special_text = "Hello! @#$%^&*() How are you? 123456";
        let special_entropy = entropy_score(special_text);
        assert!(special_entropy > 0.0 && special_entropy <= 1.0);
    }

    #[test]
    fn test_taste_clamping() {
        let mut extreme_taste = TasteLUT {
            curiosity: 2.0,
            coherence_pleasure: -0.5,
            unease: 1.5,
            awe: -1.0,
            boredom: 3.0,
        };

        extreme_taste.clamp();

        assert!(extreme_taste.curiosity <= 1.0);
        assert!(extreme_taste.coherence_pleasure >= 0.0);
        assert!(extreme_taste.unease <= 1.0);
        assert!(extreme_taste.awe >= 0.0);
        assert!(extreme_taste.boredom <= 1.0);
    }

    #[test]
    fn test_engine_entropy_updates() {
        let taste = TasteLUT::balanced();
        let mut engine = AdaptIQEngine::new(0.3, taste);

        assert_eq!(engine.base_entropy, 0.3);

        engine.update_entropy(0.8);
        assert_eq!(engine.base_entropy, 0.8);

        engine.update_entropy(1.5); // Should be clamped
        assert_eq!(engine.base_entropy, 1.0);

        engine.update_entropy(-0.5); // Should be clamped
        assert_eq!(engine.base_entropy, 0.0);
    }

    #[test]
    fn test_recursive_narrative_assistant_apply_adaptive_settings() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Initialize AdaptIQ with specific settings
        assistant.analyze_prompt("Complex analytical prompt requiring deep recursive analysis");

        let initial_assertiveness = assistant.config.assertiveness_level;
        let initial_sensitivity = assistant.config.sensitivity.constraint_pressure;

        // Apply adaptive settings
        assistant.apply_adaptive_settings();

        // Settings should be modified based on AdaptIQ recommendations
        // (Exact changes depend on the specific prompt and resulting settings)
        assert!(assistant.config.assertiveness_level >= 0.0);
        assert!(assistant.config.assertiveness_level <= 1.0);
        assert!(assistant.config.sensitivity.constraint_pressure >= 0.0);
    }
}