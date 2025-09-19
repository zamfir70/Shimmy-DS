/// 📖 Usage Example: Recursive Narrative Tracking System
///
/// This example demonstrates how to use the complete recursive narrative
/// tracking system implemented for Shimmy-DS. This showcases the integration
/// of all GPT-4o's recursive thinking recommendations.

use crate::recursive_narrative_assistant::*;
use crate::narrative_dna::*;
use crate::constraint_space::*;
use crate::multi_level_recursion::*;
use crate::character_consistency::*;
use crate::reader_engagement_loops::*;
use crate::obligation_pressure::Obligation;
use crate::emotion_resonance::EmotionalState;

/// Comprehensive example showing the recursive narrative system in action
pub fn demonstrate_recursive_narrative_system() {
    println!("🚀 Shimmy-DS Recursive Narrative System Demo");
    println!("==============================================");

    // Step 1: Initialize the unified assistant
    let mut assistant = RecursiveNarrativeAssistant::new();
    println!("✅ Initialized Recursive Narrative Assistant");

    // Step 2: Set up a character with personality and voice
    demonstrate_character_setup(&mut assistant);

    // Step 3: Establish narrative DNA foundations
    demonstrate_narrative_dna(&mut assistant);

    // Step 4: Create constraint space and reader engagement
    demonstrate_constraints_and_engagement(&mut assistant);

    // Step 5: Add multi-level recursive elements
    demonstrate_multi_level_recursion(&mut assistant);

    // Step 6: Advance story and track evolution
    demonstrate_story_progression(&mut assistant);

    // Step 7: Generate comprehensive analysis
    demonstrate_analysis_and_insights(&mut assistant);

    println!("\n🎉 Demo completed successfully!");
}

/// Demonstrates character consistency tracking
fn demonstrate_character_setup(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n📖 Step 2: Character Setup");
    println!("---------------------------");

    // Create a complex character
    let mut character = CharacterProfile {
        name: "Elena".to_string(),
        role: "protagonist".to_string(),
        personality_traits: std::collections::HashMap::new(),
        dialogue_pattern: DialoguePattern {
            vocabulary_level: "educated".to_string(),
            sentence_structure: "complex".to_string(),
            favorite_phrases: vec!["I suppose".to_string(), "rather fascinating".to_string()],
            speech_quirks: vec!["uses precise language".to_string()],
            emotional_tells: {
                let mut tells = std::collections::HashMap::new();
                tells.insert("nervous".to_string(), "speaks faster".to_string());
                tells.insert("angry".to_string(), "becomes very formal".to_string());
                tells
            },
            cultural_markers: vec!["academic background".to_string()],
            consistency_score: 1.0,
        },
        relationships: std::collections::HashMap::new(),
        character_arc: Some(CharacterArc {
            character_name: "Elena".to_string(),
            arc_theme: "learning to trust intuition over logic".to_string(),
            starting_state: "relies entirely on rational analysis".to_string(),
            desired_end_state: "balances logic with intuition".to_string(),
            current_progress: 0.1,
            arc_milestones: vec![],
            obstacles_faced: vec!["fear of being wrong".to_string()],
            growth_moments: vec![],
            regression_moments: vec![],
            consistency_with_theme: 0.9,
        }),
        physical_description: "Tall, observant eyes, careful movements".to_string(),
        background: "Academic researcher turned reluctant adventurer".to_string(),
        motivations: vec!["find the truth".to_string(), "protect her research".to_string()],
        fears: vec!["making decisions without enough data".to_string(), "trusting the wrong person".to_string()],
        secrets: vec!["her research predicted this crisis".to_string()],
        introduced_chapter: 1,
        last_appearance: 1,
        consistency_score: 1.0,
        created_at: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
    };

    // Add core personality traits
    character.personality_traits.insert("analytical".to_string(), PersonalityTrait {
        name: "Analytical".to_string(),
        description: "Approaches problems through systematic analysis".to_string(),
        intensity: 0.9,
        stability: 0.8,
        manifestations: vec![
            "asks probing questions".to_string(),
            "hesitates before acting".to_string(),
            "notices details others miss".to_string(),
        ],
        contradictions: vec![
            "acts on pure impulse".to_string(),
            "ignores obvious evidence".to_string(),
        ],
        first_established: chrono::Utc::now(),
        last_reinforced: chrono::Utc::now(),
    });

    assistant.character_engine.add_character(character);
    println!("✅ Created Elena with analytical personality and trust-vs-logic arc");

    // Record some initial dialogue to establish voice
    assistant.character_engine.record_dialogue(
        "Elena",
        "I suppose we should examine this rather fascinating anomaly more carefully before proceeding.",
        "cautious curiosity",
    );
    println!("✅ Recorded baseline dialogue for voice consistency");
}

