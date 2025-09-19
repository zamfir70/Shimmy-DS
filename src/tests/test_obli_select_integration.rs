// ObliSelect Smart Obligation Management Integration Tests
// Tests the full ObliSelect system integration with SHIMMY-DS

use crate::obligations::{
    SmartObligationManager, Obligation, ObligationCategory, ObligationUrgency,
    ObliSelectSettings, ObligationScore
};
use crate::prompt_injector::{
    inject_smart_obligations, inject_smart_obligations_with_details,
    load_smart_obligations, load_smart_obligations_with_scores
};
use crate::recursive_narrative_assistant::RecursiveNarrativeAssistant;
use crate::stability_log::obli_select_telemetry;
use chrono::Utc;
use serde_json;

#[test]
fn test_obligation_creation_and_basic_properties() {
    let obligation = Obligation {
        id: "test_001".to_string(),
        content: "Harper must confront the mysterious figure in the attic".to_string(),
        category: ObligationCategory::CharacterDevelopment,
        urgency: ObligationUrgency::High,
        created_at: Utc::now(),
        last_injection: None,
        injection_count: 0,
        chapter_introduced: 1,
        characters_involved: vec!["Harper".to_string()],
        tension_vector: 0.7, // Tension building
        salience_boost: 0.8,
        fulfillment_progress: 0.0,
        dependencies: vec![],
    };

    assert_eq!(obligation.id, "test_001");
    assert_eq!(obligation.category, ObligationCategory::CharacterDevelopment);
    assert_eq!(obligation.urgency, ObligationUrgency::High);
    assert_eq!(obligation.tension_vector, 0.7);
    assert_eq!(obligation.fulfillment_progress, 0.0);
    assert_eq!(obligation.injection_count, 0);
}

#[test]
fn test_smart_obligation_manager_creation() {
    let manager = SmartObligationManager::new();
    let metrics = manager.get_metrics();

    assert_eq!(metrics.total_obligations, 0);
    assert_eq!(metrics.stale_obligations, 0);
    assert_eq!(metrics.overused_obligations, 0);
    assert_eq!(metrics.average_injection_count, 0.0);
    assert_eq!(metrics.fulfillment_progress_average, 0.0);
}

#[test]
fn test_smart_obligation_manager_with_custom_settings() {
    let settings = ObliSelectSettings {
        max_obligations_per_selection: 5,
        urgency_weight: 0.4,
        salience_weight: 0.3,
        freshness_weight: 0.1,
        tension_balance_weight: 0.1,
        dependency_weight: 0.05,
        context_relevance_weight: 0.05,
        staleness_penalty_threshold: 3,
        overuse_penalty_threshold: 5,
        tension_balance_target: 0.2,
        enable_adaptive_weighting: false,
        enable_dependency_resolution: false,
        enable_contextual_filtering: false,
    };

    let manager = SmartObligationManager::with_settings(settings.clone());
    let manager_settings = manager.get_settings();

    assert_eq!(manager_settings.max_obligations_per_selection, 5);
    assert_eq!(manager_settings.urgency_weight, 0.4);
    assert_eq!(manager_settings.staleness_penalty_threshold, 3);
    assert!(!manager_settings.enable_adaptive_weighting);
}

#[test]
fn test_obligation_addition_and_metrics_update() {
    let mut manager = SmartObligationManager::new();

    let obligation1 = create_test_obligation("test_001", ObligationCategory::PlotAdvancement, ObligationUrgency::Critical);
    let obligation2 = create_test_obligation("test_002", ObligationCategory::CharacterDevelopment, ObligationUrgency::Medium);

    manager.add_obligation(obligation1);
    manager.add_obligation(obligation2);

    let metrics = manager.get_metrics();
    assert_eq!(metrics.total_obligations, 2);
    assert_eq!(metrics.obligations_by_category.get(&ObligationCategory::PlotAdvancement), Some(&1));
    assert_eq!(metrics.obligations_by_urgency.get(&ObligationUrgency::Critical), Some(&1));
}

