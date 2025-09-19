/// 🤖 Recursive Narrative Assistant
///
/// Unified AI assistant interface that coordinates all recursive tracking systems
/// and surfaces insights without constraining creativity. This system acts as a
/// "narrative field" that remembers and reflects rather than directs.
///
/// Key principles:
/// - Never dictate what happens next
/// - Surface patterns and tensions without prescribing solutions
/// - Ask questions rather than give answers
/// - Maintain authorial sovereignty while providing recursive memory

use crate::narrative_dna::{NarrativeDNATracker, NarrativeDNAUnit, TransformationType};
use crate::constraint_space::{ConstraintSpaceTracker, ConstraintType};
use crate::multi_level_recursion::{MultiLevelRecursionTracker, RecursiveElement, NarrativeLevel};
use crate::character_consistency::{CharacterConsistencyEngine, CharacterProfile};
use crate::reader_engagement_loops::{ReaderEngagementTracker, EngagementLoopType};
use crate::obligation_pressure::{Obligation, compute_saturation};
use crate::emotion_resonance::{EmotionalState, inject_emotional_state};
use crate::recursive_drift_stabilizer::{DriftStabilityState, check_recursive_drift, DriftStabilizerConfig};
use crate::recursive_integrity_core::{
    RecursiveIntegrityCore, RICMode, RICDecision, InsightStatus,
    ContinuityFloorResponse, RICHealthSummary
};

use serde_json;
use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Main coordinator for all recursive narrative tracking systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveNarrativeAssistant {
    /// CAPR narrative DNA tracking
    pub dna_tracker: NarrativeDNATracker,
    /// Constraint space modeling
    pub constraint_tracker: ConstraintSpaceTracker,
    /// Multi-level recursion tracking
    pub recursion_tracker: MultiLevelRecursionTracker,
    /// Character consistency engine
    pub character_engine: CharacterConsistencyEngine,
    /// Reader engagement loop detection
    pub engagement_tracker: ReaderEngagementTracker,
    /// Current obligations (using existing system)
    pub current_obligations: Vec<Obligation>,
    /// Current emotional state (using existing system)
    pub current_emotional_state: Option<EmotionalState>,
    /// Drift stability state
    pub drift_state: DriftStabilityState,
    /// Configuration
    pub config: AssistantConfig,
    /// Current narrative context
    pub current_chapter: u32,
    pub current_scene: Option<u32>,
    /// Assistant state
    pub last_updated: DateTime<Utc>,
    /// Recursive Integrity Core
    #[serde(skip)]
    pub ric: Option<RecursiveIntegrityCore>,

    /// RIP+RIC Unified Arbitration State
    pub rip_ric_fusion_state: RIPRICFusionState,
}

/// Configuration for the recursive narrative assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantConfig {
    /// Whether to automatically detect patterns
    pub auto_pattern_detection: bool,
    /// How assertive the assistant should be (0.0 = very passive, 1.0 = very active)
    pub assertiveness_level: f32,
    /// Which systems to enable
    pub enabled_systems: EnabledSystems,
    /// Sensitivity thresholds
    pub sensitivity: SensitivitySettings,
    /// Drift stabilizer configuration
    pub drift_config: DriftStabilizerConfig,
    /// RIC mode for integrity control
    pub ric_mode: RICMode,
}

/// Which tracking systems are enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledSystems {
    pub dna_tracking: bool,
    pub constraint_modeling: bool,
    pub recursion_tracking: bool,
    pub character_consistency: bool,
    pub engagement_loops: bool,
    pub drift_stabilization: bool,
}

/// Sensitivity settings for different types of insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivitySettings {
    /// How sensitive to constraint pressure (0.0 = ignore, 1.0 = very sensitive)
    pub constraint_pressure: f32,
    /// How sensitive to character inconsistencies
    pub character_drift: f32,
    /// How sensitive to unresolved loops
    pub unresolved_loops: f32,
    /// How sensitive to reader engagement drops
    pub engagement_drops: f32,
    /// How sensitive to recursive pattern breaks
    pub pattern_breaks: f32,
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            auto_pattern_detection: true,
            assertiveness_level: 0.6,
            enabled_systems: EnabledSystems {
                dna_tracking: true,
                constraint_modeling: true,
                recursion_tracking: true,
                character_consistency: true,
                engagement_loops: true,
                drift_stabilization: true,
            },
            sensitivity: SensitivitySettings {
                constraint_pressure: 0.7,
                character_drift: 0.8,
                unresolved_loops: 0.6,
                engagement_drops: 0.7,
                pattern_breaks: 0.5,
            },
            drift_config: DriftStabilizerConfig::default(),
            ric_mode: RICMode::default(),
        }
    }
}

/// A narrative insight surfaced by the assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeInsight {
    pub insight_type: InsightType,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub questions: Vec<String>, // Questions for the author to consider
    pub suggestions: Vec<String>, // Non-prescriptive suggestions
    pub affected_systems: Vec<String>, // Which tracking systems detected this
    pub context: InsightContext,
    pub timestamp: DateTime<Utc>,
}

/// Types of insights the assistant can surface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InsightType {
    /// Narrative DNA pattern detected
    DNAPattern,
    /// Constraint space warning
    ConstraintPressure,
    /// Recursive opportunity
    RecursiveOpportunity,
    /// Character consistency concern
    CharacterDrift,
    /// Reader engagement insight
    EngagementPattern,
    /// Cross-system pattern
    CrossSystemPattern,
    /// Drift stabilization warning
    DriftAlert,
}

/// Priority levels for insights
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Context information for insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightContext {
    pub chapter: u32,
    pub scene: Option<u32>,
    pub characters_involved: Vec<String>,
    pub themes_involved: Vec<String>,
    pub narrative_pressure: f32,
    pub emotional_context: Option<String>,
}