/// Demonstrates CAPR narrative DNA tracking
fn demonstrate_narrative_dna(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n🧬 Step 3: Narrative DNA (CAPR)");
    println!("--------------------------------");

    // Contradiction: Core story tension
    let contradiction = NarrativeDNAUnit::new_contradiction(
        "logic_vs_intuition".to_string(),
        "Elena must choose between logical analysis and intuitive leaps".to_string(),
        0.8,
        vec!["Elena".to_string()],
    );
    assistant.dna_tracker.add_unit(contradiction);
    println!("✅ Added core Contradiction: logic vs intuition");

    // Action: Attempt to resolve contradiction
    let action = NarrativeDNAUnit::new_action(
        "research_ancient_texts".to_string(),
        "Elena dives deeper into research to find logical proof".to_string(),
        Some("logic_vs_intuition".to_string()),
        0.7,
    );
    assistant.dna_tracker.add_unit(action);
    println!("✅ Added Action: attempting resolution through more research");

    // Pressure: Unintended consequence
    let pressure = NarrativeDNAUnit::new_pressure(
        "time_running_out".to_string(),
        "Research takes too long, crisis escalates while Elena analyzes".to_string(),
        Some("research_ancient_texts".to_string()),
        0.9,
        vec!["Elena".to_string(), "Village".to_string()],
    );
    assistant.dna_tracker.add_unit(pressure);
    println!("✅ Added Pressure: time constraint forces decision");

    // Set up for future Return (would be added later in story)
    println!("💭 Return unit will be added when Elena learns to trust intuition");
}

/// Demonstrates constraint space modeling and reader engagement
fn demonstrate_constraints_and_engagement(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n🗺️ Step 4: Constraints & Engagement");
    println!("------------------------------------");

    // Add character state constraints
    let character_constraint = ConstraintType::CharacterState {
        character: "Elena".to_string(),
        trait_name: "analytical_paralysis".to_string(),
        prevents_actions: vec![
            "quick decisions".to_string(),
            "acting on hunches".to_string(),
            "trusting strangers immediately".to_string(),
        ],
    };
    assistant.constraint_tracker.add_constraint(character_constraint);
    println!("✅ Added character constraint: analytical paralysis");

    // Add world logic constraints
    let world_constraint = ConstraintType::WorldLogic {
        rule_name: "magic_requires_belief".to_string(),
        description: "Magic only works for those who truly believe".to_string(),
        blocked_scenarios: vec![
            "Elena casting spells through pure logic".to_string(),
            "magic working for complete skeptics".to_string(),
        ],
    };
    assistant.constraint_tracker.add_constraint(world_constraint);
    println!("✅ Added world constraint: magic requires belief");

    // Add thematic commitment
    let theme_constraint = ConstraintType::ThematicCommitment {
        theme: "balance of logic and intuition".to_string(),
        stance: "both are necessary".to_string(),
        requires_consistency: true,
    };
    assistant.constraint_tracker.add_constraint(theme_constraint);
    println!("✅ Added thematic commitment: logic/intuition balance");

    // Create reader engagement loops
    let curiosity_loop = EngagementLoopType::CuriosityHypothesis {
        mystery_element: "What caused the magical anomaly?".to_string(),
        hypothesis_strength: 0.8,
    };
    assistant.engagement_tracker.initiate_loop(curiosity_loop, 0.8);
    println!("✅ Created curiosity loop: magical anomaly mystery");

    let identity_loop = EngagementLoopType::IdentityAlignment {
        character: "Elena".to_string(),
        alignment_strength: 0.7,
        reflection_depth: 0.8,
    };
    assistant.engagement_tracker.initiate_loop(identity_loop, 0.7);
    println!("✅ Created identity alignment loop with Elena");

    let moral_loop = EngagementLoopType::MoralTension {
        moral_question: "Is it right to risk everything on intuition?".to_string(),
        tension_level: 0.6,
        reconciliation_urgency: 0.4,
    };
    assistant.engagement_tracker.initiate_loop(moral_loop, 0.6);
    println!("✅ Created moral tension loop: intuition vs safety");
}

