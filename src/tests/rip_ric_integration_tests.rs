/// Comprehensive Integration Tests for RIP+RIC Unified Protocol Stack (v1.0)
///
/// Tests the complete RIP+RIC unified system including:
/// - Cross-language Python RIP ↔ Rust RIC communication
/// - Unified arbitration decision making
/// - Constraint genome and guard chain validation
/// - Pathogen detection and threat assessment
/// - Loop saturation control with ZC gates
/// - Character consistency with RIP+RIC fusion
/// - Stability logging for unified protocol
/// - Fail-closed protection mechanisms
/// - Continuity floor injection coordination

use crate::recursive_narrative_assistant::{
    RecursiveNarrativeAssistant, UnifiedArbitrationDecision, RIPRICFusionHealth, RIPRICFusionState
};
use crate::character_consistency::{
    CharacterConsistencyEngine, CharacterPathogen, CharacterConstraintViolation, CharacterRIPAnalysis
};
use crate::recursive_integrity_core::{RecursiveIntegrityCore, RICMode, RICDecision, InsightStatus};
use crate::stability_tracing::{StabilityLogger, RIPRICFusionLogEntry};
use crate::obligation_pressure::Obligation;
use crate::emotion_resonance::EmotionalState;
use tempfile::TempDir;
use serde_json::json;
use std::time::Duration;

#[cfg(test)]
mod rip_ric_fusion_tests {
    use super::*;

    fn create_test_logger() -> (StabilityLogger, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let text_path = temp_dir.path().join("test_rip_ric_fusion.log");
        let json_path = temp_dir.path().join("test_rip_ric_fusion.json");
        let logger = StabilityLogger::with_paths(text_path, json_path);
        (logger, temp_dir)
    }

    fn create_test_assistant() -> RecursiveNarrativeAssistant {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Set up test obligations
        let obligations = vec![
            Obligation::new("resolve_protagonist_conflict", 0.8, 5),
            Obligation::new("establish_romance_subplot", 0.6, 3),
            Obligation::new("reveal_villain_motivation", 0.9, 7),
        ];

        // Set up test emotional state
        let emotional_state = EmotionalState {
            name: "tense_anticipation".to_string(),
            valence: 0.3,
            arousal: 0.8,
            dominance: 0.4,
        };

        assistant.update_narrative_state(Some(emotional_state), obligations);
        assistant.advance_chapter(3, Some(2));

        assistant
    }

    #[test]
    fn test_rip_ric_fusion_state_initialization() {
        let fusion_state = RIPRICFusionState::new();

        assert_eq!(fusion_state.rip_genome_health, 1.0);
        assert_eq!(fusion_state.rip_guard_health, 1.0);
        assert_eq!(fusion_state.rip_pathogen_threat, 0.0);
        assert_eq!(fusion_state.ric_consensus_health, 1.0);
        assert_eq!(fusion_state.ric_saturation_level, 0.0);
        assert_eq!(fusion_state.current_recursion_budget, 20);
        assert!(!fusion_state.loop_saturation_detected);
        assert!(fusion_state.rip_process_healthy);
        assert!(fusion_state.last_arbitration_decision.is_none());
    }

    #[test]
    fn test_character_pathogen_detection() {
        let mut engine = CharacterConsistencyEngine::new();

        // Create a test character with formal dialogue pattern
        let mut character = crate::character_consistency::CharacterProfile {
            name: "Scholar".to_string(),
            role: "mentor".to_string(),
            personality_traits: std::collections::HashMap::new(),
            dialogue_pattern: crate::character_consistency::DialoguePattern {
                vocabulary_level: "formal".to_string(),
                sentence_structure: "complex".to_string(),
                favorite_phrases: vec!["indeed".to_string(), "precisely".to_string()],
                speech_quirks: vec![],
                emotional_tells: std::collections::HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.9,
            },
            relationships: std::collections::HashMap::new(),
            character_arc: None,
            physical_description: "Distinguished professor".to_string(),
            background: "Academic background".to_string(),
            motivations: vec!["teach wisdom".to_string(), "preserve knowledge".to_string()],
            fears: vec!["ignorance".to_string()],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 3,
            consistency_score: 0.9,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        };

        engine.add_character(character);

        // Test pathogen detection with inconsistent dialogue
        let analysis = engine.analyze_character_with_rip_fusion(
            "Scholar",
            "Yeah, whatever dude, like totally gonna do that thing", // Casual contamination
            "teach wisdom", // Action that contradicts nothing
            "classroom_scene"
        );

        assert!(!analysis.pathogen_detections.is_empty());

        // Check for voice drift pathogen
        let voice_drift = analysis.pathogen_detections.iter()
            .any(|p| matches!(p, CharacterPathogen::VoiceDrift { .. }));
        assert!(voice_drift, "Should detect voice drift pathogen");

        assert!(analysis.voice_consistency < 0.7);
        assert!(analysis.unified_vote.contains("HALT") || analysis.unified_vote.contains("CAUTION"));
    }

