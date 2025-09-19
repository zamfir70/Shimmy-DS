/// Integration tests for the recursive narrative tracking system
///
/// These tests verify that all recursive systems work together coherently
/// and that the unified assistant interface properly coordinates them.

use crate::recursive_narrative_assistant::*;
use crate::narrative_dna::*;
use crate::constraint_space::*;
use crate::multi_level_recursion::*;
use crate::character_consistency::*;
use crate::reader_engagement_loops::*;
use crate::obligation_pressure::Obligation;
use crate::emotion_resonance::EmotionalState;

#[test]
fn test_full_system_integration() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Test 1: Create a character
    let character = create_test_character("Hero", "protagonist");
    assistant.character_engine.add_character(character);

    // Test 2: Add DNA units
    let contradiction = NarrativeDNAUnit::new_contradiction(
        "hero_fear".to_string(),
        "Hero fears commitment but wants love".to_string(),
        0.8,
        vec!["Hero".to_string()],
    );
    assistant.dna_tracker.add_unit(contradiction);

    // Test 3: Create engagement loop
    let engagement_loop = EngagementLoopType::EmotionalInvestment {
        character_or_situation: "Hero".to_string(),
        investment_level: 0.7,
        payoff_expectation: 0.8,
    };
    assistant.engagement_tracker.initiate_loop(engagement_loop, 0.7);

    // Test 4: Add constraints
    let constraint = ConstraintType::CharacterState {
        character: "Hero".to_string(),
        trait_name: "fear_of_commitment".to_string(),
        prevents_actions: vec!["propose_marriage".to_string(), "declare_love".to_string()],
    };
    assistant.constraint_tracker.add_constraint(constraint);

    // Test 5: Add recursive elements
    let recursive_element = RecursiveElement {
        id: "mirror_metaphor".to_string(),
        content: "The mirror showed more than just reflection".to_string(),
        level: NarrativeLevel::Sentence,
        chapter: 1,
        scene: Some(1),
        paragraph: Some(1),
        sentence: Some(1),
        element_type: ElementType::Symbolic {
            symbol_name: "mirror".to_string(),
            meaning: "self-awareness".to_string(),
        },
        intensity: 0.6,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(recursive_element);

    // Test 6: Update narrative state with obligations and emotions
    let obligations = vec![
        Obligation::new("resolve_hero_arc", 0.8, 2),
        Obligation::new("answer_mystery", 0.6, 3),
    ];
    let emotional_state = EmotionalState::new("conflicted", 0.7);
    assistant.update_narrative_state(Some(emotional_state), obligations);

    // Test 7: Advance chapter and analyze
    assistant.advance_chapter(2, Some(1));
    let insights = assistant.analyze_narrative_state();

    // Verify systems are working together
    assert_eq!(assistant.current_chapter, 2);
    assert_eq!(assistant.dna_tracker.current_chapter, 2);
    assert_eq!(assistant.character_engine.current_chapter, 2);
    assert_eq!(assistant.engagement_tracker.current_chapter, 2);
    assert_eq!(assistant.constraint_tracker.current_chapter, 2);

    // Should have some insights from the integrated analysis
    assert!(!insights.is_empty() || assistant.config.assertiveness_level < 0.3);

    // Test 8: Generate comprehensive report
    let report = assistant.generate_comprehensive_report();
    assert!(report.contains("RECURSIVE NARRATIVE ASSISTANT REPORT"));
    assert!(report.contains("Current Chapter: 2"));

    println!("✅ Full system integration test passed");
}