/// Demonstrates multi-level recursion tracking
fn demonstrate_multi_level_recursion(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n🔄 Step 5: Multi-Level Recursion");
    println!("--------------------------------");

    // Sentence-level symbolic element
    let mirror_symbol = RecursiveElement {
        id: "scrying_mirror".to_string(),
        content: "The mirror's surface rippled like water, showing impossible truths".to_string(),
        level: NarrativeLevel::Sentence,
        chapter: 1,
        scene: Some(2),
        paragraph: Some(3),
        sentence: Some(7),
        element_type: ElementType::Symbolic {
            symbol_name: "mirror".to_string(),
            meaning: "reflection and truth-seeing".to_string(),
        },
        intensity: 0.8,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(mirror_symbol);
    println!("✅ Added sentence-level symbolic element: scrying mirror");

    // Scene-level thematic element
    let choice_theme = RecursiveElement {
        id: "first_intuitive_choice".to_string(),
        content: "Elena's first moment of choosing feeling over facts".to_string(),
        level: NarrativeLevel::Scene,
        chapter: 1,
        scene: Some(2),
        paragraph: None,
        sentence: None,
        element_type: ElementType::Thematic {
            theme: "intuition vs logic".to_string(),
            position: "intuition emerges".to_string(),
        },
        intensity: 0.6,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(choice_theme);
    println!("✅ Added scene-level thematic element: first intuitive choice");

    // Chapter-level character moment
    let character_moment = RecursiveElement {
        id: "elena_vulnerability".to_string(),
        content: "Elena admits she doesn't have all the answers".to_string(),
        level: NarrativeLevel::Chapter,
        chapter: 1,
        scene: None,
        paragraph: None,
        sentence: None,
        element_type: ElementType::CharacterMoment {
            character: "Elena".to_string(),
            trait_revealed: "hidden insecurity".to_string(),
        },
        intensity: 0.7,
        created_at: chrono::Utc::now(),
        echoes: Vec::new(),
    };
    assistant.recursion_tracker.add_element(character_moment);
    println!("✅ Added chapter-level character moment: Elena's vulnerability");
}

/// Demonstrates story progression and system evolution
fn demonstrate_story_progression(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n📚 Step 6: Story Progression");
    println!("-----------------------------");

    // Update obligations and emotional state
    let obligations = vec![
        Obligation::new("solve_magical_crisis", 0.9, 1),
        Obligation::new("protect_village", 0.8, 1),
        Obligation::new("preserve_research", 0.6, 0),
        Obligation::new("learn_to_trust_intuition", 0.7, 1),
    ];

    let emotional_state = EmotionalState::with_secondary(
        "determination", 0.7,
        "anxiety", 0.4,
    );

    assistant.update_narrative_state(Some(emotional_state), obligations);
    println!("✅ Updated obligations and emotional state");

    // Advance to chapter 3 (time has passed, pressure building)
    assistant.advance_chapter(3, Some(1));
    println!("✅ Advanced to Chapter 3, Scene 1");

    // Record character growth moment
    assistant.character_engine.record_action(
        "Elena",
        "decides to trust her gut feeling about the ritual",
        "desperation and growing intuition",
    );
    println!("✅ Recorded character growth: Elena trusts intuition");

    // Add reinforcement to engagement loops
    let loop_ids: Vec<String> = assistant.engagement_tracker.loops.keys().cloned().collect();
    for loop_id in &loop_ids {
        assistant.engagement_tracker.add_loop_event(
            loop_id,
            LoopEventType::Reinforcement,
            "Stakes increase as crisis escalates".to_string(),
            0.2,
        );
    }
    println!("✅ Reinforced engagement loops with rising stakes");

    // Update constraint space (Elena's growth reduces some constraints)
    assistant.constraint_tracker.record_choice(
        "Elena chooses intuition over analysis".to_string(),
        vec!["trust_in_intuition".to_string()],
        vec!["pure_analytical_approach".to_string()],
    );
    println!("✅ Updated constraint space reflecting character growth");
}

/// Demonstrates analysis and insight generation
fn demonstrate_analysis_and_insights(assistant: &mut RecursiveNarrativeAssistant) {
    println!("\n🤖 Step 7: Analysis & Insights");
    println!("-------------------------------");

    // Generate insights across all systems
    let insights = assistant.analyze_narrative_state();
    println!("📊 Generated {} narrative insights", insights.len());

    for (i, insight) in insights.iter().take(3).enumerate() {
        println!("  {}. [{:?}] {}", i + 1, insight.priority, insight.title);
        println!("     {}", insight.description);
        if !insight.questions.is_empty() {
            println!("     Consider: {}", insight.questions[0]);
        }
        println!();
    }

    // Generate context prompt for AI integration
    let context_prompt = assistant.generate_context_prompt();
    if !context_prompt.is_empty() {
        println!("🎯 Generated Context Prompt:");
        println!("{}", context_prompt);
    }

    // Show comprehensive report (truncated for demo)
    let report = assistant.generate_comprehensive_report();
    let report_lines: Vec<&str> = report.lines().take(15).collect();
    println!("📋 Comprehensive Report (first 15 lines):");
    for line in report_lines {
        println!("  {}", line);
    }
    println!("  ... (truncated)");

    // Demonstrate individual system reports
    println!("\n🔍 Individual System Health:");

    let dna_health = assistant.dna_tracker.analyze_pattern_health();
    println!("  • DNA Health: {:.2}/1.0 ({} units)", dna_health.health_score, dna_health.total_units);

    let constraint_pressure = assistant.constraint_tracker.analyze_constraint_pressure();
    println!("  • Constraint Freedom: {:.2}/1.0", constraint_pressure.freedom_score);

    let recursion_health = assistant.recursion_tracker.analyze_recursion_health();
    println!("  • Recursion Health: {:.2}/1.0 (density: {:.2})",
             recursion_health.health_score, recursion_health.recursion_density);

    let character_consistency = assistant.character_engine.global_consistency;
    println!("  • Character Consistency: {:.2}/1.0", character_consistency);

    let reader_retention = assistant.engagement_tracker.engagement_metrics.reader_retention_score;
    println!("  • Reader Retention: {:.2}/1.0", reader_retention);

    // Show potential recursive opportunities
    let return_opportunities = assistant.dna_tracker.find_return_opportunities();
    if !return_opportunities.is_empty() {
        println!("\n🔄 Recursive Return Opportunities:");
        for (i, opp) in return_opportunities.iter().take(2).enumerate() {
            println!("  {}. {} (pressure: {:.2})", i + 1, opp.description, opp.intensity_score);
        }
    }

    // Show cross-system patterns
    let cross_patterns = assistant.analyze_cross_system_patterns();
    if !cross_patterns.is_empty() {
        println!("\n🌐 Cross-System Patterns Detected:");
        for pattern in &cross_patterns {
            println!("  • {}: {}", pattern.title, pattern.description);
        }
    }
}

/// Configuration example showing different assistant personalities
pub fn demonstrate_assistant_configurations() {
    println!("\n⚙️ Assistant Configuration Examples");
    println!("====================================");

    // Passive assistant (minimal intrusion)
    let passive_config = AssistantConfig {
        auto_pattern_detection: true,
        assertiveness_level: 0.3,
        enabled_systems: EnabledSystems {
            dna_tracking: true,
            constraint_modeling: true,
            recursion_tracking: false, // Disabled for simplicity
            character_consistency: true,
            engagement_loops: true,
            drift_stabilization: true,
        },
        sensitivity: SensitivitySettings {
            constraint_pressure: 0.8, // Only flag major issues
            character_drift: 0.9,
            unresolved_loops: 0.8,
            engagement_drops: 0.8,
            pattern_breaks: 0.7,
        },
        drift_config: DriftStabilizerConfig::default(),
    };

    let passive_assistant = RecursiveNarrativeAssistant::with_config(passive_config);
    println!("✅ Passive Assistant: Low assertiveness, high thresholds");

    // Active assistant (comprehensive guidance)
    let active_config = AssistantConfig {
        auto_pattern_detection: true,
        assertiveness_level: 0.8,
        enabled_systems: EnabledSystems {
            dna_tracking: true,
            constraint_modeling: true,
            recursion_tracking: true,
            character_consistency: true,
            engagement_loops: true,
            drift_stabilization: true,
        },
        sensitivity: SensitivitySettings {
            constraint_pressure: 0.5, // Flag issues early
            character_drift: 0.6,
            unresolved_loops: 0.4,
            engagement_drops: 0.5,
            pattern_breaks: 0.4,
        },
        drift_config: DriftStabilizerConfig::default(),
    };

    let active_assistant = RecursiveNarrativeAssistant::with_config(active_config);
    println!("✅ Active Assistant: High assertiveness, low thresholds");

    // Specialized assistant (focus on specific systems)
    let mut specialized_config = AssistantConfig::default();
    specialized_config.enabled_systems.constraint_modeling = false;
    specialized_config.enabled_systems.recursion_tracking = false;
    specialized_config.assertiveness_level = 0.6;

    let specialized_assistant = RecursiveNarrativeAssistant::with_config(specialized_config);
    println!("✅ Specialized Assistant: Character & engagement focus only");
}

/// Integration with existing Shimmy-DS systems example
pub fn demonstrate_shimmy_integration() {
    println!("\n🔗 Shimmy-DS Integration Example");
    println!("=================================");

    let mut assistant = RecursiveNarrativeAssistant::new();

    // This would integrate with existing shimmy systems:
    println!("💭 Integration points with existing Shimmy-DS:");
    println!("  • Prompt Injector: Use context_prompt() for narrative awareness");
    println!("  • Emotion Resonance: Feed emotional states to update_narrative_state()");
    println!("  • Obligation Pressure: Use existing obligations in comprehensive tracking");
    println!("  • Waymark Validator: Validate consistency with character constraints");
    println!("  • Recursive Drift Stabilizer: Enhanced with multi-system drift detection");

    // Example of generating context for prompt injection
    let context = assistant.generate_context_prompt();
    println!("\n📝 Generated context for prompt injection:");
    if context.is_empty() {
        println!("  (No high-priority insights at this time)");
    } else {
        println!("{}", context);
    }

    // Example of how this would integrate with existing emotional system
    println!("\n🎭 Integration with emotion resonance:");
    println!("  Current emotional field: {:?}",
             assistant.current_emotional_state.as_ref().map(|s| s.describe()));

    // Example of comprehensive analysis for decision support
    println!("\n🧠 Comprehensive analysis for narrative decisions:");
    let insights = assistant.analyze_narrative_state();
    println!("  Generated {} insights across {} active systems",
             insights.len(),
             count_active_systems(&assistant.config.enabled_systems));
}

fn count_active_systems(systems: &EnabledSystems) -> usize {
    let mut count = 0;
    if systems.dna_tracking { count += 1; }
    if systems.constraint_modeling { count += 1; }
    if systems.recursion_tracking { count += 1; }
    if systems.character_consistency { count += 1; }
    if systems.engagement_loops { count += 1; }
    if systems.drift_stabilization { count += 1; }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_example_runs() {
        // This test ensures the usage example compiles and runs without panicking
        demonstrate_recursive_narrative_system();
        demonstrate_assistant_configurations();
        demonstrate_shimmy_integration();
    }
}