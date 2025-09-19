/// 📚 Recursive Reader Engagement Loop Detection
///
/// Detects and amplifies recursive feedback loops between reader engagement
/// and narrative structure. These loops manifest as structural patterns:
/// - Curiosity → Hypothesis formation
/// - Emotional Investment → Payoff expectation
/// - Confusion → Pattern Recognition seeking
/// - Moral Tension → Reconciliation need
/// - Identity Alignment → Reflection desire
///
/// This system helps maintain reader retention by tracking recursive re-binding patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Types of engagement loops that readers form with narratives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EngagementLoopType {
    /// Reader forms hypotheses about unresolved mysteries
    CuriosityHypothesis {
        mystery_element: String,
        hypothesis_strength: f32,
    },
    /// Reader becomes emotionally invested and expects payoff
    EmotionalInvestment {
        character_or_situation: String,
        investment_level: f32,
        payoff_expectation: f32,
    },
    /// Reader experiences confusion and seeks pattern recognition
    ConfusionRecognition {
        confusing_element: String,
        pattern_seeking_intensity: f32,
    },
    /// Reader faces moral tension requiring reconciliation
    MoralTension {
        moral_question: String,
        tension_level: f32,
        reconciliation_urgency: f32,
    },
    /// Reader aligns with character identity and seeks reflection
    IdentityAlignment {
        character: String,
        alignment_strength: f32,
        reflection_depth: f32,
    },
    /// Reader forms expectations about genre conventions
    GenreExpectation {
        expected_pattern: String,
        confidence_level: f32,
        subversion_potential: f32,
    },
}

/// A feedback loop instance tracking reader engagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementLoop {
    pub id: String,
    pub loop_type: EngagementLoopType,
    pub initiated_chapter: u32,
    pub initiated_scene: Option<u32>,
    pub current_intensity: f32, // 0.0 to 1.0
    pub peak_intensity: f32,
    pub tension_buildup: f32, // How much tension has accumulated
    pub last_reinforcement: DateTime<Utc>,
    pub expected_resolution_type: Option<ResolutionType>,
    pub loop_history: Vec<LoopEvent>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Types of loop resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Direct answer to mystery/question
    DirectResolution,
    /// Subversion of expectation
    Subversion,
    /// Escalation to higher stakes
    Escalation,
    /// Transformation of the question itself
    Transformation,
    /// Delayed/deferred resolution
    Deferral,
}

/// Events in an engagement loop's lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopEvent {
    pub event_type: LoopEventType,
    pub description: String,
    pub intensity_change: f32,
    pub chapter: u32,
    pub scene: Option<u32>,
    pub timestamp: DateTime<Utc>,
}

/// Types of events that affect engagement loops
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoopEventType {
    /// Initial creation of the loop
    Initiation,
    /// Reinforcement that strengthens the loop
    Reinforcement,
    /// Complication that adds complexity
    Complication,
    /// Tease that maintains tension without resolving
    Tease,
    /// Partial resolution that reduces but doesn't eliminate tension
    PartialResolution,
    /// Full resolution that closes the loop
    FullResolution,
    /// Subversion that transforms expectations
    Subversion,
    /// Abandonment of the loop without resolution
    Abandonment,
}

/// Tracks all reader engagement loops and their interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderEngagementTracker {
    /// All active and resolved engagement loops
    pub loops: HashMap<String, EngagementLoop>,
    /// Index by loop type for analysis
    pub type_index: HashMap<String, Vec<String>>,
    /// Index by character for character-focused loops
    pub character_index: HashMap<String, Vec<String>>,
    /// Current narrative context
    pub current_chapter: u32,
    pub current_scene: Option<u32>,
    /// Engagement metrics
    pub engagement_metrics: EngagementMetrics,
    /// Loop interaction patterns
    pub loop_interactions: Vec<LoopInteraction>,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Metrics about overall reader engagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub total_loops_created: usize,
    pub active_loops: usize,
    pub resolved_loops: usize,
    pub abandoned_loops: usize,
    pub average_loop_duration: f32, // Chapters
    pub engagement_intensity_score: f32, // Combined intensity of all active loops
    pub loop_resolution_rate: f32, // Percentage of loops that get resolved
    pub tension_buildup_rate: f32, // How quickly tension accumulates
    pub reader_retention_score: f32, // Estimated reader retention based on loop health
}