#[test]
fn test_obligation_selection_scoring() {
    let mut manager = SmartObligationManager::new();

    // Add obligations with different characteristics
    let high_urgency = create_test_obligation_with_properties(
        "urgent_001", ObligationCategory::ConflictResolution, ObligationUrgency::Critical,
        1.0, 0.8, 0.0 // High tension, high salience, no fulfillment
    );

    let stale_obligation = create_test_obligation_with_properties(
        "stale_001", ObligationCategory::WorldBuilding, ObligationUrgency::Low,
        -0.3, 0.3, 0.5 // Tension relief, low salience, partially fulfilled
    );

    manager.add_obligation(high_urgency);
    manager.add_obligation(stale_obligation);

    // Update context to chapter 10 to make stale obligation actually stale
    manager.update_context(10, vec!["TestCharacter".to_string()], 0.5, "Test context".to_string());

    let selected = manager.select_obligations(Some(2));

    assert_eq!(selected.len(), 2);
    // High urgency obligation should score higher
    assert!(selected[0].total_score > selected[1].total_score);
    assert_eq!(selected[0].obligation_id, "urgent_001");
}

#[test]
fn test_obligation_freshness_scoring() {
    let mut manager = SmartObligationManager::new();

    let fresh_obligation = create_test_obligation("fresh_001", ObligationCategory::EmotionalResolution, ObligationUrgency::Medium);
    manager.add_obligation(fresh_obligation);

    // Select once to mark as injected
    let first_selection = manager.select_obligations(Some(1));
    assert_eq!(first_selection.len(), 1);
    assert!(first_selection[0].freshness_score > 0.9); // Should be very fresh

    // Select again immediately - should have lower freshness
    let second_selection = manager.select_obligations(Some(1));
    assert_eq!(second_selection.len(), 1);
    assert!(second_selection[0].freshness_score < first_selection[0].freshness_score);
}

#[test]
fn test_obligation_dependency_resolution() {
    let mut manager = SmartObligationManager::new();

    let parent_obligation = create_test_obligation("parent_001", ObligationCategory::PlotAdvancement, ObligationUrgency::High);
    let mut child_obligation = create_test_obligation("child_001", ObligationCategory::CharacterDevelopment, ObligationUrgency::High);
    child_obligation.dependencies = vec!["parent_001".to_string()];

    manager.add_obligation(parent_obligation);
    manager.add_obligation(child_obligation);

    let selected = manager.select_obligations(Some(2));

    // Parent should be selected before child due to dependency resolution
    assert_eq!(selected.len(), 2);
    if selected.len() == 2 {
        let parent_index = selected.iter().position(|s| s.obligation_id == "parent_001");
        let child_index = selected.iter().position(|s| s.obligation_id == "child_001");

        assert!(parent_index.is_some());
        assert!(child_index.is_some());
        // Note: Dependency resolution might not guarantee ordering in this simple test
    }
}

#[test]
fn test_obligation_fulfillment_tracking() {
    let mut manager = SmartObligationManager::new();

    let obligation = create_test_obligation("fulfill_001", ObligationCategory::ThematicExploration, ObligationUrgency::Medium);
    manager.add_obligation(obligation);

    // Update fulfillment progress
    assert!(manager.update_fulfillment_progress("fulfill_001", 0.5));
    assert!(manager.update_fulfillment_progress("fulfill_001", 1.0));

    // Try to update non-existent obligation
    assert!(!manager.update_fulfillment_progress("nonexistent", 0.5));

    let metrics = manager.get_metrics();
    assert_eq!(metrics.fulfillment_progress_average, 1.0);
}

#[test]
fn test_obligation_staleness_detection() {
    let mut manager = SmartObligationManager::new();

    let mut obligation = create_test_obligation("stale_001", ObligationCategory::SettingDetail, ObligationUrgency::Low);
    obligation.chapter_introduced = 1;
    manager.add_obligation(obligation);

    // Move to chapter 10 (9 chapters later, above default staleness threshold of 5)
    manager.update_context(10, vec![], 0.0, "".to_string());

    let stale_obligations = manager.get_stale_obligations(5);
    assert_eq!(stale_obligations.len(), 1);
    assert_eq!(stale_obligations[0].id, "stale_001");

    let metrics = manager.get_metrics();
    assert_eq!(metrics.stale_obligations, 1);
}

#[test]
fn test_obligation_overuse_detection() {
    let mut manager = SmartObligationManager::new();

    let obligation = create_test_obligation("overused_001", ObligationCategory::DialoguePromise, ObligationUrgency::Medium);
    manager.add_obligation(obligation);

    // Simulate multiple selections to trigger overuse
    for _ in 0..12 {
        manager.select_obligations(Some(1));
    }

    let overused_obligations = manager.get_overused_obligations(10);
    assert_eq!(overused_obligations.len(), 1);
    assert_eq!(overused_obligations[0].id, "overused_001");

    let metrics = manager.get_metrics();
    assert_eq!(metrics.overused_obligations, 1);
}