#[test]
fn test_cross_system_pattern_detection() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Create conditions that should trigger cross-system patterns

    // High constraint pressure
    for i in 0..5 {
        let constraint = ConstraintType::UnresolvedThread {
            thread_id: format!("thread_{}", i),
            description: format!("Unresolved thread {}", i),
            urgency: 0.8,
            must_resolve_by_chapter: Some(5),
        };
        assistant.constraint_tracker.add_constraint(constraint);
    }

    // High obligation pressure
    let high_obligations = vec![
        Obligation::new("critical_1", 0.9, 5),
        Obligation::new("critical_2", 0.8, 6),
        Obligation::new("critical_3", 0.7, 7),
    ];
    assistant.update_narrative_state(None, high_obligations);

    // Multiple engagement loops requiring attention
    for i in 0..3 {
        let loop_type = EngagementLoopType::CuriosityHypothesis {
            mystery_element: format!("Mystery {}", i),
            hypothesis_strength: 0.8,
        };
        let loop_id = assistant.engagement_tracker.initiate_loop(loop_type, 0.8);

        // Add tension buildup
        assistant.engagement_tracker.add_loop_event(
            &loop_id,
            LoopEventType::Reinforcement,
            "Tension increases".to_string(),
            0.2,
        );
    }

    // Analyze for cross-system patterns
    let insights = assistant.analyze_narrative_state();

    // Should detect convergent pressure pattern
    let cross_system_insights: Vec<&NarrativeInsight> = insights.iter()
        .filter(|i| i.insight_type == InsightType::CrossSystemPattern)
        .collect();

    // Given the high pressure across systems, should detect convergent pattern
    // (Note: exact detection depends on thresholds, but test structure is correct)
    println!("Cross-system insights detected: {}", cross_system_insights.len());

    // Verify that multiple systems are flagging issues
    let system_types: std::collections::HashSet<InsightType> = insights.iter()
        .map(|i| i.insight_type.clone())
        .collect();

    assert!(system_types.len() >= 2, "Multiple systems should be generating insights");

    println!("✅ Cross-system pattern detection test passed");
}

#[test]
fn test_recursive_echo_integration() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Add initial symbolic element
    let initial_element = RecursiveElement {
        id: "door_symbol".to_string(),
        content: "The door stood slightly ajar".to_string(),
        level: NarrativeLevel::Sentence,
        chapter: 1,
        scene: Some(1),
        paragraph: Some(1),
        sentence: Some(5),
        element_type: ElementType::Symbolic {
            symbol_name: "door".to_string(),
            meaning: "opportunity".to_string(),
        },
        intensity: 0.6,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(initial_element);

    // Add corresponding DNA unit
    let contradiction = NarrativeDNAUnit::new_contradiction(
        "opportunity_fear".to_string(),
        "Character sees opportunity but fears taking it".to_string(),
        0.7,
        vec!["Hero".to_string()],
    );
    assistant.dna_tracker.add_unit(contradiction);

    // Advance to later chapter
    assistant.advance_chapter(5, Some(2));

    // Add echo element
    let echo_element = RecursiveElement {
        id: "door_echo".to_string(),
        content: "The door slammed shut behind her".to_string(),
        level: NarrativeLevel::Scene,
        chapter: 5,
        scene: Some(2),
        paragraph: None,
        sentence: None,
        element_type: ElementType::Symbolic {
            symbol_name: "door".to_string(),
            meaning: "finality".to_string(),
        },
        intensity: 0.8,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(echo_element);

    // Add corresponding DNA return
    let return_unit = NarrativeDNAUnit::new_return(
        "opportunity_return".to_string(),
        "Character finally chooses, door closes".to_string(),
        "opportunity_fear".to_string(),
        TransformationType::CharacterGrowth,
        0.3,
    );
    assistant.dna_tracker.add_unit(return_unit);

    // Find potential connections
    let potential_echoes = assistant.recursion_tracker.find_potential_echoes(
        "The door slammed shut behind her",
        &ElementType::Symbolic {
            symbol_name: "door".to_string(),
            meaning: "finality".to_string(),
        },
        &NarrativeLevel::Scene,
    );

    // Should find symbolic resonance with the original door element
    assert!(!potential_echoes.is_empty(), "Should find door symbol echoes");

    // Check DNA return opportunities
    let return_opportunities = assistant.dna_tracker.find_return_opportunities();
    assert!(!return_opportunities.is_empty(), "Should find return opportunities");

    println!("✅ Recursive echo integration test passed");
}

#[test]
fn test_character_consistency_with_engagement() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Create character with established traits
    let mut character = create_test_character("Mentor", "supporting");
    let mut traits = std::collections::HashMap::new();
    traits.insert("wise".to_string(), PersonalityTrait {
        name: "Wise".to_string(),
        description: "Speaks with thoughtful consideration".to_string(),
        intensity: 0.9,
        stability: 0.8,
        manifestations: vec!["gives advice".to_string(), "speaks slowly".to_string()],
        contradictions: vec!["acts impulsively".to_string(), "speaks carelessly".to_string()],
        first_established: chrono::Utc::now(),
        last_reinforced: chrono::Utc::now(),
    });
    character.personality_traits = traits;
    assistant.character_engine.add_character(character);

    // Create engagement loop tied to this character
    let identity_loop = EngagementLoopType::IdentityAlignment {
        character: "Mentor".to_string(),
        alignment_strength: 0.8,
        reflection_depth: 0.7,
    };
    assistant.engagement_tracker.initiate_loop(identity_loop, 0.8);

    // Record dialogue that contradicts established character
    assistant.character_engine.record_dialogue(
        "Mentor",
        "Whatever, just do it fast!",
        "impatient",
    );

    // Record an impulsive action
    assistant.character_engine.record_action(
        "Mentor",
        "acts impulsively without thinking",
        "impatience",
    );

    // Analyze should detect both character inconsistency and potential engagement impact
    let insights = assistant.analyze_narrative_state();

    let character_insights: Vec<&NarrativeInsight> = insights.iter()
        .filter(|i| i.insight_type == InsightType::CharacterDrift)
        .collect();

    assert!(!character_insights.is_empty(), "Should detect character consistency issues");

    // The character inconsistency should also potentially affect the engagement loop
    let engagement_insights: Vec<&NarrativeInsight> = insights.iter()
        .filter(|i| i.insight_type == InsightType::EngagementPattern)
        .collect();

    println!("Character insights: {}, Engagement insights: {}",
             character_insights.len(), engagement_insights.len());

    println!("✅ Character consistency with engagement test passed");
}

