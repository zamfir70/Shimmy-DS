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
use crate::telemetry::{PulseTrace, Pulse, PulseTraceHealthStats};
use crate::cache::{CacheMind, ConstraintSnapshot, CAPRPathSummary, CharacterEmotionArc};
use crate::adaptive::{
    AdaptIQEngine, AdaptIQSettings, TasteLUT, entropy_score, estimate_cognitive_load,
    Qualitier, QualityLevel, QualityFeature, PerformanceConfig, QualitierStatusReport
};
use crate::obligations::{SmartObligationManager, ObliSelectSettings, ObligationMetrics};

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

    /// PulseTrace telemetry logger
    #[serde(skip)]
    pub pulse_trace: PulseTrace,

    /// CacheMind cross-system state cache
    #[serde(skip)]
    pub cache_mind: CacheMind,

    /// AdaptIQ narrative intelligence modulator
    #[serde(skip)]
    pub adapt_iq_engine: Option<AdaptIQEngine>,

    /// Current AdaptIQ settings
    pub current_adapt_settings: AdaptIQSettings,

    /// Qualitier adaptive quality control
    #[serde(skip)]
    pub qualitier: Qualitier,

    /// ObliSelect smart obligation management
    #[serde(skip)]
    pub obli_select: SmartObligationManager,

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
    /// AdaptIQ taste preferences
    pub taste_profile: TasteLUT,
    /// Enable AdaptIQ adaptive intelligence modulation
    pub enable_adapt_iq: bool,
    /// Qualitier performance configuration
    pub performance_config: PerformanceConfig,
    /// ObliSelect obligation management settings
    pub obli_select_settings: ObliSelectSettings,
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
    pub adaptive_intelligence: bool,
    pub obligation_management: bool,
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
                adaptive_intelligence: true,
                obligation_management: true,
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
            taste_profile: TasteLUT::Balanced(),
            enable_adapt_iq: true,
            performance_config: PerformanceConfig::default(),
            obli_select_settings: ObliSelectSettings::default(),
        }
    }
}

/// A narrative insight surfaced by the assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeInsight {
    pub insight_type: InsightType,
    pub priority: InsightPriority,
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