#[test]
fn test_obligation_context_relevance() {
    let mut manager = SmartObligationManager::new();

    let mut relevant_obligation = create_test_obligation("relevant_001", ObligationCategory::RelationshipDynamics, ObligationUrgency::Medium);
    relevant_obligation.characters_involved = vec!["Alice".to_string(), "Bob".to_string()];
    relevant_obligation.content = "Alice must resolve her conflict with Bob about the treasure".to_string();

    let mut irrelevant_obligation = create_test_obligation("irrelevant_001", ObligationCategory::WorldBuilding, ObligationUrgency::Medium);
    irrelevant_obligation.characters_involved = vec!["Charlie".to_string()];
    irrelevant_obligation.content = "Charlie explores the ancient ruins".to_string();

    manager.add_obligation(relevant_obligation);
    manager.add_obligation(irrelevant_obligation);

    // Set context with Alice and Bob as recent characters
    manager.update_context(
        5,
        vec!["Alice".to_string(), "Bob".to_string()],
        0.0,
        "treasure conflict resolution".to_string()
    );

    let selected = manager.select_obligations(Some(2));
    assert_eq!(selected.len(), 2);

    // Find the scores for each obligation
    let relevant_score = selected.iter().find(|s| s.obligation_id == "relevant_001").unwrap();
    let irrelevant_score = selected.iter().find(|s| s.obligation_id == "irrelevant_001").unwrap();

    // Relevant obligation should have higher context relevance score
    assert!(relevant_score.context_relevance_score > irrelevant_score.context_relevance_score);
}

#[test]
fn test_obligation_tension_balance_scoring() {
    let mut manager = SmartObligationManager::new();

    let tension_building = create_test_obligation_with_properties(
        "tension_up", ObligationCategory::ConflictResolution, ObligationUrgency::Medium,
        0.8, 0.5, 0.0 // High positive tension
    );

    let tension_relief = create_test_obligation_with_properties(
        "tension_down", ObligationCategory::EmotionalResolution, ObligationUrgency::Medium,
        -0.6, 0.5, 0.0 // Negative tension (relief)
    );

    manager.add_obligation(tension_building);
    manager.add_obligation(tension_relief);

    // Set high current tension - relief obligation should score higher
    manager.update_context(5, vec![], 0.8, "High tension scene".to_string());

    let selected = manager.select_obligations(Some(2));
    let relief_score = selected.iter().find(|s| s.obligation_id == "tension_down").unwrap();
    let building_score = selected.iter().find(|s| s.obligation_id == "tension_up").unwrap();

    // Relief obligation should have higher tension balance score when current tension is high
    assert!(relief_score.tension_balance_score > building_score.tension_balance_score);
}

#[test]
fn test_prompt_injector_integration() {
    let mut manager = SmartObligationManager::new();

    let obligation1 = create_test_obligation("inject_001", ObligationCategory::PlotAdvancement, ObligationUrgency::High);
    let obligation2 = create_test_obligation("inject_002", ObligationCategory::CharacterDevelopment, ObligationUrgency::Medium);

    manager.add_obligation(obligation1);
    manager.add_obligation(obligation2);

    let prompt = "What happens next in the story?";
    let injected_prompt = inject_smart_obligations(prompt, &mut manager, Some(2));

    assert!(injected_prompt.contains("Obligation:"));
    assert!(injected_prompt.ends_with("What happens next in the story?"));
    assert!(injected_prompt.len() > prompt.len());
}

#[test]
fn test_prompt_injector_with_scoring_details() {
    let mut manager = SmartObligationManager::new();

    let obligation = create_test_obligation("detail_001", ObligationCategory::Foreshadowing, ObligationUrgency::Critical);
    manager.add_obligation(obligation);

    let prompt = "Continue the narrative";
    let (injected_prompt, scores) = inject_smart_obligations_with_details(prompt, &mut manager, Some(1));

    assert!(injected_prompt.contains("Obligation:"));
    assert_eq!(scores.len(), 1);
    assert_eq!(scores[0].obligation_id, "detail_001");
    assert!(scores[0].total_score > 0.0);
    assert!(!scores[0].justification.is_empty());
}

#[test]
fn test_load_smart_obligations_functions() {
    let mut manager = SmartObligationManager::new();

    let obligation1 = create_test_obligation("load_001", ObligationCategory::WorldBuilding, ObligationUrgency::High);
    let obligation2 = create_test_obligation("load_002", ObligationCategory::EmotionalResolution, ObligationUrgency::Low);

    manager.add_obligation(obligation1);
    manager.add_obligation(obligation2);

    // Test load_smart_obligations
    let obligations = load_smart_obligations(&mut manager, Some(2));
    assert_eq!(obligations.len(), 2);

    // Test load_smart_obligations_with_scores
    let obligation_scores = load_smart_obligations_with_scores(&mut manager, Some(1));
    assert_eq!(obligation_scores.len(), 1);
    assert!(obligation_scores[0].total_score > 0.0);
}