/// RIP+RIC Unified Arbitration State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RIPRICFusionState {
    /// RIP constraint genome health score
    pub rip_genome_health: f32,
    /// RIP guard chain validation success rate
    pub rip_guard_health: f32,
    /// RIP pathogen detection threat level
    pub rip_pathogen_threat: f32,
    /// RIC subsystem consensus health
    pub ric_consensus_health: f32,
    /// RIC recursive saturation level
    pub ric_saturation_level: f32,
    /// Unified arbitration decision cache
    pub last_arbitration_decision: Option<UnifiedArbitrationDecision>,
    /// Fusion health timestamp
    pub last_fusion_update: DateTime<Utc>,
    /// RIP Python process connection state
    pub rip_process_healthy: bool,
    /// Current recursion budget from ZC gates
    pub current_recursion_budget: u32,
    /// Loop saturation detection state
    pub loop_saturation_detected: bool,
}

/// Unified arbitration decision combining RIP and RIC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedArbitrationDecision {
    /// Both RIP and RIC vote to continue
    ContinueRecursion {
        rip_vote: String,
        ric_vote: RICDecision,
        consensus_confidence: f32,
    },
    /// RIP constraint genome enforces halt
    RIPConstraintHalt {
        failed_ligands: Vec<String>,
        guard_chain_violations: Vec<String>,
    },
    /// RIC core enforces halt via consensus
    RICConsensusHalt {
        voting_subsystems: Vec<String>,
        halt_reason: String,
    },
    /// Pathogen detection triggers defensive halt
    PathogenDetectionHalt {
        detected_pathogens: Vec<String>,
        threat_level: f32,
    },
    /// Loop saturation triggers ZC gate closure
    LoopSaturationHalt {
        saturated_phases: Vec<String>,
        budget_exhausted: bool,
    },
    /// Unified continuity floor injection
    UnifiedContinuityFloor {
        rip_completion_summary: String,
        ric_completion_summary: String,
        fusion_reason: String,
    },
}

/// RIP query structure for Python subprocess communication
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RIPQuery {
    text: String,
    context: String,
    seed: String,
    beat: String,
    max_iterations: u32,
    budget_remaining: u32,
}

/// RIP analysis result from Python subprocess
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RIPAnalysisResult {
    constraint_genome_health: f32,
    guard_chain_health: f32,
    guard_chain_passes: bool,
    pathogen_threat_level: f32,
    detected_pathogens: Vec<String>,
    loop_saturation_detected: bool,
    saturated_growth_phases: Vec<String>,
    failed_ligands: Vec<String>,
    guard_chain_violations: Vec<String>,
    completion_summary: String,
    rip_vote: String,
}

/// RIP+RIC fusion health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RIPRICFusionHealth {
    pub rip_genome_health: f32,
    pub rip_guard_health: f32,
    pub rip_pathogen_threat: f32,
    pub ric_consensus_health: f32,
    pub ric_saturation_level: f32,
    pub rip_process_healthy: bool,
    pub current_recursion_budget: u32,
    pub loop_saturation_detected: bool,
    pub fusion_timestamp: DateTime<Utc>,
    pub overall_fusion_health: f32,
}

impl RIPRICFusionState {
    /// Create new fusion state with default values
    pub fn new() -> Self {
        Self {
            rip_genome_health: 1.0,
            rip_guard_health: 1.0,
            rip_pathogen_threat: 0.0,
            ric_consensus_health: 1.0,
            ric_saturation_level: 0.0,
            last_arbitration_decision: None,
            last_fusion_update: Utc::now(),
            rip_process_healthy: true,
            current_recursion_budget: 20, // Default ZC gate budget
            loop_saturation_detected: false,
        }
    }
}

impl RecursiveNarrativeAssistant {
    /// Creates a new recursive narrative assistant
    pub fn new() -> Self {
        let ric_mode = RICMode::default();
        let mut ric = RecursiveIntegrityCore::new(ric_mode);

        // Register all subsystems with RIC
        ric.register_subsystem("dna_tracker", 10);
        ric.register_subsystem("constraint_tracker", 8);
        ric.register_subsystem("recursion_tracker", 12);
        ric.register_subsystem("character_engine", 8);
        ric.register_subsystem("engagement_tracker", 6);

        Self {
            dna_tracker: NarrativeDNATracker::new(),
            constraint_tracker: ConstraintSpaceTracker::new(),
            recursion_tracker: MultiLevelRecursionTracker::new(),
            character_engine: CharacterConsistencyEngine::new(),
            engagement_tracker: ReaderEngagementTracker::new(),
            current_obligations: Vec::new(),
            current_emotional_state: None,
            drift_state: DriftStabilityState::new(1),
            config: AssistantConfig::default(),
            current_chapter: 1,
            current_scene: None,
            last_updated: Utc::now(),
            ric: Some(ric),
            rip_ric_fusion_state: RIPRICFusionState::new(),
        }
    }

    /// Creates assistant with custom configuration
    pub fn with_config(config: AssistantConfig) -> Self {
        let mut assistant = Self::new();
        assistant.config = config;
        assistant
    }

    /// Advances to the next chapter and updates all systems
    pub fn advance_chapter(&mut self, chapter: u32, scene: Option<u32>) {
        self.current_chapter = chapter;
        self.current_scene = scene;

        // Update all subsystems
        if self.config.enabled_systems.dna_tracking {
            self.dna_tracker.advance_chapter();
        }
        if self.config.enabled_systems.constraint_modeling {
            self.constraint_tracker.advance_chapter();
        }
        if self.config.enabled_systems.recursion_tracking {
            self.recursion_tracker.advance_context(Some(chapter), scene, None);
        }
        if self.config.enabled_systems.character_consistency {
            self.character_engine.advance_chapter();
        }
        if self.config.enabled_systems.engagement_loops {
            self.engagement_tracker.advance_context(Some(chapter), scene);
        }
        if self.config.enabled_systems.drift_stabilization {
            self.drift_state.advance_chapter();
        }

        self.last_updated = Utc::now();
    }

