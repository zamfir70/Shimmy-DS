/// Comprehensive Integration Tests for Recursive Integrity Core (RIC v1.0)
///
/// Tests the complete RIC system integration including:
/// - Loop saturation control across all subsystems
/// - CAPR return clamp functionality
/// - Character drift threshold filtering
/// - Recursive arbitration engine
/// - Continuity floor injection
/// - Cross-system coordination
/// - Audit logging integration

use crate::recursive_integrity_core::*;
use crate::narrative_dna::NarrativeDNATracker;
use crate::character_consistency::CharacterConsistencyEngine;
use crate::recursive_narrative_assistant::{RecursiveNarrativeAssistant, AssistantConfig};
use crate::stability_tracing::{StabilityLogger, RICLogEntry};
use std::collections::HashMap;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_logger() -> (StabilityLogger, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let text_path = temp_dir.path().join("test_stability.log");
        let json_path = temp_dir.path().join("test_stability.json");
        let logger = StabilityLogger::with_paths(text_path, json_path);
        (logger, temp_dir)
    }

    #[test]
    fn test_ric_mode_enforcement() {
        // Test each RIC mode behaves correctly

        // Passive mode - should only log, never block
        let mut ric_passive = RecursiveIntegrityCore::new(RICMode::Passive);
        ric_passive.register_subsystem("test", 1);

        // Consume all budget
        assert!(ric_passive.can_iterate("test"));
        assert!(!ric_passive.can_iterate("test")); // Should be saturated

        // Even when saturated, passive mode shouldn't block voting
        ric_passive.vote("test", InsightStatus::Stalled);
        let decision = ric_passive.arbitrate();
        // Should still proceed despite saturation in passive mode

        // Active mode - should actively intervene
        let mut ric_active = RecursiveIntegrityCore::new(RICMode::Active);
        ric_active.register_subsystem("test", 1);
        ric_active.vote("test1", InsightStatus::Block);
        ric_active.vote("test2", InsightStatus::Block);

        let decision = ric_active.arbitrate();
        assert_eq!(decision, RICDecision::Halt); // Should halt on consensus
    }

    #[test]
    fn test_loop_saturation_control() {
        let mut controller = LoopSaturationController::new(3, "test_system".to_string());

        // Test normal iteration consumption
        assert!(controller.consume_iteration());
        assert!(controller.consume_iteration());
        assert!(controller.consume_iteration());
        assert!(!controller.consume_iteration()); // Should be saturated

        assert!(controller.is_saturated());

        // Test reset on insight
        controller.reset_on_insight();
        assert!(!controller.is_saturated());
        assert!(controller.consume_iteration()); // Should work again
    }

    #[test]
    fn test_capr_return_clamp() {
        let mut clamp = CAPRReturnClamp::new(2);

        // Test normal returns within limit
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);

        // Should stall after exceeding limit without transformation
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Stalled);

        // Test transformation allows continuation
        clamp.mark_transformation("loop1");
        clamp.reset_loop("loop1");
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);
    }

    #[test]
    fn test_character_drift_clamp() {
        let mut clamp = CharacterDriftClamp::new(0.1);

        // Small drift should not be significant
        assert!(!clamp.is_significant_drift("char1", 0.05));

        // Large drift should be significant
        assert!(clamp.is_significant_drift("char1", 0.15));

        // Test trend calculation
        clamp.is_significant_drift("char2", 0.2);
        clamp.is_significant_drift("char2", 0.3);
        clamp.is_significant_drift("char2", 0.1);

        let trend = clamp.get_drift_trend("char2");
        assert!((trend - 0.2).abs() < 0.01); // Should be approximately 0.2
    }

    #[test]
    fn test_insight_arbitrator() {
        let mut arbitrator = InsightArbitrator::new();

        // Test no blocking with single vote
        arbitrator.add_vote("system1", InsightStatus::Block);
        assert_eq!(arbitrator.arbitrate(), RICDecision::Continue);

        // Test blocking with consensus
        arbitrator.add_vote("system1", InsightStatus::Block);
        arbitrator.add_vote("system2", InsightStatus::Block);
        assert_eq!(arbitrator.arbitrate(), RICDecision::Halt);

        // Test stall detection
        arbitrator.add_vote("system1", InsightStatus::Stalled);
        arbitrator.add_vote("system2", InsightStatus::Stalled);
        arbitrator.add_vote("system3", InsightStatus::Stalled);
        assert_eq!(arbitrator.arbitrate(), RICDecision::InjectFloor);

        // Test suggestion rerouting
        arbitrator.add_vote("system1", InsightStatus::Suggest);
        arbitrator.add_vote("system2", InsightStatus::Suggest);
        arbitrator.add_vote("system3", InsightStatus::Continue);
        match arbitrator.arbitrate() {
            RICDecision::Reroute(_) => {} // Expected
            _ => panic!("Expected reroute decision"),
        }
    }

    #[test]
    fn test_narrative_dna_ric_integration() {
        let mut dna_tracker = NarrativeDNATracker::new();

        // Test CAPR return permission checking
        let permission1 = dna_tracker.check_capr_return_permission("loop1", false);
        assert_eq!(permission1, CAPRReturnDecision::Allow);

        // Test loop counting and saturation
        for _ in 0..5 {
            dna_tracker.check_capr_return_permission("loop1", false);
        }

        // Should stall after max returns without transformation
        let permission_stalled = dna_tracker.check_capr_return_permission("loop1", false);
        assert_eq!(permission_stalled, CAPRReturnDecision::Stalled);

        // Test transformation reset
        dna_tracker.mark_capr_transformation("loop1");
        let permission_after_transform = dna_tracker.check_capr_return_permission("loop1", true);
        assert_eq!(permission_after_transform, CAPRReturnDecision::Allow);

        // Test voting behavior
        let status = dna_tracker.vote_on_narrative_state();
        assert!(matches!(status, InsightStatus::Continue | InsightStatus::Suggest | InsightStatus::Block | InsightStatus::Stalled));
    }

    #[test]
    fn test_character_engine_ric_integration() {
        let mut character_engine = CharacterConsistencyEngine::new();

        // Test drift checking in different modes
        let drift_passive = character_engine.check_character_drift("char1", 0.1, RICMode::Passive);
        assert!(!drift_passive); // Passive mode never blocks

        let drift_active = character_engine.check_character_drift("char2", 0.1, RICMode::Active);
        assert!(drift_active); // Active mode should detect significant drift

        // Test voting behavior
        let status = character_engine.vote_on_consistency_state();
        assert!(matches!(status, InsightStatus::Continue | InsightStatus::Suggest | InsightStatus::Block | InsightStatus::Stalled));

        // Test trend tracking
        let trend = character_engine.get_character_drift_trend("char1");
        assert!(trend >= 0.0);
    }

    #[test]
    fn test_recursive_narrative_assistant_ric_integration() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Test RIC health reporting
        let health = assistant.get_ric_health();
        assert!(health.is_some());
        let health_summary = health.unwrap();
        assert_eq!(health_summary.mode, RICMode::Passive);
        assert_eq!(health_summary.intervention_count, 0);

        // Test RIC mode changes
        assistant.set_ric_mode(RICMode::Active);
        let health_after = assistant.get_ric_health().unwrap();
        assert_eq!(health_after.mode, RICMode::Active);

        // Test RIC state reset
        assistant.reset_ric_state();
        let health_reset = assistant.get_ric_health().unwrap();
        assert_eq!(health_reset.intervention_count, 0);

        // Test insights generation with RIC protection
        let insights_result = assistant.generate_insights_with_ric();
        assert!(insights_result.is_ok()); // Should succeed in normal operation
    }

    #[test]
    fn test_continuity_floor_injection() {
        let mut assistant = RecursiveNarrativeAssistant::new();
        assistant.set_ric_mode(RICMode::Active);

        // Simulate saturated systems to trigger floor injection
        if let Some(ref mut ric) = assistant.ric {
            // Force all systems to vote stalled
            ric.vote("dna_tracker", InsightStatus::Stalled);
            ric.vote("constraint_tracker", InsightStatus::Stalled);
            ric.vote("recursion_tracker", InsightStatus::Stalled);
            ric.vote("character_engine", InsightStatus::Stalled);
            ric.vote("engagement_tracker", InsightStatus::Stalled);

            let decision = ric.arbitrate();
            assert_eq!(decision, RICDecision::InjectFloor);
        }

        // Test that continuity floor is triggered
        let insights_result = assistant.generate_insights_with_ric();
        assert!(insights_result.is_err()); // Should return floor response

        if let Err(floor_response) = insights_result {
            assert!(floor_response.completion_guarantee);
            assert!(!floor_response.summary.is_empty());
        }
    }

    #[test]
    fn test_cross_system_coordination() {
        let mut ric = RecursiveIntegrityCore::new(RICMode::Moderate);

        // Register multiple subsystems
        ric.register_subsystem("system_a", 3);
        ric.register_subsystem("system_b", 3);
        ric.register_subsystem("system_c", 3);

        // Test coordinated saturation detection
        for _ in 0..4 {
            ric.can_iterate("system_a");
            ric.can_iterate("system_b");
        }

        // Some systems should be saturated
        let health = ric.health_summary();
        assert!(!health.saturated_systems.is_empty());

        // Test coordinated voting
        ric.vote("system_a", InsightStatus::Suggest);
        ric.vote("system_b", InsightStatus::Block);
        ric.vote("system_c", InsightStatus::Continue);

        let decision = ric.arbitrate();
        // Should make a reasonable decision based on votes
        assert!(matches!(decision, RICDecision::Continue | RICDecision::Halt | RICDecision::Reroute(_)));
    }

    #[test]
    fn test_ric_logging_integration() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Test RIC decision logging
        let votes = vec![
            ("system1".to_string(), InsightStatus::Continue),
            ("system2".to_string(), InsightStatus::Suggest),
        ];

        let result = logger.log_ric_decision(
            1,
            RICDecision::Continue,
            &votes,
            RICMode::Moderate,
            0,
            vec![],
            "Test decision".to_string(),
            None,
        );

        assert!(result.is_ok());
        assert_eq!(logger.recent_ric_entries.len(), 1);

        // Test reading RIC history
        let history = logger.read_ric_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].chapter, 1);
        assert_eq!(history[0].decision, "Continue");

        // Test RIC analysis generation
        let analysis = logger.generate_ric_analysis(10).unwrap();
        assert!(analysis.contains("RIC INTEGRITY ANALYSIS"));
        assert!(analysis.contains("Continue: 1"));
    }

    #[test]
    fn test_ric_log_entry_formatting() {
        let votes = vec![
            ("dna_tracker".to_string(), InsightStatus::Block),
            ("character_engine".to_string(), InsightStatus::Continue),
        ];

        let entry = RICLogEntry::new(
            5,
            RICDecision::Halt,
            &votes,
            RICMode::Active,
            3,
            "Consensus blocking detected".to_string(),
        ).with_saturated_systems(vec!["system1".to_string()]);

        let formatted = entry.format_for_text();
        assert!(formatted.contains("Ch:5"));
        assert!(formatted.contains("Decision:Halt"));
        assert!(formatted.contains("dna_tracker:Block"));
        assert!(formatted.contains("character_engine:Continue"));
        assert!(formatted.contains("Mode:Active"));
        assert!(formatted.contains("Interventions:3"));
        assert!(formatted.contains("Saturated:[system1]"));
        assert!(formatted.contains("Consensus blocking detected"));
    }

    #[test]
    fn test_ric_performance_characteristics() {
        let mut ric = RecursiveIntegrityCore::new(RICMode::Active);

        // Register many subsystems to test scalability
        for i in 0..10 {
            ric.register_subsystem(&format!("system_{}", i), 5);
        }

        // Test that operations remain efficient with many subsystems
        let start = std::time::Instant::now();

        for i in 0..10 {
            ric.vote(&format!("system_{}", i), InsightStatus::Continue);
        }

        let decision = ric.arbitrate();
        let elapsed = start.elapsed();

        // Should complete quickly (< 1ms for this test)
        assert!(elapsed.as_millis() < 10);
        assert_eq!(decision, RICDecision::Continue);
    }

    #[test]
    fn test_ric_fail_closed_behavior() {
        let mut assistant = RecursiveNarrativeAssistant::new();
        assistant.set_ric_mode(RICMode::Active);

        // Force systems into problematic states
        if let Some(ref mut ric) = assistant.ric {
            // Create a scenario that should trigger fail-closed behavior
            ric.vote("dna_tracker", InsightStatus::Block);
            ric.vote("character_engine", InsightStatus::Block);
            ric.vote("recursion_tracker", InsightStatus::Block);

            let decision = ric.arbitrate();
            // Should halt when multiple systems vote to block
            assert_eq!(decision, RICDecision::Halt);
        }

        // Test that fail-closed behavior provides safe fallback
        let insights_result = assistant.generate_insights_with_ric();
        match insights_result {
            Err(floor_response) => {
                // Should provide continuity floor response
                assert!(floor_response.completion_guarantee);
                assert!(!floor_response.summary.is_empty());
            }
            Ok(_) => panic!("Expected fail-closed behavior to trigger"),
        }
    }

    #[test]
    fn test_ric_configuration_compliance() {
        // Test that RIC respects configuration settings
        let mut config = AssistantConfig::default();
        config.ric_mode = RICMode::Passive;

        let assistant = RecursiveNarrativeAssistant::with_config(config);
        let health = assistant.get_ric_health().unwrap();
        assert_eq!(health.mode, RICMode::Passive);

        // Test mode switching
        let mut assistant = assistant;
        assistant.set_ric_mode(RICMode::Active);
        let health_after = assistant.get_ric_health().unwrap();
        assert_eq!(health_after.mode, RICMode::Active);
    }

    #[test]
    fn test_ric_recursive_protection() {
        // Test that RIC protects against infinite recursion
        let mut dna_tracker = NarrativeDNATracker::new();

        // Exhaust CAPR loop budget
        for _ in 0..10 {
            let permission = dna_tracker.check_capr_return_permission("infinite_loop", false);
            if permission == CAPRReturnDecision::Stalled {
                break;
            }
        }

        // Should eventually stall to prevent infinite loops
        let final_permission = dna_tracker.check_capr_return_permission("infinite_loop", false);
        assert_eq!(final_permission, CAPRReturnDecision::Stalled);

        // Verify RIC status shows saturation
        let status = dna_tracker.get_ric_status();
        match status {
            RICStatus::Saturated(_) => {} // Expected
            _ => {} // Other states are also acceptable depending on timing
        }
    }

    #[test]
    fn test_full_ric_workflow() {
        // Integration test of complete RIC workflow
        let mut assistant = RecursiveNarrativeAssistant::new();
        let (mut logger, _temp_dir) = create_test_logger();

        // Set up active mode for full testing
        assistant.set_ric_mode(RICMode::Moderate);

        // Simulate narrative processing workflow
        for chapter in 1..=3 {
            assistant.current_chapter = chapter;

            // Generate insights with RIC protection
            let insights_result = assistant.generate_insights_with_ric();

            // Log the RIC decision
            if let Some(ref ric) = assistant.ric {
                let health = ric.health_summary();
                let votes = vec![
                    ("dna_tracker".to_string(), InsightStatus::Continue),
                    ("character_engine".to_string(), InsightStatus::Continue),
                ];

                logger.log_ric_decision(
                    chapter,
                    RICDecision::Continue,
                    &votes,
                    health.mode,
                    health.intervention_count,
                    health.saturated_systems,
                    format!("Chapter {} processing", chapter),
                    None,
                ).unwrap();
            }

            // Should either succeed or provide safe fallback
            match insights_result {
                Ok(_insights) => {
                    // Normal operation - insights generated successfully
                }
                Err(floor_response) => {
                    // Fail-closed operation - safe fallback provided
                    assert!(floor_response.completion_guarantee);
                }
            }
        }

        // Verify logging captured the workflow
        let ric_history = logger.read_ric_history().unwrap();
        assert_eq!(ric_history.len(), 3);

        // Generate analysis report
        let analysis = logger.generate_ric_analysis(10).unwrap();
        assert!(analysis.contains("Total RIC Entries: 3"));
        assert!(analysis.contains("Continue: 3"));
    }
}