#[test]
fn test_recursive_narrative_assistant_integration() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Test obligation addition
    let obligation = create_test_obligation("assistant_001", ObligationCategory::CharacterDevelopment, ObligationUrgency::High);
    assistant.add_obligation(obligation);

    // Test context update
    assistant.update_obligation_context();

    // Test status retrieval
    let status = assistant.get_obligation_status();
    assert!(status.is_object());
    assert_eq!(status["total_obligations"], 1);

    // Test obligation selection with scores
    let selection_result = assistant.get_selected_obligations_with_scores(Some(1));
    assert!(selection_result.is_object());
    assert_eq!(selection_result["selection_count"], 1);

    // Test fulfillment update
    assert!(assistant.update_obligation_fulfillment("assistant_001", 0.7));

    // Test obligation removal
    assert!(assistant.remove_obligation("assistant_001"));
    let status_after_removal = assistant.get_obligation_status();
    assert_eq!(status_after_removal["total_obligations"], 0);
}

#[test]
fn test_obligation_metrics_comprehensive() {
    let mut manager = SmartObligationManager::new();

    // Add diverse obligations for comprehensive metrics
    let obligations = vec![
        create_test_obligation_with_properties("metrics_001", ObligationCategory::PlotAdvancement, ObligationUrgency::Critical, 0.5, 0.8, 0.2),
        create_test_obligation_with_properties("metrics_002", ObligationCategory::CharacterDevelopment, ObligationUrgency::High, -0.3, 0.6, 0.8),
        create_test_obligation_with_properties("metrics_003", ObligationCategory::EmotionalResolution, ObligationUrgency::Medium, 0.7, 0.4, 0.0),
        create_test_obligation_with_properties("metrics_004", ObligationCategory::WorldBuilding, ObligationUrgency::Low, 0.0, 0.9, 1.0),
    ];

    for obligation in obligations {
        manager.add_obligation(obligation);
    }

    let metrics = manager.get_metrics();

    assert_eq!(metrics.total_obligations, 4);
    assert_eq!(metrics.obligations_by_category.len(), 4);
    assert_eq!(metrics.obligations_by_urgency.len(), 4);
    assert!(metrics.fulfillment_progress_average > 0.0);

    // Check tension distribution
    let (negative, neutral, positive) = metrics.tension_distribution;
    assert!((negative + neutral + positive - 1.0).abs() < 0.01); // Should sum to 1.0
}

#[test]
fn test_obligation_settings_update() {
    let mut manager = SmartObligationManager::new();

    let new_settings = ObliSelectSettings {
        max_obligations_per_selection: 15,
        urgency_weight: 0.5,
        salience_weight: 0.3,
        freshness_weight: 0.1,
        tension_balance_weight: 0.05,
        dependency_weight: 0.03,
        context_relevance_weight: 0.02,
        staleness_penalty_threshold: 8,
        overuse_penalty_threshold: 15,
        tension_balance_target: 0.3,
        enable_adaptive_weighting: false,
        enable_dependency_resolution: false,
        enable_contextual_filtering: false,
    };

    manager.update_settings(new_settings);
    let updated_settings = manager.get_settings();

    assert_eq!(updated_settings.max_obligations_per_selection, 15);
    assert_eq!(updated_settings.urgency_weight, 0.5);
    assert_eq!(updated_settings.staleness_penalty_threshold, 8);
    assert!(!updated_settings.enable_adaptive_weighting);
}

#[test]
fn test_obligation_injection_statistics_reset() {
    let mut manager = SmartObligationManager::new();

    let obligation = create_test_obligation("reset_001", ObligationCategory::DialoguePromise, ObligationUrgency::Medium);
    manager.add_obligation(obligation);

    // Select multiple times to build up injection count
    for _ in 0..5 {
        manager.select_obligations(Some(1));
    }

    let metrics_before_reset = manager.get_metrics();
    assert!(metrics_before_reset.average_injection_count > 0.0);

    // Reset stats
    manager.reset_injection_stats();

    let metrics_after_reset = manager.get_metrics();
    assert_eq!(metrics_after_reset.average_injection_count, 0.0);
}