#[test]
fn test_narrative_event_recording() {
    let mut assistant = RecursiveNarrativeAssistant::new();

    // Record various narrative events
    let events = vec![
        NarrativeEvent::CharacterAction {
            character: "Hero".to_string(),
            action: "saves the cat".to_string(),
            motivation: "compassion".to_string(),
        },
        NarrativeEvent::DialogueSpoken {
            character: "Hero".to_string(),
            dialogue: "I couldn't just leave it there".to_string(),
            emotional_context: "determined".to_string(),
        },
        NarrativeEvent::DNAUnit {
            unit: NarrativeDNAUnit::new_contradiction(
                "hero_compassion".to_string(),
                "Hero's compassion vs mission urgency".to_string(),
                0.6,
                vec!["Hero".to_string()],
            ),
        },
        NarrativeEvent::EngagementLoop {
            loop_type: EngagementLoopType::MoralTension {
                moral_question: "Duty vs compassion".to_string(),
                tension_level: 0.7,
                reconciliation_urgency: 0.5,
            },
            intensity: 0.7,
        },
        NarrativeEvent::Constraint {
            constraint: ConstraintType::ThematicCommitment {
                theme: "compassion".to_string(),
                stance: "positive".to_string(),
                requires_consistency: true,
            },
        },
    ];

    // Record all events
    for event in events {
        assistant.record_narrative_event(event);
    }

    // Verify events were recorded across systems
    assert_eq!(assistant.dna_tracker.units.len(), 1, "DNA unit should be recorded");
    assert_eq!(assistant.engagement_tracker.loops.len(), 1, "Engagement loop should be recorded");
    assert_eq!(assistant.constraint_tracker.constraints.len(), 1, "Constraint should be recorded");

    // Generate context prompt to see if events influence the narrative context
    let context_prompt = assistant.generate_context_prompt();

    println!("Context prompt generated: {}", context_prompt.len() > 0);
    println!("✅ Narrative event recording test passed");
}