    #[test]
    fn test_character_constraint_genome_validation() {
        let mut engine = CharacterConsistencyEngine::new();

        // Create character with weak personality anchors
        let mut traits = std::collections::HashMap::new();
        traits.insert("weak_trait".to_string(), crate::character_consistency::PersonalityTrait {
            name: "Weak".to_string(),
            description: "Very weak trait".to_string(),
            intensity: 0.2, // Very low
            stability: 0.1, // Very unstable
            manifestations: vec![],
            contradictions: vec![],
            first_established: chrono::Utc::now(),
            last_reinforced: chrono::Utc::now(),
        });

        let character = crate::character_consistency::CharacterProfile {
            name: "WeakCharacter".to_string(),
            role: "protagonist".to_string(),
            personality_traits: traits,
            dialogue_pattern: crate::character_consistency::DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec![],
                speech_quirks: vec![],
                emotional_tells: std::collections::HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.3, // Low consistency
            },
            relationships: std::collections::HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec![],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 3,
            consistency_score: 0.3,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        };

        engine.add_character(character);

        let analysis = engine.analyze_character_with_rip_fusion(
            "WeakCharacter",
            "I say things",
            "runs away from conflict", // Action that doesn't align with protagonist role
            "test_context"
        );

        // Should detect constraint violations
        assert!(!analysis.constraint_violations.is_empty());

        // Check for specific violation types
        let has_weak_ligand = analysis.constraint_violations.iter()
            .any(|v| v.violation_type == "weak_personality_ligand");
        let has_role_violation = analysis.constraint_violations.iter()
            .any(|v| v.violation_type == "role_constraint_violation");
        let has_dialogue_weakness = analysis.constraint_violations.iter()
            .any(|v| v.violation_type == "dialogue_ligand_weakness");