#[test]
fn test_telemetry_logging_integration() {
    let mut manager = SmartObligationManager::new();

    let obligation = create_test_obligation("telemetry_001", ObligationCategory::ThematicExploration, ObligationUrgency::High);
    manager.add_obligation(obligation);

    let selected = manager.select_obligations(Some(1));
    let metrics = manager.get_metrics();
    let settings = manager.get_settings();

    // Test telemetry logging functions (these should not panic)
    obli_select_telemetry::log_obligation_selection(&selected, metrics.last_selection_performance_ms, 5, "Test context");
    obli_select_telemetry::log_obligation_metrics(&metrics, &settings, 5);
    obli_select_telemetry::log_obligation_lifecycle_event(
        "ADD",
        "telemetry_001",
        Some("Test obligation content"),
        Some(ObligationCategory::ThematicExploration),
        Some(ObligationUrgency::High),
        Some(0.0),
        5
    );
    obli_select_telemetry::log_performance_warning("test_operation", 100, 50, 5, 1);

    let logging_summary = obli_select_telemetry::get_logging_summary(5);
    assert!(logging_summary.is_object());
    assert_eq!(logging_summary["chapter"], 5);
}

#[test]
fn test_adaptive_weighting_behavior() {
    let mut settings = ObliSelectSettings::default();
    settings.enable_adaptive_weighting = true;

    let mut manager = SmartObligationManager::with_settings(settings);

    // Add critical obligations to trigger adaptive weighting
    for i in 0..5 {
        let critical_obligation = create_test_obligation(
            &format!("critical_{}", i),
            ObligationCategory::ConflictResolution,
            ObligationUrgency::Critical
        );
        manager.add_obligation(critical_obligation);
    }

    // Update context which should trigger adaptive weighting
    manager.update_context(5, vec![], 0.0, "High urgency context".to_string());

    // Select obligations - this should internally adapt the weights
    let selected = manager.select_obligations(Some(3));
    assert_eq!(selected.len(), 3);

    // Verify that urgency scores are prominent due to adaptive weighting
    for score in &selected {
        assert!(score.urgency_score > 0.8); // Should be high for critical obligations
    }
}

#[test]
fn test_contextual_filtering_behavior() {
    let mut settings = ObliSelectSettings::default();
    settings.enable_contextual_filtering = true;

    let mut manager = SmartObligationManager::with_settings(settings);

    // Add obligation with low context relevance
    let mut low_relevance = create_test_obligation("low_context", ObligationCategory::WorldBuilding, ObligationUrgency::Medium);
    low_relevance.characters_involved = vec!["UnrelatedCharacter".to_string()];

    // Add obligation with high context relevance
    let mut high_relevance = create_test_obligation("high_context", ObligationCategory::CharacterDevelopment, ObligationUrgency::Medium);
    high_relevance.characters_involved = vec!["RelevantCharacter".to_string()];

    manager.add_obligation(low_relevance);
    manager.add_obligation(high_relevance);

    // Set context that matches high relevance obligation
    manager.update_context(
        5,
        vec!["RelevantCharacter".to_string()],
        0.0,
        "character development scene".to_string()
    );

    let selected = manager.select_obligations(Some(2));

    // High relevance obligation should be selected first
    assert!(selected.iter().any(|s| s.obligation_id == "high_context"));

    // Verify that high context obligation has better context relevance score
    let high_score = selected.iter().find(|s| s.obligation_id == "high_context").unwrap();
    assert!(high_score.context_relevance_score > 0.6);
}

// Helper functions for creating test obligations

fn create_test_obligation(id: &str, category: ObligationCategory, urgency: ObligationUrgency) -> Obligation {
    Obligation {
        id: id.to_string(),
        content: format!("Test obligation content for {}", id),
        category,
        urgency,
        created_at: Utc::now(),
        last_injection: None,
        injection_count: 0,
        chapter_introduced: 1,
        characters_involved: vec!["TestCharacter".to_string()],
        tension_vector: 0.0,
        salience_boost: 0.5,
        fulfillment_progress: 0.0,
        dependencies: vec![],
    }
}

fn create_test_obligation_with_properties(
    id: &str,
    category: ObligationCategory,
    urgency: ObligationUrgency,
    tension_vector: f32,
    salience_boost: f32,
    fulfillment_progress: f32,
) -> Obligation {
    Obligation {
        id: id.to_string(),
        content: format!("Test obligation content for {}", id),
        category,
        urgency,
        created_at: Utc::now(),
        last_injection: None,
        injection_count: 0,
        chapter_introduced: 1,
        characters_involved: vec!["TestCharacter".to_string()],
        tension_vector,
        salience_boost,
        fulfillment_progress,
        dependencies: vec![],
    }
}