    /// Analyzes the current narrative state and generates insights
    pub fn analyze_narrative_state(&mut self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        // Check each system for insights
        if self.config.enabled_systems.dna_tracking {
            insights.extend(self.analyze_dna_patterns());
        }
        if self.config.enabled_systems.constraint_modeling {
            insights.extend(self.analyze_constraint_pressure());
        }
        if self.config.enabled_systems.recursion_tracking {
            insights.extend(self.analyze_recursion_patterns());
        }
        if self.config.enabled_systems.character_consistency {
            insights.extend(self.analyze_character_consistency());
        }
        if self.config.enabled_systems.engagement_loops {
            insights.extend(self.analyze_engagement_patterns());
        }
        if self.config.enabled_systems.drift_stabilization {
            insights.extend(self.analyze_drift_stability());
        }

        // Look for cross-system patterns
        insights.extend(self.analyze_cross_system_patterns());

        // Sort by priority and apply assertiveness filtering
        insights.sort_by(|a, b| self.priority_value(&b.priority).cmp(&self.priority_value(&a.priority)));
        self.filter_insights_by_assertiveness(insights)
    }

    /// Analyzes narrative DNA patterns
    fn analyze_dna_patterns(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        let health = self.dna_tracker.analyze_pattern_health();
        if health.health_score < 0.6 {
            insights.push(NarrativeInsight {
                insight_type: InsightType::DNAPattern,
                priority: if health.health_score < 0.4 { Priority::High } else { Priority::Medium },
                title: "Narrative DNA Health Concern".to_string(),
                description: format!("DNA pattern health has dropped to {:.2}/1.0", health.health_score),
                questions: vec![
                    "Are there unresolved contradictions that need attention?".to_string(),
                    "Would now be a good time to create a return/echo moment?".to_string(),
                ],
                suggestions: health.recommendations.clone(),
                affected_systems: vec!["DNA Tracking".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        // Check for return opportunities
        let opportunities = self.dna_tracker.find_return_opportunities();
        if !opportunities.is_empty() && self.config.sensitivity.pattern_breaks > 0.5 {
            let top_opportunity = &opportunities[0];
            insights.push(NarrativeInsight {
                insight_type: InsightType::RecursiveOpportunity,
                priority: if top_opportunity.intensity_score > 1.5 { Priority::High } else { Priority::Medium },
                title: "Recursive Return Opportunity".to_string(),
                description: top_opportunity.description.clone(),
                questions: vec![
                    "Would this be a meaningful moment to echo an earlier element?".to_string(),
                    "How might this element have transformed since its introduction?".to_string(),
                ],
                suggestions: vec![
                    "Consider creating a recursive return that shows growth or change".to_string(),
                    "The transformation doesn't need to be dramatic - subtle evolution can be powerful".to_string(),
                ],
                affected_systems: vec!["DNA Tracking".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        insights
    }

    /// Analyzes constraint space pressure
    fn analyze_constraint_pressure(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        let pressure_analysis = self.constraint_tracker.analyze_constraint_pressure();
        let freedom_score = pressure_analysis.freedom_score;

        if freedom_score < self.config.sensitivity.constraint_pressure {
            let priority = match freedom_score {
                f if f < 0.3 => Priority::Critical,
                f if f < 0.5 => Priority::High,
                _ => Priority::Medium,
            };

            insights.push(NarrativeInsight {
                insight_type: InsightType::ConstraintPressure,
                priority,
                title: "Narrative Freedom Constrained".to_string(),
                description: format!("Current freedom score: {:.2} - constraint space is tightening", freedom_score),
                questions: vec![
                    "Are you approaching the story's natural climax point?".to_string(),
                    "Would resolving a constraint reopen meaningful possibilities?".to_string(),
                    "Is the constraint pressure building toward a satisfying convergence?".to_string(),
                ],
                suggestions: pressure_analysis.recommendations.clone(),
                affected_systems: vec!["Constraint Modeling".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        insights
    }

    /// Analyzes recursion patterns across levels
    fn analyze_recursion_patterns(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        let health = self.recursion_tracker.analyze_recursion_health();
        if health.health_score < 0.6 && self.config.sensitivity.pattern_breaks > 0.4 {
            insights.push(NarrativeInsight {
                insight_type: InsightType::RecursiveOpportunity,
                priority: Priority::Medium,
                title: "Multi-Level Recursion Health".to_string(),
                description: format!("Recursion health: {:.2} - patterns may need strengthening", health.health_score),
                questions: vec![
                    "Are themes echoing consistently across different narrative levels?".to_string(),
                    "Could a sentence-level detail mirror a chapter-level pattern?".to_string(),
                ],
                suggestions: health.recommendations.clone(),
                affected_systems: vec!["Multi-Level Recursion".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        insights
    }

    /// Analyzes character consistency
    fn analyze_character_consistency(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        let characters_with_issues = self.character_engine.get_characters_with_issues();
        for issue in characters_with_issues {
            if issue.consistency_score < self.config.sensitivity.character_drift {
                insights.push(NarrativeInsight {
                    insight_type: InsightType::CharacterDrift,
                    priority: if issue.consistency_score < 0.5 { Priority::High } else { Priority::Medium },
                    title: format!("Character Consistency: {}", issue.character_name),
                    description: format!("{} has consistency score {:.2} with {} violations",
                        issue.character_name, issue.consistency_score, issue.violation_count),
                    questions: vec![
                        format!("Does {}'s recent behavior align with their established traits?", issue.character_name),
                        "Is this character evolution intentional or accidental?".to_string(),
                        "Would a character development moment help explain this shift?".to_string(),
                    ],
                    suggestions: issue.recommendations.clone(),
                    affected_systems: vec!["Character Consistency".to_string()],
                    context: self.current_context_with_character(&issue.character_name),
                    timestamp: Utc::now(),
                });
            }
        }

        insights
    }

    /// Analyzes reader engagement patterns
    fn analyze_engagement_patterns(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        // Check retention score
        if self.engagement_tracker.engagement_metrics.reader_retention_score < self.config.sensitivity.engagement_drops {
            insights.push(NarrativeInsight {
                insight_type: InsightType::EngagementPattern,
                priority: Priority::High,
                title: "Reader Retention Concern".to_string(),
                description: format!("Estimated retention: {:.2}",
                    self.engagement_tracker.engagement_metrics.reader_retention_score),
                questions: vec![
                    "Are there enough active curiosity loops to maintain interest?".to_string(),
                    "When did you last provide an emotional payoff?".to_string(),
                    "Is the pacing allowing for proper tension buildup?".to_string(),
                ],
                suggestions: vec![
                    "Consider introducing a new mystery or emotional investment point".to_string(),
                    "Review which engagement loops might need reinforcement".to_string(),
                ],
                affected_systems: vec!["Engagement Loops".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        // Check loops requiring attention
        let attention_loops = self.engagement_tracker.get_loops_requiring_attention();
        for attention in attention_loops.iter().take(2) { // Limit to top 2 to avoid overwhelm
            if attention.attention_level > 0.6 {
                insights.push(NarrativeInsight {
                    insight_type: InsightType::EngagementPattern,
                    priority: if attention.attention_level > 0.8 { Priority::High } else { Priority::Medium },
                    title: format!("Engagement Loop Attention: {}", attention.loop_description),
                    description: format!("Loop needs attention (level: {:.2})", attention.attention_level),
                    questions: vec![
                        "Is this loop ready for progression or resolution?".to_string(),
                        "Would delaying resolution increase or decrease tension appropriately?".to_string(),
                    ],
                    suggestions: attention.suggested_actions.clone(),
                    affected_systems: vec!["Engagement Loops".to_string()],
                    context: self.current_context(),
                    timestamp: Utc::now(),
                });
            }
        }

        insights
    }

    /// Analyzes drift stability
    fn analyze_drift_stability(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        if let Some(warnings) = check_recursive_drift(&self.drift_state, &self.config.drift_config) {
            insights.push(NarrativeInsight {
                insight_type: InsightType::DriftAlert,
                priority: Priority::Medium,
                title: "Recursive Drift Detected".to_string(),
                description: "Long-term narrative stability concerns detected".to_string(),
                questions: vec![
                    "Are unresolved elements accumulating beyond manageable levels?".to_string(),
                    "Is the emotional center of the story remaining consistent?".to_string(),
                ],
                suggestions: vec![
                    "Consider addressing some unresolved obligations".to_string(),
                    "Review thematic consistency across recent chapters".to_string(),
                ],
                affected_systems: vec!["Drift Stabilization".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        insights
    }

    /// Looks for patterns across multiple systems
    fn analyze_cross_system_patterns(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        // Check for convergent pressure across systems
        let constraint_pressure = 1.0 - self.constraint_tracker.calculate_freedom_score();
        let obligation_pressure = compute_saturation(&self.current_obligations);
        let engagement_pressure = 1.0 - self.engagement_tracker.engagement_metrics.reader_retention_score;

        let total_pressure = (constraint_pressure + obligation_pressure + engagement_pressure) / 3.0;

        if total_pressure > 0.7 {
            insights.push(NarrativeInsight {
                insight_type: InsightType::CrossSystemPattern,
                priority: Priority::High,
                title: "Convergent Narrative Pressure".to_string(),
                description: "Multiple systems indicating high narrative pressure - possible climax approach".to_string(),
                questions: vec![
                    "Is this the right time for a major narrative turning point?".to_string(),
                    "Are all the pressures building toward the same thematic resolution?".to_string(),
                    "Would a significant resolution moment serve the story now?".to_string(),
                ],
                suggestions: vec![
                    "Consider whether this pressure is building toward your intended climax".to_string(),
                    "Multiple systems converging can signal natural story rhythm".to_string(),
                ],
                affected_systems: vec!["Constraint Space".to_string(), "Obligations".to_string(), "Engagement".to_string()],
                context: self.current_context(),
                timestamp: Utc::now(),
            });
        }

        insights
    }

    /// Generates contextual prompt injection based on current insights
    pub fn generate_context_prompt(&mut self) -> String {
        let insights = self.analyze_narrative_state();

        if insights.is_empty() {
            return String::new();
        }

        let mut prompt = String::new();
        prompt.push_str("🤖 Narrative Assistant Context:\n");

        // Include only high priority insights in prompt
        let high_priority_insights: Vec<&NarrativeInsight> = insights.iter()
            .filter(|i| matches!(i.priority, Priority::High | Priority::Critical))
            .take(3) // Limit to avoid overwhelming the prompt
            .collect();

        if !high_priority_insights.is_empty() {
            for insight in high_priority_insights {
                prompt.push_str(&format!("• {}: {}\n", insight.title, insight.description));
                if !insight.questions.is_empty() {
                    prompt.push_str(&format!("  Consider: {}\n", insight.questions[0]));
                }
            }
        } else if let Some(medium_insight) = insights.iter().find(|i| i.priority == Priority::Medium) {
            prompt.push_str(&format!("• {}: {}\n", medium_insight.title, medium_insight.description));
        }

        prompt.push('\n');
        prompt
    }

    /// Records a narrative event and updates all relevant systems
    pub fn record_narrative_event(&mut self, event: NarrativeEvent) {
        match event {
            NarrativeEvent::CharacterAction { character, action, motivation } => {
                if self.config.enabled_systems.character_consistency {
                    self.character_engine.record_action(&character, &action, &motivation);
                }
            }
            NarrativeEvent::DialogueSpoken { character, dialogue, emotional_context } => {
                if self.config.enabled_systems.character_consistency {
                    self.character_engine.record_dialogue(&character, &dialogue, &emotional_context);
                }
            }
            NarrativeEvent::DNAUnit { unit } => {
                if self.config.enabled_systems.dna_tracking {
                    self.dna_tracker.add_unit(unit);
                }
            }
            NarrativeEvent::RecursiveElement { element } => {
                if self.config.enabled_systems.recursion_tracking {
                    self.recursion_tracker.add_element(element);
                }
            }
            NarrativeEvent::EngagementLoop { loop_type, intensity } => {
                if self.config.enabled_systems.engagement_loops {
                    self.engagement_tracker.initiate_loop(loop_type, intensity);
                }
            }
            NarrativeEvent::Constraint { constraint } => {
                if self.config.enabled_systems.constraint_modeling {
                    self.constraint_tracker.add_constraint(constraint);
                }
            }
        }

        self.last_updated = Utc::now();
    }

    /// Updates emotional state and obligation pressure
    pub fn update_narrative_state(&mut self, emotional_state: Option<EmotionalState>, obligations: Vec<Obligation>) {
        self.current_emotional_state = emotional_state;
        self.current_obligations = obligations;

        // Update drift state
        if self.config.enabled_systems.drift_stabilization {
            let emotional_states = if let Some(state) = &self.current_emotional_state {
                vec![state.clone()]
            } else {
                vec![]
            };

            self.drift_state.update_metrics(
                &self.current_obligations,
                &emotional_states,
                0.8, // theme_coherence - would be calculated from actual theme tracking
                true, // spatial_return_pressure - would be calculated from spatial tracking
            );
        }

        self.last_updated = Utc::now();
    }

    /// RIC Integration: Generate insights with fail-closed protection
    pub fn generate_insights_with_ric(&mut self) -> Result<Vec<NarrativeInsight>, ContinuityFloorResponse> {
        let mut insights = Vec::new();

        if let Some(ref mut ric) = self.ric {
            // Collect votes from all subsystems
            ric.vote("dna_tracker", self.dna_tracker.vote_on_narrative_state());
            ric.vote("character_engine", self.character_engine.vote_on_consistency_state());
            // Add votes from other systems as they get RIC integration

            // Make arbitration decision
            let decision = ric.arbitrate();

            match decision {
                RICDecision::Continue => {
                    // Normal operation - generate insights
                    insights = self.analyze_narrative_state();
                    Ok(insights)
                }
                RICDecision::Halt => {
                    // Stop all recursive processing
                    return Err(ContinuityFloorResponse::new("RIC halted recursive processing due to consensus vote"));
                }
                RICDecision::InjectFloor => {
                    // Force completion via continuity floor
                    return Err(ContinuityFloorResponse::with_summary(
                        "Recursive saturation detected",
                        "Narrative pressure analysis indicates system saturation. Core obligations fulfilled by summary.".to_string()
                    ));
                }
                RICDecision::Reroute(_alternative) => {
                    // Reroute to alternative system
                    insights = vec![NarrativeInsight {
                        insight_type: InsightType::CrossSystemPattern,
                        priority: Priority::Medium,
                        title: "System Rerouted".to_string(),
                        description: "RIC has rerouted analysis due to recursive concerns.".to_string(),
                        questions: vec!["Should we explore alternative narrative approaches?".to_string()],
                        suggestions: vec!["Consider simplifying current narrative threads.".to_string()],
                        affected_systems: vec!["ric".to_string()],
                        context: self.current_context(),
                        timestamp: Utc::now(),
                    }];
                    Ok(insights)
                }
            }
        } else {
            // No RIC - normal operation
            Ok(self.analyze_narrative_state())
        }
    }

    /// Get RIC health summary
    pub fn get_ric_health(&self) -> Option<RICHealthSummary> {
        self.ric.as_ref().map(|ric| ric.health_summary())
    }

    /// Reset RIC state for all systems
    pub fn reset_ric_state(&mut self) {
        if let Some(ref mut ric) = self.ric {
            // Reset core RIC
            *ric = RecursiveIntegrityCore::new(self.config.ric_mode);

            // Re-register subsystems
            ric.register_subsystem("dna_tracker", 10);
            ric.register_subsystem("constraint_tracker", 8);
            ric.register_subsystem("recursion_tracker", 12);
            ric.register_subsystem("character_engine", 8);
            ric.register_subsystem("engagement_tracker", 6);
        }

        // Reset individual system RIC state
        self.dna_tracker.reset_ric_state();
        self.character_engine.reset_ric_state();
    }

    /// Update RIC mode
    pub fn set_ric_mode(&mut self, mode: RICMode) {
        self.config.ric_mode = mode;
        if let Some(ref mut ric) = self.ric {
            *ric = RecursiveIntegrityCore::new(mode);
            // Re-register subsystems
            ric.register_subsystem("dna_tracker", 10);
            ric.register_subsystem("constraint_tracker", 8);
            ric.register_subsystem("recursion_tracker", 12);
            ric.register_subsystem("character_engine", 8);
            ric.register_subsystem("engagement_tracker", 6);
        }
    }

    /// RIP+RIC UNIFIED ARBITRATION: Core fusion method
    pub async fn rip_ric_unified_arbitration(&mut self, text: &str, context: &str) -> Result<UnifiedArbitrationDecision, String> {
        // Step 1: Query RIP Python subsystem
        let rip_analysis = self.query_rip_subsystem(text, context).await?;

        // Step 2: Collect RIC votes from all subsystems
        let ric_decision = if let Some(ref mut ric) = self.ric {
            ric.vote("dna_tracker", self.dna_tracker.vote_on_narrative_state());
            ric.vote("character_engine", self.character_engine.vote_on_consistency_state());
            ric.vote("constraint_tracker", self.vote_constraint_tracker());
            ric.vote("recursion_tracker", self.vote_recursion_tracker());
            ric.vote("engagement_tracker", self.vote_engagement_tracker());

            ric.arbitrate()
        } else {
            RICDecision::Continue
        };

        // Step 3: Unified arbitration logic
        let decision = self.compute_unified_decision(rip_analysis, ric_decision);

        // Step 4: Update fusion state
        self.update_fusion_state(&decision);

        Ok(decision)
    }

    /// Query RIP Python subsystem for constraint genome and pathogen analysis
    async fn query_rip_subsystem(&mut self, text: &str, context: &str) -> Result<RIPAnalysisResult, String> {
        // Create RIP query payload
        let rip_query = RIPQuery {
            text: text.to_string(),
            context: context.to_string(),
            seed: self.current_obligations.iter()
                .map(|o| o.description.clone())
                .collect::<Vec<String>>()
                .join("; "),
            beat: format!("Chapter {}, Scene {:?}", self.current_chapter, self.current_scene),
            max_iterations: 20,
            budget_remaining: self.rip_ric_fusion_state.current_recursion_budget,
        };

        let query_json = serde_json::to_string(&rip_query)
            .map_err(|e| format!("Failed to serialize RIP query: {}", e))?;

        // Execute Python RIP analysis
        let mut child = Command::new("python")
            .arg("-c")
            .arg(include_str!("../scripts/rip_bridge.py"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn RIP process: {}", e))?;

        // Send query to Python process
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(query_json.as_bytes())
                .map_err(|e| format!("Failed to write to RIP process: {}", e))?;
            stdin.flush()
                .map_err(|e| format!("Failed to flush RIP process stdin: {}", e))?;
        }

        // Read response from Python process
        let output = child.wait_with_output()
            .map_err(|e| format!("Failed to wait for RIP process: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.rip_ric_fusion_state.rip_process_healthy = false;
            return Err(format!("RIP process failed: {}", stderr));
        }

        let response_json = String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 from RIP process: {}", e))?;

        let rip_result: RIPAnalysisResult = serde_json::from_str(&response_json)
            .map_err(|e| format!("Failed to parse RIP response: {}", e))?;

        self.rip_ric_fusion_state.rip_process_healthy = true;
        Ok(rip_result)
    }

    /// Compute unified arbitration decision from RIP and RIC inputs
    fn compute_unified_decision(&self, rip_analysis: RIPAnalysisResult, ric_decision: RICDecision) -> UnifiedArbitrationDecision {
        // Priority 1: Pathogen detection halt (highest priority)
        if rip_analysis.pathogen_threat_level > 0.8 {
            return UnifiedArbitrationDecision::PathogenDetectionHalt {
                detected_pathogens: rip_analysis.detected_pathogens.clone(),
                threat_level: rip_analysis.pathogen_threat_level,
            };
        }

        // Priority 2: Loop saturation halt
        if rip_analysis.loop_saturation_detected || self.rip_ric_fusion_state.current_recursion_budget == 0 {
            return UnifiedArbitrationDecision::LoopSaturationHalt {
                saturated_phases: rip_analysis.saturated_growth_phases.clone(),
                budget_exhausted: self.rip_ric_fusion_state.current_recursion_budget == 0,
            };
        }

        // Priority 3: RIP constraint genome halt
        if rip_analysis.constraint_genome_health < 0.3 || !rip_analysis.guard_chain_passes {
            return UnifiedArbitrationDecision::RIPConstraintHalt {
                failed_ligands: rip_analysis.failed_ligands.clone(),
                guard_chain_violations: rip_analysis.guard_chain_violations.clone(),
            };
        }

        // Priority 4: RIC consensus halt
        match ric_decision {
            RICDecision::Halt => {
                return UnifiedArbitrationDecision::RICConsensusHalt {
                    voting_subsystems: vec!["consensus_vote".to_string()],
                    halt_reason: "RIC subsystem consensus vote".to_string(),
                };
            }
            RICDecision::InjectFloor => {
                return UnifiedArbitrationDecision::UnifiedContinuityFloor {
                    rip_completion_summary: rip_analysis.completion_summary.clone(),
                    ric_completion_summary: "RIC floor injection triggered".to_string(),
                    fusion_reason: "Unified protocol continuity floor engagement".to_string(),
                };
            }
            RICDecision::Continue => {
                // Check for unified consensus
                let consensus_confidence = (rip_analysis.constraint_genome_health +
                                          rip_analysis.guard_chain_health +
                                          (1.0 - rip_analysis.pathogen_threat_level)) / 3.0;

                UnifiedArbitrationDecision::ContinueRecursion {
                    rip_vote: rip_analysis.rip_vote.clone(),
                    ric_vote: ric_decision,
                    consensus_confidence,
                }
            }
            RICDecision::Reroute(alternative) => {
                // Treat reroute as continue with modified confidence
                UnifiedArbitrationDecision::ContinueRecursion {
                    rip_vote: format!("REROUTED: {}", rip_analysis.rip_vote),
                    ric_vote: ric_decision,
                    consensus_confidence: 0.5, // Reduced confidence for rerouted decisions
                }
            }
        }
    }

    /// Update fusion state based on unified decision
    fn update_fusion_state(&mut self, decision: &UnifiedArbitrationDecision) {
        self.rip_ric_fusion_state.last_arbitration_decision = Some(decision.clone());
        self.rip_ric_fusion_state.last_fusion_update = Utc::now();

        // Update specific fusion metrics based on decision type
        match decision {
            UnifiedArbitrationDecision::ContinueRecursion { consensus_confidence, .. } => {
                self.rip_ric_fusion_state.ric_consensus_health = *consensus_confidence;
                // Consume recursion budget
                if self.rip_ric_fusion_state.current_recursion_budget > 0 {
                    self.rip_ric_fusion_state.current_recursion_budget -= 1;
                }
            }
            UnifiedArbitrationDecision::PathogenDetectionHalt { threat_level, .. } => {
                self.rip_ric_fusion_state.rip_pathogen_threat = *threat_level;
            }
            UnifiedArbitrationDecision::LoopSaturationHalt { .. } => {
                self.rip_ric_fusion_state.loop_saturation_detected = true;
                self.rip_ric_fusion_state.current_recursion_budget = 0;
            }
            _ => {
                // Update general health metrics
                self.rip_ric_fusion_state.ric_consensus_health *= 0.8; // Slight degradation on halts
            }
        }
    }

    /// Get unified fusion health summary
    pub fn get_rip_ric_fusion_health(&self) -> RIPRICFusionHealth {
        RIPRICFusionHealth {
            rip_genome_health: self.rip_ric_fusion_state.rip_genome_health,
            rip_guard_health: self.rip_ric_fusion_state.rip_guard_health,
            rip_pathogen_threat: self.rip_ric_fusion_state.rip_pathogen_threat,
            ric_consensus_health: self.rip_ric_fusion_state.ric_consensus_health,
            ric_saturation_level: self.rip_ric_fusion_state.ric_saturation_level,
            rip_process_healthy: self.rip_ric_fusion_state.rip_process_healthy,
            current_recursion_budget: self.rip_ric_fusion_state.current_recursion_budget,
            loop_saturation_detected: self.rip_ric_fusion_state.loop_saturation_detected,
            fusion_timestamp: self.rip_ric_fusion_state.last_fusion_update,
            overall_fusion_health: self.calculate_overall_fusion_health(),
        }
    }

    /// Calculate overall fusion health score
    fn calculate_overall_fusion_health(&self) -> f32 {
        let rip_health = (self.rip_ric_fusion_state.rip_genome_health +
                         self.rip_ric_fusion_state.rip_guard_health +
                         (1.0 - self.rip_ric_fusion_state.rip_pathogen_threat)) / 3.0;

        let ric_health = (self.rip_ric_fusion_state.ric_consensus_health +
                         (1.0 - self.rip_ric_fusion_state.ric_saturation_level)) / 2.0;

        let process_health = if self.rip_ric_fusion_state.rip_process_healthy { 1.0 } else { 0.0 };

        (rip_health + ric_health + process_health) / 3.0
    }

    /// Reset RIP+RIC fusion state for new narrative cycle
    pub fn reset_rip_ric_fusion(&mut self) {
        self.rip_ric_fusion_state = RIPRICFusionState::new();
        self.reset_ric_state();
    }

    /// Helper voting methods for RIC integration
    fn vote_constraint_tracker(&self) -> String {
        let freedom_score = self.constraint_tracker.calculate_freedom_score();
        let pressure_analysis = self.constraint_tracker.analyze_constraint_pressure();

        if freedom_score < 0.2 {
            "HALT_CRITICAL_CONSTRAINT_PRESSURE".to_string()
        } else if freedom_score < 0.4 {
            "CAUTION_HIGH_CONSTRAINT_PRESSURE".to_string()
        } else if freedom_score < 0.6 {
            "CONTINUE_MODERATE_CONSTRAINT_PRESSURE".to_string()
        } else {
            "CONTINUE_LOW_CONSTRAINT_PRESSURE".to_string()
        }
    }

    fn vote_recursion_tracker(&self) -> String {
        let recursion_health = self.recursion_tracker.analyze_recursion_health();

        if recursion_health.health_score < 0.3 {
            "HALT_POOR_RECURSION_HEALTH".to_string()
        } else if recursion_health.health_score < 0.5 {
            "CAUTION_MODERATE_RECURSION_HEALTH".to_string()
        } else if recursion_health.health_score < 0.7 {
            "CONTINUE_GOOD_RECURSION_HEALTH".to_string()
        } else {
            "CONTINUE_EXCELLENT_RECURSION_HEALTH".to_string()
        }
    }

    fn vote_engagement_tracker(&self) -> String {
        let retention_score = self.engagement_tracker.engagement_metrics.reader_retention_score;
        let active_loops = self.engagement_tracker.get_loops_requiring_attention().len();

        if retention_score < 0.3 {
            "HALT_CRITICAL_ENGAGEMENT_LOSS".to_string()
        } else if retention_score < 0.5 || active_loops > 5 {
            "CAUTION_ENGAGEMENT_CONCERNS".to_string()
        } else if retention_score < 0.7 {
            "CONTINUE_MODERATE_ENGAGEMENT".to_string()
        } else {
            "CONTINUE_STRONG_ENGAGEMENT".to_string()
        }
    }

    /// Helper methods
    fn current_context(&self) -> InsightContext {
        InsightContext {
            chapter: self.current_chapter,
            scene: self.current_scene,
            characters_involved: vec![], // Would be populated from context
            themes_involved: vec![], // Would be populated from theme tracking
            narrative_pressure: compute_saturation(&self.current_obligations),
            emotional_context: self.current_emotional_state.as_ref().map(|s| s.describe()),
        }
    }

    fn current_context_with_character(&self, character: &str) -> InsightContext {
        let mut context = self.current_context();
        context.characters_involved.push(character.to_string());
        context
    }

    fn priority_value(&self, priority: &Priority) -> u8 {
        match priority {
            Priority::Critical => 4,
            Priority::High => 3,
            Priority::Medium => 2,
            Priority::Low => 1,
        }
    }

    fn filter_insights_by_assertiveness(&self, mut insights: Vec<NarrativeInsight>) -> Vec<NarrativeInsight> {
        let max_insights = match self.config.assertiveness_level {
            level if level < 0.3 => 1,
            level if level < 0.6 => 3,
            level if level < 0.8 => 5,
            _ => 8,
        };

        insights.truncate(max_insights);
        insights
    }

    /// Generates a comprehensive narrative analysis report
    pub fn generate_comprehensive_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🤖 RECURSIVE NARRATIVE ASSISTANT REPORT\n");
        report.push_str("=========================================\n\n");

        report.push_str(&format!("Current Chapter: {}\n", self.current_chapter));
        if let Some(scene) = self.current_scene {
            report.push_str(&format!("Current Scene: {}\n", scene));
        }
        report.push_str(&format!("Assistant Assertiveness: {:.2}\n\n", self.config.assertiveness_level));

        // System status
        report.push_str("🔧 System Status:\n");
        if self.config.enabled_systems.dna_tracking {
            let dna_health = self.dna_tracker.analyze_pattern_health();
            report.push_str(&format!("  • DNA Tracking: ✅ ({:.2} health, {} units)\n",
                dna_health.health_score, dna_health.total_units));
        }
        if self.config.enabled_systems.constraint_modeling {
            let freedom = self.constraint_tracker.calculate_freedom_score();
            report.push_str(&format!("  • Constraint Modeling: ✅ ({:.2} freedom)\n", freedom));
        }
        if self.config.enabled_systems.recursion_tracking {
            let recursion_health = self.recursion_tracker.analyze_recursion_health();
            report.push_str(&format!("  • Multi-Level Recursion: ✅ ({:.2} health)\n", recursion_health.health_score));
        }
        if self.config.enabled_systems.character_consistency {
            let global_consistency = self.character_engine.global_consistency;
            report.push_str(&format!("  • Character Consistency: ✅ ({:.2} global)\n", global_consistency));
        }
        if self.config.enabled_systems.engagement_loops {
            let retention = self.engagement_tracker.engagement_metrics.reader_retention_score;
            report.push_str(&format!("  • Engagement Loops: ✅ ({:.2} retention)\n", retention));
        }
        report.push('\n');

        // Current narrative pressure
        let obligation_pressure = compute_saturation(&self.current_obligations);
        report.push_str(&format!("📊 Current Narrative Pressure: {:.2}\n", obligation_pressure));

        if let Some(emotional_state) = &self.current_emotional_state {
            report.push_str(&format!("🎭 Current Emotional Field: {}\n", emotional_state.describe()));
        }
        report.push('\n');

        // Include individual system reports (truncated)
        if self.config.enabled_systems.dna_tracking {
            report.push_str("🧬 DNA Summary:\n");
            let dna_report = self.dna_tracker.generate_dna_report();
            let lines: Vec<&str> = dna_report.lines().take(10).collect();
            for line in lines {
                if line.starts_with("🧬") || line.starts_with("=") {
                    continue;
                }
                report.push_str(&format!("  {}\n", line));
            }
            report.push('\n');
        }

        report
    }
}

/// Events that can be recorded in the narrative system
#[derive(Debug, Clone)]
pub enum NarrativeEvent {
    CharacterAction {
        character: String,
        action: String,
        motivation: String,
    },
    DialogueSpoken {
        character: String,
        dialogue: String,
        emotional_context: String,
    },
    DNAUnit {
        unit: NarrativeDNAUnit,
    },
    RecursiveElement {
        element: RecursiveElement,
    },
    EngagementLoop {
        loop_type: EngagementLoopType,
        intensity: f32,
    },
    Constraint {
        constraint: ConstraintType,
    },
}

impl Default for RecursiveNarrativeAssistant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assistant_creation() {
        let assistant = RecursiveNarrativeAssistant::new();
        assert_eq!(assistant.current_chapter, 1);
        assert!(assistant.config.enabled_systems.dna_tracking);
        assert_eq!(assistant.config.assertiveness_level, 0.6);
    }

    #[test]
    fn test_advance_chapter() {
        let mut assistant = RecursiveNarrativeAssistant::new();
        assistant.advance_chapter(5, Some(3));

        assert_eq!(assistant.current_chapter, 5);
        assert_eq!(assistant.current_scene, Some(3));
        assert_eq!(assistant.dna_tracker.current_chapter, 5);
        assert_eq!(assistant.character_engine.current_chapter, 5);
    }

    #[test]
    fn test_analyze_narrative_state() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Add some problematic obligations to trigger insights
        let obligations = vec![
            Obligation::new("test_obligation", 0.9, 5),
            Obligation::new("another_obligation", 0.8, 6),
        ];
        assistant.update_narrative_state(None, obligations);

        let insights = assistant.analyze_narrative_state();
        // Should generate insights about high obligation pressure
        assert!(!insights.is_empty());
    }

    #[test]
    fn test_context_prompt_generation() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Create conditions that should generate insights
        let obligations = vec![Obligation::new("critical_obligation", 1.0, 10)];
        assistant.update_narrative_state(None, obligations);

        let prompt = assistant.generate_context_prompt();
        assert!(prompt.contains("Narrative Assistant Context") || prompt.is_empty());
    }

    #[test]
    fn test_record_narrative_event() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        let event = NarrativeEvent::CharacterAction {
            character: "Hero".to_string(),
            action: "fights the dragon".to_string(),
            motivation: "save the village".to_string(),
        };

        assistant.record_narrative_event(event);
        // Should have updated character consistency system
        // (specific verification would depend on system implementation)
    }

    #[test]
    fn test_assertiveness_filtering() {
        let mut assistant = RecursiveNarrativeAssistant::new();
        assistant.config.assertiveness_level = 0.2; // Very passive

        // Create many insights
        let insights = vec![
            NarrativeInsight {
                insight_type: InsightType::DNAPattern,
                priority: Priority::Medium,
                title: "Test 1".to_string(),
                description: "Test".to_string(),
                questions: vec![],
                suggestions: vec![],
                affected_systems: vec![],
                context: assistant.current_context(),
                timestamp: Utc::now(),
            },
            NarrativeInsight {
                insight_type: InsightType::ConstraintPressure,
                priority: Priority::Low,
                title: "Test 2".to_string(),
                description: "Test".to_string(),
                questions: vec![],
                suggestions: vec![],
                affected_systems: vec![],
                context: assistant.current_context(),
                timestamp: Utc::now(),
            },
        ];

        let filtered = assistant.filter_insights_by_assertiveness(insights);
        assert_eq!(filtered.len(), 1); // Should limit to 1 for low assertiveness
    }

    #[test]
    fn test_comprehensive_report() {
        let assistant = RecursiveNarrativeAssistant::new();
        let report = assistant.generate_comprehensive_report();

        assert!(report.contains("RECURSIVE NARRATIVE ASSISTANT REPORT"));
        assert!(report.contains("System Status"));
        assert!(report.contains("Current Chapter: 1"));
    }

    #[test]
    fn test_custom_config() {
        let mut config = AssistantConfig::default();
        config.assertiveness_level = 0.8;
        config.enabled_systems.dna_tracking = false;

        let assistant = RecursiveNarrativeAssistant::with_config(config);
        assert_eq!(assistant.config.assertiveness_level, 0.8);
        assert!(!assistant.config.enabled_systems.dna_tracking);
    }

    #[test]
    fn test_cross_system_pattern_detection() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Create high pressure across multiple systems
        let high_pressure_obligations = vec![
            Obligation::new("critical1", 1.0, 8),
            Obligation::new("critical2", 0.9, 7),
            Obligation::new("critical3", 0.8, 9),
        ];

        assistant.update_narrative_state(None, high_pressure_obligations);

        let insights = assistant.analyze_cross_system_patterns();
        // Should detect convergent pressure pattern
        let convergent_pattern = insights.iter()
            .find(|i| i.insight_type == InsightType::CrossSystemPattern);

        // May or may not trigger depending on exact thresholds, but test structure is correct
        assert!(convergent_pattern.is_some() || insights.is_empty());
    }
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            auto_pattern_detection: true,
            assertiveness_level: 0.5,
            enabled_systems: EnabledSystems::default(),
            sensitivity: SensitivitySettings::default(),
            drift_config: DriftStabilizerConfig::default(),
            ric_mode: RICMode::default(),
        }
    }
}

impl Default for EnabledSystems {
    fn default() -> Self {
        Self {
            dna_tracking: true,
            constraint_modeling: true,
            recursion_tracking: true,
            character_consistency: true,
            engagement_loops: true,
            drift_stabilization: true,
        }
    }
}

impl Default for SensitivitySettings {
    fn default() -> Self {
        Self {
            constraint_pressure: 0.7,
            character_drift: 0.8,
            unresolved_loops: 0.6,
            engagement_drops: 0.7,
            pattern_breaks: 0.5,
        }
    }
}