/// Interaction between multiple engagement loops
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopInteraction {
    pub primary_loop_id: String,
    pub secondary_loop_id: String,
    pub interaction_type: InteractionType,
    pub synergy_factor: f32, // How much loops amplify each other
    pub conflict_factor: f32, // How much loops compete for attention
    pub resolution_dependency: bool, // Whether one loop must resolve before the other
    pub chapter: u32,
    pub timestamp: DateTime<Utc>,
}

/// Types of interactions between engagement loops
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    /// Loops reinforce each other
    Synergy,
    /// Loops compete for reader attention
    Competition,
    /// One loop depends on another's resolution
    Dependency,
    /// Loops merge into a larger pattern
    Convergence,
    /// One loop contradicts another
    Contradiction,
}

impl ReaderEngagementTracker {
    /// Creates a new reader engagement tracker
    pub fn new() -> Self {
        Self {
            loops: HashMap::new(),
            type_index: HashMap::new(),
            character_index: HashMap::new(),
            current_chapter: 1,
            current_scene: None,
            engagement_metrics: EngagementMetrics {
                total_loops_created: 0,
                active_loops: 0,
                resolved_loops: 0,
                abandoned_loops: 0,
                average_loop_duration: 0.0,
                engagement_intensity_score: 0.0,
                loop_resolution_rate: 0.0,
                tension_buildup_rate: 0.0,
                reader_retention_score: 1.0,
            },
            loop_interactions: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    /// Creates a new engagement loop
    pub fn initiate_loop(&mut self, loop_type: EngagementLoopType, initial_intensity: f32) -> String {
        let loop_id = self.generate_loop_id(&loop_type);

        let initial_event = LoopEvent {
            event_type: LoopEventType::Initiation,
            description: format!("Loop initiated: {}", self.describe_loop_type(&loop_type)),
            intensity_change: initial_intensity,
            chapter: self.current_chapter,
            scene: self.current_scene,
            timestamp: Utc::now(),
        };

        let engagement_loop = EngagementLoop {
            id: loop_id.clone(),
            loop_type: loop_type.clone(),
            initiated_chapter: self.current_chapter,
            initiated_scene: self.current_scene,
            current_intensity: initial_intensity,
            peak_intensity: initial_intensity,
            tension_buildup: initial_intensity,
            last_reinforcement: Utc::now(),
            expected_resolution_type: None,
            loop_history: vec![initial_event],
            is_active: true,
            created_at: Utc::now(),
        };

        // Update indices
        let type_key = self.loop_type_to_string(&loop_type);
        self.type_index
            .entry(type_key)
            .or_insert_with(Vec::new)
            .push(loop_id.clone());

        if let Some(character) = self.extract_character_from_loop(&loop_type) {
            self.character_index
                .entry(character)
                .or_insert_with(Vec::new)
                .push(loop_id.clone());
        }

        self.loops.insert(loop_id.clone(), engagement_loop);
        self.update_metrics();
        self.last_updated = Utc::now();

        loop_id
    }

    /// Generates a unique ID for a loop
    fn generate_loop_id(&self, loop_type: &EngagementLoopType) -> String {
        let type_prefix = match loop_type {
            EngagementLoopType::CuriosityHypothesis { .. } => "curiosity",
            EngagementLoopType::EmotionalInvestment { .. } => "emotional",
            EngagementLoopType::ConfusionRecognition { .. } => "confusion",
            EngagementLoopType::MoralTension { .. } => "moral",
            EngagementLoopType::IdentityAlignment { .. } => "identity",
            EngagementLoopType::GenreExpectation { .. } => "genre",
        };

        format!("{}_{}_ch{}", type_prefix, self.loops.len(), self.current_chapter)
    }

    /// Converts loop type to string for indexing
    fn loop_type_to_string(&self, loop_type: &EngagementLoopType) -> String {
        match loop_type {
            EngagementLoopType::CuriosityHypothesis { .. } => "curiosity".to_string(),
            EngagementLoopType::EmotionalInvestment { .. } => "emotional".to_string(),
            EngagementLoopType::ConfusionRecognition { .. } => "confusion".to_string(),
            EngagementLoopType::MoralTension { .. } => "moral".to_string(),
            EngagementLoopType::IdentityAlignment { .. } => "identity".to_string(),
            EngagementLoopType::GenreExpectation { .. } => "genre".to_string(),
        }
    }

    /// Describes a loop type in human-readable form
    fn describe_loop_type(&self, loop_type: &EngagementLoopType) -> String {
        match loop_type {
            EngagementLoopType::CuriosityHypothesis { mystery_element, .. } => {
                format!("Curiosity about {}", mystery_element)
            }
            EngagementLoopType::EmotionalInvestment { character_or_situation, .. } => {
                format!("Emotional investment in {}", character_or_situation)
            }
            EngagementLoopType::ConfusionRecognition { confusing_element, .. } => {
                format!("Confusion about {}", confusing_element)
            }
            EngagementLoopType::MoralTension { moral_question, .. } => {
                format!("Moral tension: {}", moral_question)
            }
            EngagementLoopType::IdentityAlignment { character, .. } => {
                format!("Identity alignment with {}", character)
            }
            EngagementLoopType::GenreExpectation { expected_pattern, .. } => {
                format!("Genre expectation: {}", expected_pattern)
            }
        }
    }

    /// Extracts character name from loop type if applicable
    fn extract_character_from_loop(&self, loop_type: &EngagementLoopType) -> Option<String> {
        match loop_type {
            EngagementLoopType::EmotionalInvestment { character_or_situation, .. } => {
                Some(character_or_situation.clone())
            }
            EngagementLoopType::IdentityAlignment { character, .. } => {
                Some(character.clone())
            }
            _ => None,
        }
    }

    /// Adds an event to an existing loop
    pub fn add_loop_event(&mut self, loop_id: &str, event_type: LoopEventType, description: String, intensity_change: f32) {
        if let Some(loop_data) = self.loops.get_mut(loop_id) {
            let event = LoopEvent {
                event_type: event_type.clone(),
                description,
                intensity_change,
                chapter: self.current_chapter,
                scene: self.current_scene,
                timestamp: Utc::now(),
            };

            // Update loop intensity
            loop_data.current_intensity = (loop_data.current_intensity + intensity_change).max(0.0).min(1.0);
            loop_data.peak_intensity = loop_data.peak_intensity.max(loop_data.current_intensity);

            // Update tension buildup
            match event_type {
                LoopEventType::Reinforcement | LoopEventType::Complication => {
                    loop_data.tension_buildup += intensity_change.abs();
                    loop_data.last_reinforcement = Utc::now();
                }
                LoopEventType::PartialResolution => {
                    loop_data.tension_buildup *= 0.7; // Partial release
                }
                LoopEventType::FullResolution => {
                    loop_data.tension_buildup = 0.0;
                    loop_data.is_active = false;
                }
                LoopEventType::Abandonment => {
                    loop_data.is_active = false;
                }
                _ => {}
            }

            loop_data.loop_history.push(event);
        }

        self.update_metrics();
        self.last_updated = Utc::now();
    }

    /// Resolves a loop with specified resolution type
    pub fn resolve_loop(&mut self, loop_id: &str, resolution_type: ResolutionType, description: String) {
        if let Some(loop_data) = self.loops.get_mut(loop_id) {
            loop_data.expected_resolution_type = Some(resolution_type.clone());

            let event_type = match resolution_type {
                ResolutionType::DirectResolution => LoopEventType::FullResolution,
                ResolutionType::Subversion => LoopEventType::Subversion,
                ResolutionType::Escalation => LoopEventType::Complication,
                ResolutionType::Transformation => LoopEventType::Subversion,
                ResolutionType::Deferral => LoopEventType::Tease,
            };

            let intensity_change = match resolution_type {
                ResolutionType::DirectResolution => -loop_data.current_intensity,
                ResolutionType::Subversion => -0.5,
                ResolutionType::Escalation => 0.3,
                ResolutionType::Transformation => -0.3,
                ResolutionType::Deferral => 0.1,
            };

            self.add_loop_event(loop_id, event_type, description, intensity_change);
        }
    }

    /// Advances narrative context
    pub fn advance_context(&mut self, chapter: Option<u32>, scene: Option<u32>) {
        if let Some(ch) = chapter {
            self.current_chapter = ch;
        }
        self.current_scene = scene;

        // Check for stale loops (not reinforced recently)
        self.check_stale_loops();
        self.update_metrics();
        self.last_updated = Utc::now();
    }

    /// Checks for loops that haven't been reinforced recently
    fn check_stale_loops(&mut self) {
        let current_time = Utc::now();
        let stale_threshold = chrono::Duration::hours(2); // 2 hours in story time

        for (loop_id, loop_data) in &mut self.loops {
            if loop_data.is_active && current_time - loop_data.last_reinforcement > stale_threshold {
                // Mark loop as potentially stale
                loop_data.current_intensity *= 0.9; // Gradual decay

                if loop_data.current_intensity < 0.1 {
                    self.add_loop_event(
                        loop_id,
                        LoopEventType::Abandonment,
                        "Loop abandoned due to lack of reinforcement".to_string(),
                        -loop_data.current_intensity,
                    );
                }
            }
        }
    }

    /// Records an interaction between two loops
    pub fn record_loop_interaction(&mut self, primary_loop: &str, secondary_loop: &str, interaction_type: InteractionType, synergy: f32, conflict: f32) {
        let interaction = LoopInteraction {
            primary_loop_id: primary_loop.to_string(),
            secondary_loop_id: secondary_loop.to_string(),
            interaction_type,
            synergy_factor: synergy,
            conflict_factor: conflict,
            resolution_dependency: false,
            chapter: self.current_chapter,
            timestamp: Utc::now(),
        };

        self.loop_interactions.push(interaction);
        self.last_updated = Utc::now();
    }

    /// Detects potential loop interactions automatically
    pub fn detect_loop_interactions(&mut self) -> Vec<PotentialInteraction> {
        let mut potential_interactions = Vec::new();

        let active_loops: Vec<&EngagementLoop> = self.loops
            .values()
            .filter(|loop_data| loop_data.is_active)
            .collect();

        // Check for loops involving the same character
        for i in 0..active_loops.len() {
            for j in i + 1..active_loops.len() {
                let loop_a = active_loops[i];
                let loop_b = active_loops[j];

                // Check character overlap
                if let (Some(char_a), Some(char_b)) = (
                    self.extract_character_from_loop(&loop_a.loop_type),
                    self.extract_character_from_loop(&loop_b.loop_type),
                ) {
                    if char_a == char_b {
                        potential_interactions.push(PotentialInteraction {
                            loop_a_id: loop_a.id.clone(),
                            loop_b_id: loop_b.id.clone(),
                            interaction_type: InteractionType::Synergy,
                            confidence: 0.8,
                            description: format!("Both loops involve character {}", char_a),
                        });
                    }
                }

                // Check for thematic overlap
                if self.has_thematic_overlap(&loop_a.loop_type, &loop_b.loop_type) {
                    potential_interactions.push(PotentialInteraction {
                        loop_a_id: loop_a.id.clone(),
                        loop_b_id: loop_b.id.clone(),
                        interaction_type: InteractionType::Convergence,
                        confidence: 0.6,
                        description: "Loops share thematic elements".to_string(),
                    });
                }

                // Check for competing attention
                if loop_a.current_intensity > 0.7 && loop_b.current_intensity > 0.7 {
                    potential_interactions.push(PotentialInteraction {
                        loop_a_id: loop_a.id.clone(),
                        loop_b_id: loop_b.id.clone(),
                        interaction_type: InteractionType::Competition,
                        confidence: 0.5,
                        description: "High-intensity loops may compete for reader attention".to_string(),
                    });
                }
            }
        }

        potential_interactions
    }

    /// Checks if two loop types have thematic overlap
    fn has_thematic_overlap(&self, loop_a: &EngagementLoopType, loop_b: &EngagementLoopType) -> bool {
        match (loop_a, loop_b) {
            (EngagementLoopType::MoralTension { .. }, EngagementLoopType::IdentityAlignment { .. }) => true,
            (EngagementLoopType::EmotionalInvestment { .. }, EngagementLoopType::IdentityAlignment { .. }) => true,
            (EngagementLoopType::CuriosityHypothesis { .. }, EngagementLoopType::ConfusionRecognition { .. }) => true,
            _ => false,
        }
    }

    /// Updates engagement metrics
    fn update_metrics(&mut self) {
        self.engagement_metrics.total_loops_created = self.loops.len();
        self.engagement_metrics.active_loops = self.loops.values().filter(|l| l.is_active).count();
        self.engagement_metrics.resolved_loops = self.loops.values()
            .filter(|l| !l.is_active && l.expected_resolution_type.is_some())
            .count();
        self.engagement_metrics.abandoned_loops = self.loops.values()
            .filter(|l| !l.is_active && l.expected_resolution_type.is_none())
            .count();

        // Calculate average loop duration
        let completed_loops: Vec<&EngagementLoop> = self.loops.values()
            .filter(|l| !l.is_active)
            .collect();

        if !completed_loops.is_empty() {
            let total_duration: u32 = completed_loops.iter()
                .map(|l| self.current_chapter - l.initiated_chapter)
                .sum();
            self.engagement_metrics.average_loop_duration = total_duration as f32 / completed_loops.len() as f32;
        }

        // Calculate engagement intensity
        self.engagement_metrics.engagement_intensity_score = self.loops.values()
            .filter(|l| l.is_active)
            .map(|l| l.current_intensity)
            .sum::<f32>() / self.engagement_metrics.active_loops.max(1) as f32;

        // Calculate resolution rate
        if self.engagement_metrics.total_loops_created > 0 {
            self.engagement_metrics.loop_resolution_rate =
                self.engagement_metrics.resolved_loops as f32 / self.engagement_metrics.total_loops_created as f32;
        }

        // Calculate reader retention score
        self.engagement_metrics.reader_retention_score = self.calculate_retention_score();
    }

    /// Calculates estimated reader retention based on loop health
    fn calculate_retention_score(&self) -> f32 {
        let mut score = 0.5; // Base score

        // Reward active engagement loops
        if self.engagement_metrics.active_loops > 0 {
            score += 0.2;
        }

        // Reward good intensity balance
        if self.engagement_metrics.engagement_intensity_score >= 0.4 && self.engagement_metrics.engagement_intensity_score <= 0.8 {
            score += 0.2;
        }

        // Reward good resolution rate
        if self.engagement_metrics.loop_resolution_rate >= 0.7 {
            score += 0.2;
        }

        // Penalize too many abandoned loops
        if self.engagement_metrics.abandoned_loops > self.engagement_metrics.resolved_loops {
            score -= 0.3;
        }

        // Penalize excessive complexity
        if self.engagement_metrics.active_loops > 8 {
            score -= 0.1;
        }

        score.max(0.0).min(1.0)
    }

    /// Gets all active loops requiring attention
    pub fn get_loops_requiring_attention(&self) -> Vec<LoopAttentionReport> {
        let mut reports = Vec::new();

        for (loop_id, loop_data) in &self.loops {
            if !loop_data.is_active {
                continue;
            }

            let mut attention_level = 0.0;
            let mut reasons = Vec::new();

            // Check for high tension buildup
            if loop_data.tension_buildup > 2.0 {
                attention_level += 0.4;
                reasons.push("High tension buildup".to_string());
            }

            // Check for stale loops
            let hours_since_reinforcement = (Utc::now() - loop_data.last_reinforcement).num_hours();
            if hours_since_reinforcement > 1 {
                attention_level += 0.3;
                reasons.push(format!("No reinforcement for {} hours", hours_since_reinforcement));
            }

            // Check for very high intensity
            if loop_data.current_intensity > 0.9 {
                attention_level += 0.2;
                reasons.push("Very high intensity - may need resolution".to_string());
            }

            // Check for very low intensity
            if loop_data.current_intensity < 0.2 {
                attention_level += 0.1;
                reasons.push("Low intensity - may be losing reader interest".to_string());
            }

            if attention_level > 0.3 {
                reports.push(LoopAttentionReport {
                    loop_id: loop_id.clone(),
                    loop_description: self.describe_loop_type(&loop_data.loop_type),
                    attention_level,
                    current_intensity: loop_data.current_intensity,
                    tension_buildup: loop_data.tension_buildup,
                    reasons,
                    suggested_actions: self.generate_loop_suggestions(loop_data),
                });
            }
        }

        // Sort by attention level (highest first)
        reports.sort_by(|a, b| b.attention_level.partial_cmp(&a.attention_level).unwrap_or(std::cmp::Ordering::Equal));
        reports
    }

    /// Generates suggestions for a specific loop
    fn generate_loop_suggestions(&self, loop_data: &EngagementLoop) -> Vec<String> {
        let mut suggestions = Vec::new();

        if loop_data.tension_buildup > 2.0 {
            suggestions.push("Consider providing resolution or escalation to address high tension".to_string());
        }

        if loop_data.current_intensity > 0.9 {
            suggestions.push("High intensity suggests reader expects imminent payoff".to_string());
        }

        if loop_data.current_intensity < 0.2 {
            suggestions.push("Consider reinforcement or graceful abandonment of low-intensity loop".to_string());
        }

        let chapters_active = self.current_chapter - loop_data.initiated_chapter;
        if chapters_active > 5 {
            suggestions.push("Long-running loop may benefit from progression or partial resolution".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Continue monitoring loop development".to_string());
        }

        suggestions
    }

    /// Generates comprehensive engagement report
    pub fn generate_engagement_report(&self) -> String {
        let mut report = String::new();

        report.push_str("📚 READER ENGAGEMENT ANALYSIS\n");
        report.push_str("==============================\n\n");

        report.push_str(&format!("Current Chapter: {}\n", self.current_chapter));
        report.push_str(&format!("Active Engagement Loops: {}\n", self.engagement_metrics.active_loops));
        report.push_str(&format!("Total Loops Created: {}\n", self.engagement_metrics.total_loops_created));
        report.push_str(&format!("Resolution Rate: {:.2}\n", self.engagement_metrics.loop_resolution_rate));
        report.push_str(&format!("Reader Retention Score: {:.2}\n\n", self.engagement_metrics.reader_retention_score));

        // Loop type distribution
        report.push_str("🔄 Loop Type Distribution:\n");
        for (loop_type, loop_ids) in &self.type_index {
            let active_count = loop_ids.iter()
                .filter_map(|id| self.loops.get(id))
                .filter(|l| l.is_active)
                .count();
            let total_count = loop_ids.len();
            report.push_str(&format!("  • {}: {} active / {} total\n", loop_type, active_count, total_count));
        }
        report.push('\n');

        // Engagement intensity
        report.push_str(&format!("📊 Engagement Intensity: {:.2}/1.0\n", self.engagement_metrics.engagement_intensity_score));
        report.push_str(&format!("📊 Average Loop Duration: {:.1} chapters\n\n", self.engagement_metrics.average_loop_duration));

        // Loops requiring attention
        let attention_loops = self.get_loops_requiring_attention();
        if !attention_loops.is_empty() {
            report.push_str("⚠️ Loops Requiring Attention:\n");
            for (i, attention) in attention_loops.iter().take(5).enumerate() {
                report.push_str(&format!("  {}. {} (intensity: {:.2}, attention: {:.2})\n",
                    i + 1, attention.loop_description, attention.current_intensity, attention.attention_level));
            }
            report.push('\n');
        }

        // Recent interactions
        if !self.loop_interactions.is_empty() {
            report.push_str("🔗 Recent Loop Interactions:\n");
            let recent_interactions: Vec<&LoopInteraction> = self.loop_interactions
                .iter()
                .filter(|i| i.chapter >= self.current_chapter.saturating_sub(2))
                .collect();

            for interaction in recent_interactions.iter().take(3) {
                report.push_str(&format!("  • {:?} between {} and {} (synergy: {:.2})\n",
                    interaction.interaction_type,
                    interaction.primary_loop_id,
                    interaction.secondary_loop_id,
                    interaction.synergy_factor));
            }
            report.push('\n');
        }

        // Recommendations
        report.push_str("💡 Recommendations:\n");
        if self.engagement_metrics.reader_retention_score < 0.6 {
            report.push_str("  • Reader retention needs improvement - consider strengthening engagement loops\n");
        }
        if self.engagement_metrics.active_loops < 2 {
            report.push_str("  • Low number of active loops - consider introducing new engagement elements\n");
        }
        if self.engagement_metrics.active_loops > 8 {
            report.push_str("  • High number of active loops - consider resolving some to reduce complexity\n");
        }
        if self.engagement_metrics.loop_resolution_rate < 0.5 {
            report.push_str("  • Low resolution rate - readers may lose trust in payoffs\n");
        }

        report
    }
}

/// A potential interaction between engagement loops
#[derive(Debug, Clone)]
pub struct PotentialInteraction {
    pub loop_a_id: String,
    pub loop_b_id: String,
    pub interaction_type: InteractionType,
    pub confidence: f32,
    pub description: String,
}

/// Report of a loop requiring attention
#[derive(Debug, Clone)]
pub struct LoopAttentionReport {
    pub loop_id: String,
    pub loop_description: String,
    pub attention_level: f32,
    pub current_intensity: f32,
    pub tension_buildup: f32,
    pub reasons: Vec<String>,
    pub suggested_actions: Vec<String>,
}

impl Default for ReaderEngagementTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_engagement_tracker_creation() {
        let tracker = ReaderEngagementTracker::new();
        assert_eq!(tracker.current_chapter, 1);
        assert!(tracker.loops.is_empty());
        assert_eq!(tracker.engagement_metrics.reader_retention_score, 1.0);
    }

    #[test]
    fn test_initiate_loop() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop_type = EngagementLoopType::CuriosityHypothesis {
            mystery_element: "The locked door".to_string(),
            hypothesis_strength: 0.8,
        };

        let loop_id = tracker.initiate_loop(loop_type, 0.7);

        assert_eq!(tracker.loops.len(), 1);
        assert!(tracker.loops.contains_key(&loop_id));
        assert_eq!(tracker.engagement_metrics.active_loops, 1);

        let loop_data = tracker.loops.get(&loop_id).unwrap();
        assert_eq!(loop_data.current_intensity, 0.7);
        assert!(loop_data.is_active);
    }

    #[test]
    fn test_add_loop_event() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop_type = EngagementLoopType::EmotionalInvestment {
            character_or_situation: "Hero's safety".to_string(),
            investment_level: 0.6,
            payoff_expectation: 0.8,
        };

        let loop_id = tracker.initiate_loop(loop_type, 0.5);

        tracker.add_loop_event(
            &loop_id,
            LoopEventType::Reinforcement,
            "Hero faces new danger".to_string(),
            0.2,
        );

        let loop_data = tracker.loops.get(&loop_id).unwrap();
        assert_eq!(loop_data.current_intensity, 0.7); // 0.5 + 0.2
        assert_eq!(loop_data.loop_history.len(), 2); // Initial + reinforcement
    }