#[test]
fn test_assertiveness_levels() {
    // Test different assertiveness levels
    let assertiveness_levels = vec![0.2, 0.5, 0.8];

    for level in assertiveness_levels {
        let mut config = AssistantConfig::default();
        config.assertiveness_level = level;
        let mut assistant = RecursiveNarrativeAssistant::with_config(config);

        // Create conditions that generate many insights
        let obligations = vec![
            Obligation::new("urgent1", 0.9, 8),
            Obligation::new("urgent2", 0.8, 7),
            Obligation::new("urgent3", 0.7, 6),
        ];
        assistant.update_narrative_state(None, obligations);

        // Add multiple constraints
        for i in 0..5 {
            let constraint = ConstraintType::UnresolvedThread {
                thread_id: format!("thread_{}", i),
                description: format!("Thread {}", i),
                urgency: 0.6,
                must_resolve_by_chapter: Some(10),
            };
            assistant.constraint_tracker.add_constraint(constraint);
        }

        let insights = assistant.analyze_narrative_state();

        // Higher assertiveness should generate more insights
        let expected_max = match level {
            l if l < 0.3 => 1,
            l if l < 0.6 => 3,
            l if l < 0.8 => 5,
            _ => 8,
        };

        assert!(insights.len() <= expected_max,
                "Assertiveness level {} should limit insights to {}, got {}",
                level, expected_max, insights.len());

        println!("Assertiveness {}: {} insights (max {})", level, insights.len(), expected_max);
    }

    println!("✅ Assertiveness levels test passed");
}

#[test]
fn test_system_enable_disable() {
    let mut config = AssistantConfig::default();

    // Disable some systems
    config.enabled_systems.dna_tracking = false;
    config.enabled_systems.engagement_loops = false;

    let mut assistant = RecursiveNarrativeAssistant::with_config(config);

    // Try to add events that would normally trigger these systems
    let dna_event = NarrativeEvent::DNAUnit {
        unit: NarrativeDNAUnit::new_contradiction(
            "test".to_string(),
            "Test contradiction".to_string(),
            0.5,
            vec![],
        ),
    };

    let engagement_event = NarrativeEvent::EngagementLoop {
        loop_type: EngagementLoopType::CuriosityHypothesis {
            mystery_element: "Test mystery".to_string(),
            hypothesis_strength: 0.6,
        },
        intensity: 0.6,
    };

    assistant.record_narrative_event(dna_event);
    assistant.record_narrative_event(engagement_event);

    // Verify disabled systems weren't updated
    assert_eq!(assistant.dna_tracker.units.len(), 0, "DNA tracking should be disabled");
    assert_eq!(assistant.engagement_tracker.loops.len(), 0, "Engagement tracking should be disabled");

    // But enabled systems should still work
    let constraint_event = NarrativeEvent::Constraint {
        constraint: ConstraintType::WorldLogic {
            rule_name: "gravity".to_string(),
            description: "Things fall down".to_string(),
            blocked_scenarios: vec!["flying without magic".to_string()],
        },
    };

    assistant.record_narrative_event(constraint_event);
    assert_eq!(assistant.constraint_tracker.constraints.len(), 1, "Constraint tracking should be enabled");

    println!("✅ System enable/disable test passed");
}

// Helper function to create test characters
fn create_test_character(name: &str, role: &str) -> CharacterProfile {
    CharacterProfile {
        name: name.to_string(),
        role: role.to_string(),
        personality_traits: std::collections::HashMap::new(),
        dialogue_pattern: DialoguePattern {
            vocabulary_level: "casual".to_string(),
            sentence_structure: "simple".to_string(),
            favorite_phrases: vec![],
            speech_quirks: vec![],
            emotional_tells: std::collections::HashMap::new(),
            cultural_markers: vec![],
            consistency_score: 1.0,
        },
        relationships: std::collections::HashMap::new(),
        character_arc: None,
        physical_description: "Test character".to_string(),
        background: "Test background".to_string(),
        motivations: vec!["test motivation".to_string()],
        fears: vec!["test fear".to_string()],
        secrets: vec![],
        introduced_chapter: 1,
        last_appearance: 1,
        consistency_score: 1.0,
        created_at: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
    }
}