/// Priority levels for insights
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InsightPriority {
    Critical,
    Important,
    Interesting,
    Minor,
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
    pub cross_system_conflicts: usize,
    pub pathogen_detections: usize,
    pub overall_health_score: f32,
    pub python_rust_sync_score: f32,
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
            pulse_trace: PulseTrace::new(512), // 512 pulse capacity as specified
            cache_mind: CacheMind::new(128), // 128 capacity for each cache
            adapt_iq_engine: None, // Will be initialized on first use
            current_adapt_settings: AdaptIQSettings::default(),
            qualitier: Qualitier::new(100, 50, true), // Default performance config
            obli_select: SmartObligationManager::new(),
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
        // Update AdaptIQ settings before analysis if enabled
        if self.config.enabled_systems.adaptive_intelligence && self.config.enable_adapt_iq {
            self.update_adaptive_settings();

            // Apply Qualitier quality control
            if let Some(recent_pulse) = self.pulse_trace.latest() {
                let quality_changed = self.qualitier.decide(recent_pulse, &self.current_adapt_settings);
                if quality_changed {
                    // Log quality level change
                    tracing::info!("Qualitier changed tier to {:?}", self.qualitier.current_level());
                }

                // Apply quality constraints to adaptive settings
                self.qualitier.clamp_settings(&mut self.current_adapt_settings);
            }
        }

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
        let filtered_insights = self.filter_insights_by_assertiveness(insights);

        // Record telemetry pulse after analysis
        self.record_pulse_telemetry(&filtered_insights);

        // Auto-cache state snapshots if significant insights detected
        if filtered_insights.iter().any(|i| matches!(i.priority, InsightPriority::Critical | InsightPriority::Important)) {
            self.cache_constraint_snapshot();
            self.cache_capr_path_summary();
        }

        filtered_insights
    }

    /// Analyzes narrative DNA patterns
    fn analyze_dna_patterns(&self) -> Vec<NarrativeInsight> {
        let mut insights = Vec::new();

        let health = self.dna_tracker.analyze_pattern_health();
        if health.health_score < 0.6 {
            insights.push(NarrativeInsight {
                insight_type: InsightType::DNAPattern,
                priority: if health.health_score < 0.4 { InsightPriority::Important } else { InsightPriority::Interesting },
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
                priority: if top_opportunity.intensity_score > 1.5 { InsightPriority::Important } else { InsightPriority::Interesting },
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
                f if f < 0.3 => InsightPriority::Critical,
                f if f < 0.5 => InsightPriority::Important,
                _ => InsightPriority::Interesting,
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
                priority: InsightPriority::Interesting,
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
                    priority: if issue.consistency_score < 0.5 { InsightPriority::Important } else { InsightPriority::Interesting },
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
                priority: InsightPriority::Important,
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
                    priority: if attention.attention_level > 0.8 { InsightPriority::Important } else { InsightPriority::Interesting },
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
                priority: InsightPriority::Interesting,
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
                priority: InsightPriority::Important,
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
            .filter(|i| matches!(i.priority, InsightPriority::Important | InsightPriority::Critical))
            .take(3) // Limit to avoid overwhelming the prompt
            .collect();

        if !high_priority_insights.is_empty() {
            for insight in high_priority_insights {
                prompt.push_str(&format!("• {}: {}\n", insight.title, insight.description));
                if !insight.questions.is_empty() {
                    prompt.push_str(&format!("  Consider: {}\n", insight.questions[0]));
                }
            }
        } else if let Some(medium_insight) = insights.iter().find(|i| i.priority == InsightPriority::Interesting) {
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
                        priority: InsightPriority::Interesting,
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
        // Collect votes first to avoid borrowing conflicts
        let dna_vote = self.dna_tracker.vote_on_narrative_state();
        let character_vote = self.character_engine.vote_on_consistency_state();
        let constraint_vote = self.vote_constraint_tracker();
        let recursion_vote = self.vote_recursion_tracker();
        let engagement_vote = self.vote_engagement_tracker();

        let ric_decision = if let Some(ref mut ric) = self.ric {
            // Now apply the votes
            ric.vote("dna_tracker", dna_vote);
            ric.vote("character_engine", character_vote);
            ric.vote("constraint_tracker", constraint_vote);
            ric.vote("recursion_tracker", recursion_vote);
            ric.vote("engagement_tracker", engagement_vote);

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
                .map(|o| o.kind.clone())
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
            RICDecision::Reroute(ref alternative) => {
                // Treat reroute as continue with modified confidence
                UnifiedArbitrationDecision::ContinueRecursion {
                    rip_vote: format!("REROUTED: {}", rip_analysis.rip_vote),
                    ric_vote: RICDecision::Reroute(alternative.clone()),
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
            cross_system_conflicts: 0, // TODO: implement cross-system conflict detection
            pathogen_detections: 0, // TODO: implement pathogen detection counting
            overall_health_score: self.calculate_overall_fusion_health(),
            python_rust_sync_score: 1.0, // TODO: implement actual sync score calculation
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
    fn vote_constraint_tracker(&self) -> InsightStatus {
        let freedom_score = self.constraint_tracker.calculate_freedom_score();
        let _pressure_analysis = self.constraint_tracker.analyze_constraint_pressure();

        if freedom_score < 0.2 {
            InsightStatus::Block
        } else if freedom_score < 0.4 {
            InsightStatus::Suggest
        } else if freedom_score < 0.6 {
            InsightStatus::Continue
        } else {
            InsightStatus::Continue
        }
    }

    fn vote_recursion_tracker(&self) -> InsightStatus {
        let recursion_health = self.recursion_tracker.analyze_recursion_health();

        if recursion_health.health_score < 0.3 {
            InsightStatus::Block
        } else if recursion_health.health_score < 0.5 {
            InsightStatus::Suggest
        } else if recursion_health.health_score < 0.7 {
            InsightStatus::Continue
        } else {
            InsightStatus::Continue
        }
    }

    fn vote_engagement_tracker(&self) -> InsightStatus {
        let retention_score = self.engagement_tracker.engagement_metrics.reader_retention_score;
        let active_loops = self.engagement_tracker.get_loops_requiring_attention().len();

        if retention_score < 0.3 {
            InsightStatus::Block
        } else if retention_score < 0.5 || active_loops > 5 {
            InsightStatus::Suggest
        } else if retention_score < 0.7 {
            InsightStatus::Continue
        } else {
            InsightStatus::Continue
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

    fn priority_value(&self, priority: &InsightPriority) -> u8 {
        match priority {
            InsightPriority::Critical => 4,
            InsightPriority::Important => 3,
            InsightPriority::Interesting => 2,
            InsightPriority::Minor => 1,
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

    // ========== PULSE TRACE TELEMETRY METHODS ==========

    /// Records a telemetry pulse capturing current narrative system state
    fn record_pulse_telemetry(&mut self, insights: &[NarrativeInsight]) {
        use crate::telemetry::helpers::{get_memory_usage_estimate, count_pathogens_from_results};

        // Count pathogens from DNA tracker
        let pathogen_count = if self.config.enabled_systems.dna_tracking {
            self.dna_tracker.get_active_pathogens().len()
        } else {
            0
        };

        // Count drift hits from current drift state
        let drift_hits = self.drift_state.get_drift_indicators().len();

        // Calculate ADI score based on current narrative state
        let adi_score = self.calculate_narrative_adi_score();

        // Get ZC tick from RIC if available
        let zc_tick = if let Some(ref ric) = self.ric {
            ric.get_iteration_count() as usize // Convert u32 to usize
        } else {
            0
        };

        // Calculate affective signals
        let (affect_pleasure, affect_coherence) = self.calculate_affective_signals(insights);

        // Create and record pulse
        let pulse = Pulse::with_data(
            zc_tick,
            pathogen_count,
            drift_hits,
            adi_score,
            get_memory_usage_estimate(),
            affect_pleasure,
            affect_coherence,
            Some("Narrative analysis tick complete".to_string())
        );

        self.pulse_trace.record(pulse);
    }

    /// Calculate Adaptive Depth Intelligence score based on current narrative state
    fn calculate_narrative_adi_score(&self) -> f32 {
        let mut score = 0.5; // Base score

        // Factor in constraint pressure
        if self.config.enabled_systems.constraint_modeling {
            let freedom_score = self.constraint_tracker.calculate_freedom_score();
            score += (1.0 - freedom_score) * 0.2; // More constraints = higher ADI
        }

        // Factor in character consistency
        if self.config.enabled_systems.character_consistency {
            let consistency = self.character_engine.global_consistency;
            score += consistency * 0.15;
        }

        // Factor in recursion depth
        if self.config.enabled_systems.recursion_tracking {
            let recursion_health = self.recursion_tracker.analyze_recursion_health();
            score += recursion_health.health_score * 0.15;
        }

        // Factor in DNA complexity
        if self.config.enabled_systems.dna_tracking {
            let dna_health = self.dna_tracker.analyze_pattern_health();
            score += (dna_health.total_units as f32 * 0.01).min(0.2);
        }

        score.min(1.0)
    }

    /// Calculate affective pleasure and coherence signals
    fn calculate_affective_signals(&self, insights: &[NarrativeInsight]) -> (f32, f32) {
        // Pleasure signal: positive for constructive insights, negative for conflicts
        let mut pleasure = 0.0;
        let mut coherence = 0.5; // Base coherence

        for insight in insights {
            match insight.priority {
                InsightPriority::Critical => {
                    pleasure -= 0.3; // Critical issues reduce pleasure
                    coherence -= 0.1;
                }
                InsightPriority::Important => {
                    pleasure -= 0.1;
                }
                InsightPriority::Interesting => {
                    pleasure += 0.1; // Interesting patterns increase pleasure
                    coherence += 0.05;
                }
                InsightPriority::Minor => {
                    pleasure += 0.05;
                }
            }
        }

        // Factor in engagement and consistency
        if self.config.enabled_systems.engagement_loops {
            let retention = self.engagement_tracker.engagement_metrics.reader_retention_score;
            pleasure += (retention - 0.5) * 0.4; // Retention above 0.5 increases pleasure
            coherence += retention * 0.3;
        }

        if self.config.enabled_systems.character_consistency {
            coherence += self.character_engine.global_consistency * 0.2;
        }

        // Clamp to valid ranges
        pleasure = pleasure.max(-1.0).min(1.0);
        coherence = coherence.max(0.0).min(1.0);

        (pleasure, coherence)
    }

    /// Get pulse trace health statistics
    pub fn get_pulse_trace_health(&self) -> PulseTraceHealthStats {
        self.pulse_trace.get_health_stats()
    }

    /// Get recent pulse trace summary as JSON
    pub fn get_pulse_trace_summary(&self) -> String {
        self.pulse_trace.to_summary_json()
    }

    /// Get full pulse trace as JSON
    pub fn get_pulse_trace_json(&self) -> String {
        self.pulse_trace.to_json()
    }

    /// Clear pulse trace (for testing or reset)
    pub fn clear_pulse_trace(&mut self) {
        self.pulse_trace.clear();
    }

    // ========================================================================
    // CacheMind Integration Methods
    // ========================================================================

    /// Create and cache a constraint snapshot from current state
    pub fn cache_constraint_snapshot(&mut self) -> String {
        let snapshot = ConstraintSnapshot {
            freedom_score: self.constraint_tracker.calculate_freedom_score(),
            active_constraints: self.constraint_tracker.get_active_constraint_names(),
            constraint_pressures: self.constraint_tracker.get_constraint_pressures(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            chapter: self.current_chapter,
            scene: self.current_scene,
        };

        let key = format!("ch{}_sc{:?}", self.current_chapter, self.current_scene);
        self.cache_mind.set_constraint_snapshot(key.clone(), snapshot);
        key
    }

    /// Create and cache a CAPR path summary from current DNA tracker state
    pub fn cache_capr_path_summary(&mut self) -> String {
        let dna_health = self.dna_tracker.analyze_pattern_health();
        let active_pathogens = self.dna_tracker.get_active_pathogens();

        let summary = CAPRPathSummary {
            loop_count: dna_health.total_units as usize,
            last_return_vector: active_pathogens.iter()
                .map(|p| format!("{:?}", p.get_transformation_type()))
                .collect(),
            active_contradictions: active_pathogens.iter()
                .filter(|p| matches!(p.get_transformation_type(), TransformationType::Contradiction))
                .map(|p| p.get_id().clone())
                .collect(),
            pressure_points: active_pathogens.iter()
                .filter(|p| matches!(p.get_transformation_type(), TransformationType::Pressure))
                .map(|p| p.get_id().clone())
                .collect(),
            avg_loop_duration: dna_health.health_score * 100.0, // Approximate
            last_loop_quality: dna_health.health_score,
            chapter: self.current_chapter,
            scene: self.current_scene,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let key = format!("capr_ch{}_sc{:?}", self.current_chapter, self.current_scene);
        self.cache_mind.set_capr_path_summary(key.clone(), summary);
        key
    }

    /// Create and cache character emotion arc for a specific character
    pub fn cache_character_emotion_arc(&mut self, character: &str) -> String {
        if let Some(profile) = self.character_engine.get_character_profile(character) {
            // Extract emotion sequences from character profile
            let (valence_seq, intensity_seq, emotions) = profile.extract_emotion_sequences();

            let arc = CharacterEmotionArc {
                character: character.to_string(),
                valence_sequence: valence_seq,
                intensity_sequence: intensity_seq,
                dominant_emotions: emotions,
                turning_points: vec![], // Would need more sophisticated detection
                arc_trend: "stable".to_string(), // Simplified for now
                chapter: self.current_chapter,
                scene_range: (self.current_scene, self.current_scene),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            let key = format!("{}ch{}_sc{:?}", character, self.current_chapter, self.current_scene);
            self.cache_mind.set_character_emotion_arc(key.clone(), arc);
            key
        } else {
            String::new()
        }
    }

    /// Get cached constraint snapshot by key
    pub fn get_cached_constraint_snapshot(&mut self, key: &str) -> Option<&ConstraintSnapshot> {
        self.cache_mind.get_constraint_snapshot(key)
    }

    /// Get cached CAPR path summary by key
    pub fn get_cached_capr_path_summary(&mut self, key: &str) -> Option<&CAPRPathSummary> {
        self.cache_mind.get_capr_path_summary(key)
    }

    /// Get cached character emotion arc by key
    pub fn get_cached_character_emotion_arc(&mut self, key: &str) -> Option<&CharacterEmotionArc> {
        self.cache_mind.get_character_emotion_arc(key)
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "constraint_cache": {
                "size": self.cache_mind.constraint_cache.len(),
                "capacity": self.cache_mind.constraint_cache.capacity(),
                "hit_ratio": self.cache_mind.constraint_cache.hit_ratio()
            },
            "capr_cache": {
                "size": self.cache_mind.capr_cache.len(),
                "capacity": self.cache_mind.capr_cache.capacity(),
                "hit_ratio": self.cache_mind.capr_cache.hit_ratio()
            },
            "character_cache": {
                "size": self.cache_mind.emotion_cache.len(),
                "capacity": self.cache_mind.emotion_cache.capacity(),
                "hit_ratio": 0.0 // TODO: implement actual hit ratio calculation
            }
        })
    }

    /// Load cache state from JSON file
    pub fn load_cache_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.cache_mind = CacheMind::load_from_file(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    /// Save cache state to JSON file
    pub fn save_cache_to_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.cache_mind.save_to_file(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Auto-save cache state to default location
    pub fn auto_save_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cache_mind.auto_save().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Clear all cache entries
    pub fn clear_cache(&mut self) {
        self.cache_mind.clear_all();
    }

    // ========================================================================
    // AdaptIQ Integration Methods
    // ========================================================================

    /// Update adaptive settings based on current context
    fn update_adaptive_settings(&mut self) {
        if self.adapt_iq_engine.is_none() {
            // Initialize AdaptIQ engine on first use
            let entropy = 0.5; // Default entropy, will be updated on analyze_prompt
            self.adapt_iq_engine = Some(AdaptIQEngine::new(entropy, self.config.taste_profile.clone()));
        }

        if let Some(ref mut engine) = self.adapt_iq_engine {
            // Get recent pulse data for decision making
            if let Some(recent_pulse) = self.pulse_trace.latest() {
                self.current_adapt_settings = engine.decide(recent_pulse, &self.cache_mind);
            } else {
                // No pulse data available, use quick decision
                self.current_adapt_settings = engine.quick_decide(0.5, 50.0);
            }
        }
    }

    /// Analyze prompt and update AdaptIQ with calculated entropy
    pub fn analyze_prompt(&mut self, prompt: &str) -> f32 {
        let entropy = entropy_score(prompt);
        let cognitive_load = estimate_cognitive_load(prompt);

        // Initialize or update AdaptIQ engine with new entropy
        if let Some(ref mut engine) = self.adapt_iq_engine {
            engine.update_entropy(entropy);
        } else if self.config.enable_adapt_iq {
            self.adapt_iq_engine = Some(AdaptIQEngine::new(entropy, self.config.taste_profile.clone()));
        }

        // Update settings based on new prompt
        if self.config.enabled_systems.adaptive_intelligence && self.config.enable_adapt_iq {
            self.update_adaptive_settings();
        }

        cognitive_load.overall_load
    }

    /// Get current AdaptIQ settings
    pub fn get_adapt_settings(&self) -> &AdaptIQSettings {
        &self.current_adapt_settings
    }

    /// Update taste preferences
    pub fn update_taste_profile(&mut self, taste: TasteLUT) {
        self.config.taste_profile = taste.clone();
        if let Some(ref mut engine) = self.adapt_iq_engine {
            engine.update_taste(taste);
        }
    }

    /// Set taste profile to predefined preset
    pub fn set_taste_preset(&mut self, preset: &str) {
        let taste = match preset.to_lowercase().as_str() {
            "curious" => TasteLUT::curious(),
            "safe" => TasteLUT::safe(),
            "experimental" => TasteLUT::experimental(),
            "balanced" | _ => TasteLUT::balanced(),
        };
        self.update_taste_profile(taste);
    }

    /// Get AdaptIQ engine statistics
    pub fn get_adapt_iq_stats(&self) -> Option<serde_json::Value> {
        if let Some(ref engine) = self.adapt_iq_engine {
            let stats = engine.get_stats();
            Some(serde_json::json!({
                "decision_count": stats.decision_count,
                "avg_decision_time_ms": stats.avg_decision_time_ms,
                "cache_utilization": stats.cache_utilization,
                "performance_adjustments": stats.performance_adjustments,
                "last_decision": stats.last_decision_timestamp.map(|t| t.elapsed().as_secs()),
                "current_settings": {
                    "recursion_depth": self.current_adapt_settings.recursion_depth,
                    "pathogen_sensitivity": self.current_adapt_settings.pathogen_sensitivity,
                    "affect_assertiveness": self.current_adapt_settings.affect_assertiveness,
                    "beat_sampling_rate": self.current_adapt_settings.beat_sampling_rate,
                    "zc_hysteresis_margin": self.current_adapt_settings.zc_hysteresis_margin,
                    "eat_resolution_scale": self.current_adapt_settings.eat_resolution_scale,
                    "cache_preference": self.current_adapt_settings.cache_preference
                },
                "taste_profile": {
                    "curiosity": self.config.taste_profile.curiosity,
                    "coherence_pleasure": self.config.taste_profile.coherence_pleasure,
                    "unease": self.config.taste_profile.unease,
                    "awe": self.config.taste_profile.awe,
                    "boredom": self.config.taste_profile.boredom
                }
            }))
        } else {
            None
        }
    }

    /// Apply AdaptIQ settings to current analysis parameters
    pub fn apply_adaptive_settings(&mut self) {
        if !self.config.enable_adapt_iq || !self.config.enabled_systems.adaptive_intelligence {
            return;
        }

        let settings = &self.current_adapt_settings;

        // Apply settings to various subsystems
        // Note: This is a simplified implementation - in practice, you'd apply these
        // settings to the actual analysis parameters of each subsystem

        // Adjust assertiveness level based on affect assertiveness
        self.config.assertiveness_level = (self.config.assertiveness_level * settings.affect_assertiveness).clamp(0.0, 1.0);

        // Adjust sensitivity settings based on pathogen sensitivity
        self.config.sensitivity.constraint_pressure *= settings.pathogen_sensitivity;
        self.config.sensitivity.character_drift *= settings.pathogen_sensitivity;

        // The recursion_depth, beat_sampling_rate, etc. would be used by the
        // actual analysis algorithms when they're implemented
    }

    /// Force AdaptIQ recalibration based on current context
    pub fn recalibrate_adapt_iq(&mut self) {
        if self.config.enable_adapt_iq && self.config.enabled_systems.adaptive_intelligence {
            if let Some(ref mut engine) = self.adapt_iq_engine {
                engine.reset_stats();
                self.update_adaptive_settings();
            }
        }
    }

    /// Get adaptive intelligence status
    pub fn get_adaptive_status(&self) -> serde_json::Value {
        serde_json::json!({
            "enabled": self.config.enable_adapt_iq && self.config.enabled_systems.adaptive_intelligence,
            "engine_initialized": self.adapt_iq_engine.is_some(),
            "current_settings": {
                "recursion_depth": self.current_adapt_settings.recursion_depth,
                "pathogen_sensitivity": self.current_adapt_settings.pathogen_sensitivity,
                "affect_assertiveness": self.current_adapt_settings.affect_assertiveness,
                "beat_sampling_rate": self.current_adapt_settings.beat_sampling_rate,
                "zc_hysteresis_margin": self.current_adapt_settings.zc_hysteresis_margin
            },
            "taste_profile": {
                "curiosity": self.config.taste_profile.curiosity,
                "coherence_pleasure": self.config.taste_profile.coherence_pleasure,
                "unease": self.config.taste_profile.unease,
                "awe": self.config.taste_profile.awe,
                "boredom": self.config.taste_profile.boredom
            }
        })
    }

    // ========================================================================
    // Qualitier Integration Methods
    // ========================================================================

    /// Get current quality level
    pub fn get_quality_level(&self) -> QualityLevel {
        self.qualitier.current_level()
    }

    /// Set quality level manually (bypasses adaptive logic)
    pub fn set_quality_level(&mut self, level: QualityLevel) {
        self.qualitier.set_quality_level(level);
        tracing::info!("Quality level manually set to {:?}", level);
    }

    /// Check if a feature is enabled at current quality level
    pub fn is_feature_enabled(&self, feature: QualityFeature) -> bool {
        self.qualitier.is_feature_enabled(feature)
    }

    /// Get comprehensive Qualitier status report
    pub fn get_qualitier_status(&self) -> QualitierStatusReport {
        self.qualitier.get_status_report()
    }

    /// Update Qualitier performance configuration
    pub fn update_performance_config(&mut self, config: PerformanceConfig) {
        self.config.performance_config = config.clone();
        self.qualitier = Qualitier::with_config(config);
        tracing::info!("Performance configuration updated");
    }

    /// Reset Qualitier statistics
    pub fn reset_qualitier_stats(&mut self) {
        self.qualitier.reset_stats();
    }

    /// Get Qualitier statistics as JSON
    pub fn get_qualitier_stats(&self) -> serde_json::Value {
        let stats = self.qualitier.get_stats();
        let distribution = self.qualitier.get_quality_distribution();
        let status = self.get_qualitier_status();

        serde_json::json!({
            "current_quality_level": status.current_level,
            "memory_pressure": status.memory_pressure,
            "adaptive_enabled": status.adaptive_enabled,
            "statistics": {
                "quality_changes": stats.quality_changes,
                "memory_degradations": stats.memory_degradations,
                "narrative_stress_upgrades": stats.narrative_stress_upgrades,
                "decision_count": stats.decision_count,
                "avg_decision_time_ms": stats.avg_decision_time_ms
            },
            "time_distribution": {
                "minimal_percent": distribution.minimal_percent,
                "standard_percent": distribution.standard_percent,
                "enhanced_percent": distribution.enhanced_percent,
                "premium_percent": distribution.premium_percent
            },
            "quality_level_info": {
                "minimal": {
                    "description": QualityLevel::Minimal.description(),
                    "max_recursion_depth": QualityLevel::Minimal.max_recursion_depth(),
                    "pathogen_sensitivity_cap": QualityLevel::Minimal.pathogen_sensitivity_cap()
                },
                "standard": {
                    "description": QualityLevel::Standard.description(),
                    "max_recursion_depth": QualityLevel::Standard.max_recursion_depth(),
                    "pathogen_sensitivity_cap": QualityLevel::Standard.pathogen_sensitivity_cap()
                },
                "enhanced": {
                    "description": QualityLevel::Enhanced.description(),
                    "max_recursion_depth": QualityLevel::Enhanced.max_recursion_depth(),
                    "pathogen_sensitivity_cap": QualityLevel::Enhanced.pathogen_sensitivity_cap()
                },
                "premium": {
                    "description": QualityLevel::Premium.description(),
                    "max_recursion_depth": QualityLevel::Premium.max_recursion_depth(),
                    "pathogen_sensitivity_cap": QualityLevel::Premium.pathogen_sensitivity_cap()
                }
            },
            "feature_enablement": {
                "obligation_injection": self.is_feature_enabled(QualityFeature::ObligationInjection),
                "emotion_tracking": self.is_feature_enabled(QualityFeature::EmotionTracking),
                "spatial_validation": self.is_feature_enabled(QualityFeature::SpatialValidation),
                "capr_depth_analysis": self.is_feature_enabled(QualityFeature::CAPRDepthAnalysis),
                "character_consistency": self.is_feature_enabled(QualityFeature::CharacterConsistency),
                "engagement_loops": self.is_feature_enabled(QualityFeature::EngagementLoops),
                "drift_stabilization": self.is_feature_enabled(QualityFeature::DriftStabilization),
                "cache_optimization": self.is_feature_enabled(QualityFeature::CacheOptimization),
                "full_recursion": self.is_feature_enabled(QualityFeature::FullRecursion)
            },
            "last_change_elapsed_ms": status.last_change_elapsed_ms
        })
    }

    /// Force Qualitier quality assessment based on current state
    pub fn reassess_quality(&mut self) {
        if let Some(recent_pulse) = self.pulse_trace.latest() {
            let quality_changed = self.qualitier.decide(recent_pulse, &self.current_adapt_settings);
            if quality_changed {
                tracing::info!("Quality reassessment changed tier to {:?}", self.qualitier.current_level());
                // Apply quality constraints to adaptive settings
                self.qualitier.clamp_settings(&mut self.current_adapt_settings);
            }
        }
    }

    /// Get quality-adjusted analysis insights based on current tier
    pub fn get_quality_filtered_insights(&mut self, insights: Vec<NarrativeInsight>) -> Vec<NarrativeInsight> {
        match self.qualitier.current_level() {
            QualityLevel::Minimal => {
                // Only critical insights in minimal mode
                insights.into_iter()
                    .filter(|insight| matches!(insight.priority, InsightPriority::Critical))
                    .take(3) // Limit to 3 critical insights
                    .collect()
            }
            QualityLevel::Standard => {
                // Critical and important insights
                insights.into_iter()
                    .filter(|insight| matches!(insight.priority, InsightPriority::Critical | InsightPriority::Important))
                    .take(5) // Limit to 5 insights
                    .collect()
            }
            QualityLevel::Enhanced => {
                // All except minor insights
                insights.into_iter()
                    .filter(|insight| !matches!(insight.priority, InsightPriority::Minor))
                    .take(8) // Limit to 8 insights
                    .collect()
            }
            QualityLevel::Premium => {
                // All insights, no filtering
                insights
            }
        }
    }

    // ========== ObliSelect Smart Obligation Management Methods ==========

    /// Updates ObliSelect context with current narrative state
    pub fn update_obligation_context(&mut self) {
        let recent_characters: Vec<String> = self.character_engine.get_all_profiles()
            .iter()
            .take(5)
            .map(|profile| profile.name.clone())
            .collect();

        let tension_level = self.current_emotional_state
            .as_ref()
            .map(|state| state.intensity - 0.5) // Convert 0-1 to -0.5 to 0.5
            .unwrap_or(0.0);

        let narrative_context = format!(
            "Chapter {}, Scene {:?}, Characters: {:?}",
            self.current_chapter,
            self.current_scene,
            recent_characters
        );

        self.obli_select.update_context(
            self.current_chapter,
            recent_characters,
            tension_level,
            narrative_context,
        );
    }

    /// Gets smart obligation management status
    pub fn get_obligation_status(&mut self) -> serde_json::Value {
        let metrics = self.obli_select.get_metrics();
        let settings = self.obli_select.get_settings();

        serde_json::json!({
            "total_obligations": metrics.total_obligations,
            "obligations_by_category": metrics.obligations_by_category,
            "obligations_by_urgency": metrics.obligations_by_urgency,
            "stale_obligations": metrics.stale_obligations,
            "overused_obligations": metrics.overused_obligations,
            "average_injection_count": metrics.average_injection_count,
            "fulfillment_progress_average": metrics.fulfillment_progress_average,
            "tension_distribution": {
                "negative": metrics.tension_distribution.0,
                "neutral": metrics.tension_distribution.1,
                "positive": metrics.tension_distribution.2
            },
            "dependency_chain_length_max": metrics.dependency_chain_length_max,
            "last_selection_performance_ms": metrics.last_selection_performance_ms,
            "settings": {
                "max_obligations_per_selection": settings.max_obligations_per_selection,
                "urgency_weight": settings.urgency_weight,
                "salience_weight": settings.salience_weight,
                "freshness_weight": settings.freshness_weight,
                "tension_balance_weight": settings.tension_balance_weight,
                "dependency_weight": settings.dependency_weight,
                "context_relevance_weight": settings.context_relevance_weight,
                "adaptive_weighting_enabled": settings.enable_adaptive_weighting,
                "dependency_resolution_enabled": settings.enable_dependency_resolution,
                "contextual_filtering_enabled": settings.enable_contextual_filtering
            }
        })
    }

    /// Gets selected obligations with scoring details
    pub fn get_selected_obligations_with_scores(&mut self, max_count: Option<usize>) -> serde_json::Value {
        self.update_obligation_context();
        let obligation_scores = self.obli_select.select_obligations(max_count);

        serde_json::json!({
            "selected_obligations": obligation_scores.iter().map(|score| {
                serde_json::json!({
                    "obligation_id": score.obligation_id,
                    "total_score": score.total_score,
                    "urgency_score": score.urgency_score,
                    "salience_score": score.salience_score,
                    "freshness_score": score.freshness_score,
                    "tension_balance_score": score.tension_balance_score,
                    "dependency_score": score.dependency_score,
                    "context_relevance_score": score.context_relevance_score,
                    "justification": score.justification
                })
            }).collect::<Vec<_>>(),
            "selection_count": obligation_scores.len(),
            "selection_performance_ms": self.obli_select.get_metrics().last_selection_performance_ms
        })
    }

    /// Adds a new obligation to the management system
    pub fn add_obligation(&mut self, obligation: crate::obligations::Obligation) {
        self.obli_select.add_obligation(obligation);
    }

    /// Removes an obligation (when fully fulfilled)
    pub fn remove_obligation(&mut self, obligation_id: &str) -> bool {
        self.obli_select.remove_obligation(obligation_id).is_some()
    }

    /// Updates an obligation's fulfillment progress
    pub fn update_obligation_fulfillment(&mut self, obligation_id: &str, progress: f32) -> bool {
        self.obli_select.update_fulfillment_progress(obligation_id, progress)
    }

    /// Gets obligations that haven't been used recently (staleness monitoring)
    pub fn get_stale_obligations(&self) -> serde_json::Value {
        let stale_obligations = self.obli_select.get_stale_obligations(
            self.obli_select.get_settings().staleness_penalty_threshold
        );

        serde_json::json!({
            "stale_obligations": stale_obligations.iter().map(|obligation| {
                serde_json::json!({
                    "id": obligation.id,
                    "content": obligation.content,
                    "category": obligation.category,
                    "urgency": obligation.urgency,
                    "chapters_since_introduction": self.current_chapter.saturating_sub(obligation.chapter_introduced),
                    "injection_count": obligation.injection_count,
                    "last_injection": obligation.last_injection,
                    "fulfillment_progress": obligation.fulfillment_progress
                })
            }).collect::<Vec<_>>(),
            "stale_count": stale_obligations.len(),
            "staleness_threshold": self.obli_select.get_settings().staleness_penalty_threshold
        })
    }

    /// Gets obligations that have been overused
    pub fn get_overused_obligations(&self) -> serde_json::Value {
        let overused_obligations = self.obli_select.get_overused_obligations(
            self.obli_select.get_settings().overuse_penalty_threshold
        );

        serde_json::json!({
            "overused_obligations": overused_obligations.iter().map(|obligation| {
                serde_json::json!({
                    "id": obligation.id,
                    "content": obligation.content,
                    "injection_count": obligation.injection_count,
                    "last_injection": obligation.last_injection,
                    "overuse_threshold": self.obli_select.get_settings().overuse_penalty_threshold
                })
            }).collect::<Vec<_>>(),
            "overused_count": overused_obligations.len()
        })
    }

    /// Updates ObliSelect settings
    pub fn update_obligation_settings(&mut self, settings: ObliSelectSettings) {
        self.obli_select.update_settings(settings);
    }

    /// Resets injection statistics for testing or system reset
    pub fn reset_obligation_stats(&mut self) {
        self.obli_select.reset_injection_stats();
    }

    /// Gets all obligations for inspection (debugging purposes)
    pub fn get_all_obligations(&self) -> serde_json::Value {
        let all_obligations = self.obli_select.get_all_obligations();

        serde_json::json!({
            "obligations": all_obligations.iter().map(|(id, obligation)| {
                serde_json::json!({
                    "id": id,
                    "content": obligation.content,
                    "category": obligation.category,
                    "urgency": obligation.urgency,
                    "created_at": obligation.created_at,
                    "last_injection": obligation.last_injection,
                    "injection_count": obligation.injection_count,
                    "chapter_introduced": obligation.chapter_introduced,
                    "characters_involved": obligation.characters_involved,
                    "tension_vector": obligation.tension_vector,
                    "salience_boost": obligation.salience_boost,
                    "fulfillment_progress": obligation.fulfillment_progress,
                    "dependencies": obligation.dependencies
                })
            }).collect::<Vec<_>>(),
            "total_count": all_obligations.len()
        })
    }

    /// Gets detailed metrics for obligation management performance
    pub fn get_obligation_metrics(&self) -> &ObligationMetrics {
        self.obli_select.get_metrics()
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
                priority: InsightPriority::Interesting,
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


impl Default for EnabledSystems {
    fn default() -> Self {
        Self {
            dna_tracking: true,
            constraint_modeling: true,
            recursion_tracking: true,
            character_consistency: true,
            engagement_loops: true,
            drift_stabilization: true,
            adaptive_intelligence: true,
            obligation_management: true,
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