    #[test]
    fn test_resolve_loop() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop_type = EngagementLoopType::MoralTension {
            moral_question: "Is revenge justified?".to_string(),
            tension_level: 0.8,
            reconciliation_urgency: 0.9,
        };

        let loop_id = tracker.initiate_loop(loop_type, 0.8);

        tracker.resolve_loop(
            &loop_id,
            ResolutionType::DirectResolution,
            "Character chooses forgiveness".to_string(),
        );

        let loop_data = tracker.loops.get(&loop_id).unwrap();
        assert!(!loop_data.is_active);
        assert_eq!(loop_data.expected_resolution_type, Some(ResolutionType::DirectResolution));
        assert_eq!(loop_data.tension_buildup, 0.0);
    }

    #[test]
    fn test_loop_interactions() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop1_type = EngagementLoopType::IdentityAlignment {
            character: "Hero".to_string(),
            alignment_strength: 0.7,
            reflection_depth: 0.6,
        };

        let loop2_type = EngagementLoopType::EmotionalInvestment {
            character_or_situation: "Hero".to_string(),
            investment_level: 0.8,
            payoff_expectation: 0.9,
        };

        let loop1_id = tracker.initiate_loop(loop1_type, 0.7);
        let loop2_id = tracker.initiate_loop(loop2_type, 0.8);

        tracker.record_loop_interaction(
            &loop1_id,
            &loop2_id,
            InteractionType::Synergy,
            0.8,
            0.1,
        );

        assert_eq!(tracker.loop_interactions.len(), 1);
        assert_eq!(tracker.loop_interactions[0].interaction_type, InteractionType::Synergy);
    }

    #[test]
    fn test_detect_loop_interactions() {
        let mut tracker = ReaderEngagementTracker::new();

        // Create two loops involving the same character
        let loop1_type = EngagementLoopType::IdentityAlignment {
            character: "Hero".to_string(),
            alignment_strength: 0.7,
            reflection_depth: 0.6,
        };

        let loop2_type = EngagementLoopType::EmotionalInvestment {
            character_or_situation: "Hero".to_string(),
            investment_level: 0.8,
            payoff_expectation: 0.9,
        };

        tracker.initiate_loop(loop1_type, 0.7);
        tracker.initiate_loop(loop2_type, 0.8);

        let potential_interactions = tracker.detect_loop_interactions();
        assert!(!potential_interactions.is_empty());

        let character_interaction = potential_interactions.iter()
            .find(|i| i.interaction_type == InteractionType::Synergy);
        assert!(character_interaction.is_some());
    }

    #[test]
    fn test_advance_context() {
        let mut tracker = ReaderEngagementTracker::new();
        assert_eq!(tracker.current_chapter, 1);

        tracker.advance_context(Some(3), Some(2));
        assert_eq!(tracker.current_chapter, 3);
        assert_eq!(tracker.current_scene, Some(2));
    }

    #[test]
    fn test_stale_loop_detection() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop_type = EngagementLoopType::CuriosityHypothesis {
            mystery_element: "Test mystery".to_string(),
            hypothesis_strength: 0.6,
        };

        let loop_id = tracker.initiate_loop(loop_type, 0.6);

        // Manually set an old timestamp to simulate staleness
        if let Some(loop_data) = tracker.loops.get_mut(&loop_id) {
            loop_data.last_reinforcement = Utc::now() - chrono::Duration::hours(3);
        }

        tracker.advance_context(Some(2), None);

        let loop_data = tracker.loops.get(&loop_id).unwrap();
        assert!(loop_data.current_intensity < 0.6); // Should have decayed
    }

    #[test]
    fn test_loops_requiring_attention() {
        let mut tracker = ReaderEngagementTracker::new();

        // Create a loop with high tension
        let loop_type = EngagementLoopType::MoralTension {
            moral_question: "Test question".to_string(),
            tension_level: 0.9,
            reconciliation_urgency: 0.8,
        };

        let loop_id = tracker.initiate_loop(loop_type, 0.9);

        // Add events to build up tension
        tracker.add_loop_event(&loop_id, LoopEventType::Reinforcement, "Tension increases".to_string(), 0.1);
        tracker.add_loop_event(&loop_id, LoopEventType::Complication, "More complications".to_string(), 0.2);

        let attention_reports = tracker.get_loops_requiring_attention();
        assert!(!attention_reports.is_empty());

        let high_tension_report = attention_reports.iter()
            .find(|r| r.reasons.iter().any(|reason| reason.contains("tension")));
        assert!(high_tension_report.is_some());
    }

    #[test]
    fn test_engagement_metrics_calculation() {
        let mut tracker = ReaderEngagementTracker::new();

        // Create several loops
        for i in 0..5 {
            let loop_type = EngagementLoopType::CuriosityHypothesis {
                mystery_element: format!("Mystery {}", i),
                hypothesis_strength: 0.5,
            };
            tracker.initiate_loop(loop_type, 0.6);
        }

        // Resolve some loops
        let loop_ids: Vec<String> = tracker.loops.keys().take(2).cloned().collect();
        for loop_id in &loop_ids {
            tracker.resolve_loop(loop_id, ResolutionType::DirectResolution, "Resolved".to_string());
        }

        assert_eq!(tracker.engagement_metrics.total_loops_created, 5);
        assert_eq!(tracker.engagement_metrics.active_loops, 3);
        assert_eq!(tracker.engagement_metrics.resolved_loops, 2);
        assert!(tracker.engagement_metrics.loop_resolution_rate > 0.0);
    }

    #[test]
    fn test_generate_engagement_report() {
        let mut tracker = ReaderEngagementTracker::new();

        let loop_type = EngagementLoopType::GenreExpectation {
            expected_pattern: "Hero's journey".to_string(),
            confidence_level: 0.8,
            subversion_potential: 0.3,
        };

        tracker.initiate_loop(loop_type, 0.7);

        let report = tracker.generate_engagement_report();
        assert!(report.contains("READER ENGAGEMENT ANALYSIS"));
        assert!(report.contains("Active Engagement Loops: 1"));
        assert!(report.contains("Reader Retention Score"));
    }
}