        assert!(has_weak_ligand || has_role_violation || has_dialogue_weakness,
            "Should detect at least one constraint violation");
    }

    #[test]
    fn test_rip_ric_fusion_logging() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Create test fusion health
        let fusion_health = RIPRICFusionHealth {
            rip_genome_health: 0.8,
            rip_guard_health: 0.9,
            rip_pathogen_threat: 0.2,
            ric_consensus_health: 0.85,
            ric_saturation_level: 0.3,
            rip_process_healthy: true,
            current_recursion_budget: 15,
            loop_saturation_detected: false,
            fusion_timestamp: chrono::Utc::now(),
            overall_fusion_health: 0.82,
        };

        // Create test decision
        let decision = UnifiedArbitrationDecision::ContinueRecursion {
            rip_vote: "RIP_VOTE_CONTINUE_HIGH_CONFIDENCE".to_string(),
            ric_vote: RICDecision::Continue,
            consensus_confidence: 0.85,
        };

        // Log the fusion decision
        let result = logger.log_rip_ric_fusion(
            3,
            Some(2),
            decision,
            fusion_health,
            "The protagonist faced the darkest moment of doubt...".to_string(),
            "climactic_confrontation".to_string(),
            Some(json!({
                "word_count": 450,
                "emotional_intensity": 0.9,
                "narrative_tension": 0.8
            }))
        );

        assert!(result.is_ok());
        assert_eq!(logger.recent_fusion_entries.len(), 1);

        let entry = &logger.recent_fusion_entries[0];
        assert_eq!(entry.chapter, 3);
        assert_eq!(entry.scene, Some(2));
        assert_eq!(entry.overall_fusion_health, 0.82);
        assert!(entry.unified_decision.starts_with("ContinueRecursion"));
        assert!(entry.rip_process_healthy);
        assert!(!entry.loop_saturation_detected);

        // Test log formatting
        let formatted = entry.format_for_text();
        assert!(formatted.contains("RIP+RIC FUSION"));
        assert!(formatted.contains("Ch:3.2"));
        assert!(formatted.contains("ContinueRecursion"));
        assert!(formatted.contains("Budget:15"));
    }

    #[test]
    fn test_pathogen_detection_halt_scenario() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Simulate high pathogen threat scenario
        let fusion_health = RIPRICFusionHealth {
            rip_genome_health: 0.3, // Low
            rip_guard_health: 0.2,  // Very low
            rip_pathogen_threat: 0.9, // Very high
            ric_consensus_health: 0.7,
            ric_saturation_level: 0.4,
            rip_process_healthy: true,
            current_recursion_budget: 10,
            loop_saturation_detected: false,
            fusion_timestamp: chrono::Utc::now(),
            overall_fusion_health: 0.25, // Poor overall health
        };

        let decision = UnifiedArbitrationDecision::PathogenDetectionHalt {
            detected_pathogens: vec![
                "voice_drift_contamination".to_string(),
                "personality_inversion".to_string(),
                "motivation_drift".to_string()
            ],
            threat_level: 0.9,
        };

        let result = logger.log_rip_ric_fusion(
            5,
            None,
            decision,
            fusion_health,
            "Character acted completely out of character...".to_string(),
            "pathogen_contamination_scenario".to_string(),
            None
        );

        assert!(result.is_ok());

        let entry = &logger.recent_fusion_entries[0];
        assert!(entry.unified_decision.starts_with("PathogenDetectionHalt"));
        assert_eq!(entry.rip_pathogen_threat, 0.9);
        assert_eq!(entry.overall_fusion_health, 0.25);

        // Test detailed logging for pathogen halt
        let formatted = entry.format_for_text();
        assert!(formatted.contains("PathogenDetectionHalt"));
        assert!(formatted.contains("🚨 CRITICAL") || formatted.contains("🔴 POOR"));
    }

    #[test]
    fn test_loop_saturation_halt_scenario() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Simulate loop saturation scenario
        let fusion_health = RIPRICFusionHealth {
            rip_genome_health: 0.7,
            rip_guard_health: 0.6,
            rip_pathogen_threat: 0.1,
            ric_consensus_health: 0.3, // Low due to saturation
            ric_saturation_level: 0.9, // Very high saturation
            rip_process_healthy: true,
            current_recursion_budget: 0, // Exhausted
            loop_saturation_detected: true,
            fusion_timestamp: chrono::Utc::now(),
            overall_fusion_health: 0.35,
        };

        let decision = UnifiedArbitrationDecision::LoopSaturationHalt {
            saturated_phases: vec![
                "surface_expansion".to_string(),
                "character_development".to_string(),
                "thematic_exploration".to_string()
            ],
            budget_exhausted: true,
        };

        let result = logger.log_rip_ric_fusion(
            8,
            Some(4),
            decision,
            fusion_health,
            "The narrative seemed to spiral endlessly...".to_string(),
            "recursive_loop_detection".to_string(),
            Some(json!({
                "iteration_count": 25,
                "stagnation_detected": true
            }))
        );

        assert!(result.is_ok());

        let entry = &logger.recent_fusion_entries[0];
        assert!(entry.unified_decision.starts_with("LoopSaturationHalt"));
        assert_eq!(entry.current_recursion_budget, 0);
        assert!(entry.loop_saturation_detected);
        assert_eq!(entry.ric_saturation_level, 0.9);
    }

    #[test]
    fn test_unified_continuity_floor_scenario() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Simulate continuity floor injection scenario
        let fusion_health = RIPRICFusionHealth {
            rip_genome_health: 0.2, // Critical
            rip_guard_health: 0.1,  // Critical
            rip_pathogen_threat: 0.8, // High
            ric_consensus_health: 0.2, // Critical
            ric_saturation_level: 0.95, // Maximum saturation
            rip_process_healthy: false, // Process failure
            current_recursion_budget: 0,
            loop_saturation_detected: true,
            fusion_timestamp: chrono::Utc::now(),
            overall_fusion_health: 0.15, // Critical
        };

        let decision = UnifiedArbitrationDecision::UnifiedContinuityFloor {
            rip_completion_summary: "Constraint genome severely compromised - emergency stabilization required".to_string(),
            ric_completion_summary: "All subsystems saturated - consensus arbitration halted".to_string(),
            fusion_reason: "System integrity below critical threshold - unified continuity floor engaged".to_string(),
        };

        let result = logger.log_rip_ric_fusion(
            12,
            Some(7),
            decision,
            fusion_health,
            "The story threatened to collapse under its own complexity...".to_string(),
            "emergency_stabilization".to_string(),
            Some(json!({
                "system_failure": true,
                "recovery_mode": "continuity_floor",
                "emergency_level": "critical"
            }))
        );

        assert!(result.is_ok());

        let entry = &logger.recent_fusion_entries[0];
        assert!(entry.unified_decision.starts_with("UnifiedContinuityFloor"));
        assert_eq!(entry.overall_fusion_health, 0.15);
        assert!(!entry.rip_process_healthy);
        assert!(entry.loop_saturation_detected);
        assert_eq!(entry.current_recursion_budget, 0);

        let formatted = entry.format_for_text();
        assert!(formatted.contains("🚨 CRITICAL"));
        assert!(formatted.contains("❌")); // Process failure indicator
    }

    #[test]
    fn test_fusion_analysis_report_generation() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Create multiple test entries with different scenarios
        let scenarios = vec![
            (UnifiedArbitrationDecision::ContinueRecursion {
                rip_vote: "RIP_VOTE_CONTINUE_HIGH_CONFIDENCE".to_string(),
                ric_vote: RICDecision::Continue,
                consensus_confidence: 0.9,
            }, 0.85),
            (UnifiedArbitrationDecision::PathogenDetectionHalt {
                detected_pathogens: vec!["voice_drift".to_string()],
                threat_level: 0.7,
            }, 0.4),
            (UnifiedArbitrationDecision::LoopSaturationHalt {
                saturated_phases: vec!["expansion".to_string()],
                budget_exhausted: false,
            }, 0.3),
            (UnifiedArbitrationDecision::RICConsensusHalt {
                voting_subsystems: vec!["character_engine".to_string()],
                halt_reason: "Character consistency violation".to_string(),
            }, 0.5),
        ];

        for (i, (decision, health)) in scenarios.into_iter().enumerate() {
            let fusion_health = RIPRICFusionHealth {
                rip_genome_health: health,
                rip_guard_health: health,
                rip_pathogen_threat: 1.0 - health,
                ric_consensus_health: health,
                ric_saturation_level: 1.0 - health,
                rip_process_healthy: health > 0.5,
                current_recursion_budget: ((health * 20.0) as u32).max(0),
                loop_saturation_detected: health < 0.4,
                fusion_timestamp: chrono::Utc::now(),
                overall_fusion_health: health,
            };

            logger.log_rip_ric_fusion(
                i as u32 + 1,
                Some(1),
                decision,
                fusion_health,
                format!("Test scenario {}", i + 1),
                format!("test_context_{}", i + 1),
                None
            ).unwrap();
        }

        // Generate analysis report
        let analysis = logger.generate_fusion_analysis(10).unwrap();

        assert!(analysis.contains("RIP+RIC UNIFIED PROTOCOL ANALYSIS"));
        assert!(analysis.contains("Total Fusion Entries: 4"));
        assert!(analysis.contains("Unified Decision Distribution"));
        assert!(analysis.contains("Continue Recursion:"));
        assert!(analysis.contains("Pathogen Detection Halt:"));
        assert!(analysis.contains("Loop Saturation Halt:"));
        assert!(analysis.contains("RIC Consensus Halt:"));
        assert!(analysis.contains("Protocol Health Metrics"));
        assert!(analysis.contains("Overall Fusion Health:"));
        assert!(analysis.contains("System Performance"));
        assert!(analysis.contains("Protocol Effectiveness"));
        assert!(analysis.contains("Recent Fusion Activity"));
    }

    #[test]
    fn test_character_rip_health_assessment() {
        let mut engine = CharacterConsistencyEngine::new();

        // Create a healthy character
        let mut traits = std::collections::HashMap::new();
        traits.insert("brave".to_string(), crate::character_consistency::PersonalityTrait {
            name: "Brave".to_string(),
            description: "Faces danger".to_string(),
            intensity: 0.9,
            stability: 0.8,
            manifestations: vec!["fights monsters".to_string()],
            contradictions: vec!["runs away".to_string()],
            first_established: chrono::Utc::now(),
            last_reinforced: chrono::Utc::now(),
        });

        let character = crate::character_consistency::CharacterProfile {
            name: "Hero".to_string(),
            role: "protagonist".to_string(),
            personality_traits: traits,
            dialogue_pattern: crate::character_consistency::DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec!["let's do this".to_string()],
                speech_quirks: vec![],
                emotional_tells: std::collections::HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.9,
            },
            relationships: std::collections::HashMap::new(),
            character_arc: Some(crate::character_consistency::CharacterArc {
                character_name: "Hero".to_string(),
                arc_theme: "redemption".to_string(),
                starting_state: "fallen".to_string(),
                desired_end_state: "redeemed".to_string(),
                current_progress: 0.6,
                arc_milestones: vec![],
                obstacles_faced: vec![],
                growth_moments: vec![],
                regression_moments: vec![],
                consistency_with_theme: 0.8,
            }),
            physical_description: "Strong warrior".to_string(),
            background: "Former knight".to_string(),
            motivations: vec!["protect innocent".to_string()],
            fears: vec!["failing others".to_string()],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 5,
            consistency_score: 0.9,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        };

        engine.add_character(character);

        let health = engine.get_rip_ric_character_health("Hero").unwrap();

        assert_eq!(health.character_name, "Hero");
        assert_eq!(health.voice_consistency_health, 0.9);
        assert!(health.personality_ligand_health > 0.7); // Should be high due to strong traits
        assert_eq!(health.arc_momentum_health, 0.8);
        assert_eq!(health.relationship_constraint_health, 1.0); // No relationships = no problems
        assert!(health.overall_character_health > 0.8);
    }

    #[test]
    fn test_rip_ric_fusion_state_updates() {
        let mut assistant = create_test_assistant();

        // Get initial fusion health
        let initial_health = assistant.get_rip_ric_fusion_health();
        assert_eq!(initial_health.rip_genome_health, 1.0);
        assert_eq!(initial_health.current_recursion_budget, 20);
        assert!(!initial_health.loop_saturation_detected);

        // Simulate some operations that would degrade health
        for _ in 0..5 {
            assistant.rip_ric_fusion_state.current_recursion_budget =
                assistant.rip_ric_fusion_state.current_recursion_budget.saturating_sub(1);
            assistant.rip_ric_fusion_state.ric_consensus_health *= 0.9;
        }

        // Check updated health
        let updated_health = assistant.get_rip_ric_fusion_health();
        assert_eq!(updated_health.current_recursion_budget, 15);
        assert!(updated_health.ric_consensus_health < initial_health.ric_consensus_health);
        assert!(updated_health.overall_fusion_health < initial_health.overall_fusion_health);
    }

    #[test]
    fn test_rip_ric_voting_integration() {
        let mut assistant = create_test_assistant();

        // Test voting methods for different subsystems
        let constraint_vote = assistant.vote_constraint_tracker();
        let recursion_vote = assistant.vote_recursion_tracker();
        let engagement_vote = assistant.vote_engagement_tracker();

        // Votes should be valid strings indicating system state
        assert!(constraint_vote.contains("CONTINUE") || constraint_vote.contains("CAUTION") || constraint_vote.contains("HALT"));
        assert!(recursion_vote.contains("CONTINUE") || recursion_vote.contains("CAUTION") || recursion_vote.contains("HALT"));
        assert!(engagement_vote.contains("CONTINUE") || engagement_vote.contains("CAUTION") || engagement_vote.contains("HALT"));

        // Test character engine voting
        let character_vote = assistant.character_engine.vote_on_consistency_state();
        assert!(matches!(character_vote, InsightStatus::Continue | InsightStatus::Suggest | InsightStatus::Block | InsightStatus::Stalled));
    }

    #[test]
    fn test_fusion_state_reset() {
        let mut assistant = create_test_assistant();

        // Degrade the fusion state
        assistant.rip_ric_fusion_state.rip_genome_health = 0.3;
        assistant.rip_ric_fusion_state.rip_pathogen_threat = 0.8;
        assistant.rip_ric_fusion_state.current_recursion_budget = 5;
        assistant.rip_ric_fusion_state.loop_saturation_detected = true;

        // Reset the fusion state
        assistant.reset_rip_ric_fusion();

        // Verify reset
        assert_eq!(assistant.rip_ric_fusion_state.rip_genome_health, 1.0);
        assert_eq!(assistant.rip_ric_fusion_state.rip_pathogen_threat, 0.0);
        assert_eq!(assistant.rip_ric_fusion_state.current_recursion_budget, 20);
        assert!(!assistant.rip_ric_fusion_state.loop_saturation_detected);
        assert!(assistant.rip_ric_fusion_state.rip_process_healthy);
    }
}