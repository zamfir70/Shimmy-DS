// ObliSelect: Smart Obligation Management System
// Claude Code Card #5 Implementation
// Intelligent obligation selection and scoring based on narrative context

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Represents a single narrative obligation with metadata for intelligent selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub id: String,
    pub content: String,
    pub category: ObligationCategory,
    pub urgency: ObligationUrgency,
    pub created_at: DateTime<Utc>,
    pub last_injection: Option<DateTime<Utc>>,
    pub injection_count: u32,
    pub chapter_introduced: u32,
    pub characters_involved: Vec<String>,
    pub tension_vector: f32,  // -1.0 to 1.0, where negative is tension relief, positive is tension building
    pub salience_boost: f32,  // Manual boost factor for important obligations
    pub fulfillment_progress: f32,  // 0.0 to 1.0, how much of this obligation has been addressed
    pub dependencies: Vec<String>,  // Other obligation IDs this depends on
}

/// Categories of narrative obligations for contextual grouping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationCategory {
    CharacterDevelopment,
    PlotAdvancement,
    WorldBuilding,
    EmotionalResolution,
    Foreshadowing,
    ThematicExploration,
    DialoguePromise,
    SettingDetail,
    ConflictResolution,
    RelationshipDynamics,
}

/// Urgency levels for obligation prioritization
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ObligationUrgency {
    Low,      // Can wait many chapters
    Medium,   // Should be addressed within a few chapters
    High,     // Needs attention soon
    Critical, // Must be addressed immediately
}

/// Scoring metrics for obligation selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationScore {
    pub obligation_id: String,
    pub total_score: f32,
    pub urgency_score: f32,
    pub salience_score: f32,
    pub freshness_score: f32,
    pub tension_balance_score: f32,
    pub dependency_score: f32,
    pub context_relevance_score: f32,
    pub justification: String,
}

/// Performance and selection metrics for ObliSelect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationMetrics {
    pub total_obligations: usize,
    pub obligations_by_category: HashMap<ObligationCategory, usize>,
    pub obligations_by_urgency: HashMap<ObligationUrgency, usize>,
    pub average_injection_count: f32,
    pub stale_obligations: usize,  // Not injected in > 5 chapters
    pub overused_obligations: usize,  // Injected > 10 times
    pub tension_distribution: (f32, f32, f32),  // (negative, neutral, positive) percentages
    pub fulfillment_progress_average: f32,
    pub dependency_chain_length_max: usize,
    pub last_selection_performance_ms: u64,
}

/// Configuration settings for ObliSelect behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObliSelectSettings {
    pub max_obligations_per_selection: usize,
    pub urgency_weight: f32,
    pub salience_weight: f32,
    pub freshness_weight: f32,
    pub tension_balance_weight: f32,
    pub dependency_weight: f32,
    pub context_relevance_weight: f32,
    pub staleness_penalty_threshold: u32,  // chapters
    pub overuse_penalty_threshold: u32,    // injection count
    pub tension_balance_target: f32,       // Target tension level for current context
    pub enable_adaptive_weighting: bool,
    pub enable_dependency_resolution: bool,
    pub enable_contextual_filtering: bool,
}

impl Default for ObliSelectSettings {
    fn default() -> Self {
        Self {
            max_obligations_per_selection: 8,
            urgency_weight: 0.25,
            salience_weight: 0.20,
            freshness_weight: 0.15,
            tension_balance_weight: 0.20,
            dependency_weight: 0.10,
            context_relevance_weight: 0.10,
            staleness_penalty_threshold: 5,
            overuse_penalty_threshold: 10,
            tension_balance_target: 0.0,
            enable_adaptive_weighting: true,
            enable_dependency_resolution: true,
            enable_contextual_filtering: true,
        }
    }
}

/// Core ObliSelect smart obligation management system
pub struct SmartObligationManager {
    obligations: HashMap<String, Obligation>,
    settings: ObliSelectSettings,
    current_chapter: u32,
    recent_characters: Vec<String>,
    current_tension_level: f32,
    narrative_context: String,
    selection_history: Vec<(DateTime<Utc>, Vec<String>)>,
    metrics: ObligationMetrics,
}

impl SmartObligationManager {
    /// Creates a new SmartObligationManager with default settings
    pub fn new() -> Self {
        Self {
            obligations: HashMap::new(),
            settings: ObliSelectSettings::default(),
            current_chapter: 1,
            recent_characters: Vec::new(),
            current_tension_level: 0.0,
            narrative_context: String::new(),
            selection_history: Vec::new(),
            metrics: ObligationMetrics {
                total_obligations: 0,
                obligations_by_category: HashMap::new(),
                obligations_by_urgency: HashMap::new(),
                average_injection_count: 0.0,
                stale_obligations: 0,
                overused_obligations: 0,
                tension_distribution: (0.0, 0.0, 0.0),
                fulfillment_progress_average: 0.0,
                dependency_chain_length_max: 0,
                last_selection_performance_ms: 0,
            },
        }
    }

    /// Creates a new SmartObligationManager with custom settings
    pub fn with_settings(settings: ObliSelectSettings) -> Self {
        let mut manager = Self::new();
        manager.settings = settings;
        manager
    }

    /// Adds a new obligation to the management system
    pub fn add_obligation(&mut self, obligation: Obligation) {
        self.obligations.insert(obligation.id.clone(), obligation);
        self.update_metrics();
    }

    /// Removes an obligation from the system (when fully fulfilled)
    pub fn remove_obligation(&mut self, obligation_id: &str) -> Option<Obligation> {
        let removed = self.obligations.remove(obligation_id);
        if removed.is_some() {
            self.update_metrics();
        }
        removed
    }

    /// Updates an existing obligation's properties
    pub fn update_obligation(&mut self, obligation_id: &str, updater: impl FnOnce(&mut Obligation)) -> bool {
        if let Some(obligation) = self.obligations.get_mut(obligation_id) {
            updater(obligation);
            self.update_metrics();
            true
        } else {
            false
        }
    }

    /// Updates the narrative context for contextual obligation selection
    pub fn update_context(&mut self, chapter: u32, recent_characters: Vec<String>, tension_level: f32, narrative_context: String) {
        self.current_chapter = chapter;
        self.recent_characters = recent_characters;
        self.current_tension_level = tension_level;
        self.narrative_context = narrative_context;

        if self.settings.enable_adaptive_weighting {
            self.adapt_weighting_for_context();
        }
    }

    /// Selects the best obligations for injection based on current context and scoring
    pub fn select_obligations(&mut self, max_count: Option<usize>) -> Vec<ObligationScore> {
        let start_time = std::time::Instant::now();
        let max_obligations = max_count.unwrap_or(self.settings.max_obligations_per_selection);

        let mut scored_obligations: Vec<ObligationScore> = self.obligations
            .values()
            .map(|obligation| self.score_obligation(obligation))
            .collect();

        // Sort by total score (descending)
        scored_obligations.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

        // Apply dependency resolution if enabled
        if self.settings.enable_dependency_resolution {
            scored_obligations = self.resolve_dependencies(scored_obligations);
        }

        // Apply contextual filtering if enabled
        if self.settings.enable_contextual_filtering {
            scored_obligations = self.apply_contextual_filtering(scored_obligations);
        }

        // Limit to max count
        scored_obligations.truncate(max_obligations);

        // Update injection metadata for selected obligations
        let selected_ids: Vec<String> = scored_obligations.iter().map(|s| s.obligation_id.clone()).collect();
        for id in &selected_ids {
            if let Some(obligation) = self.obligations.get_mut(id) {
                obligation.last_injection = Some(Utc::now());
                obligation.injection_count += 1;
            }
        }

        // Record selection in history
        self.selection_history.push((Utc::now(), selected_ids));

        // Update performance metrics
        self.metrics.last_selection_performance_ms = start_time.elapsed().as_millis() as u64;
        self.update_metrics();

        scored_obligations
    }

    /// Scores a single obligation based on multiple factors
    fn score_obligation(&self, obligation: &Obligation) -> ObligationScore {
        let urgency_score = self.calculate_urgency_score(obligation);
        let salience_score = self.calculate_salience_score(obligation);
        let freshness_score = self.calculate_freshness_score(obligation);
        let tension_balance_score = self.calculate_tension_balance_score(obligation);
        let dependency_score = self.calculate_dependency_score(obligation);
        let context_relevance_score = self.calculate_context_relevance_score(obligation);

        let total_score =
            urgency_score * self.settings.urgency_weight +
            salience_score * self.settings.salience_weight +
            freshness_score * self.settings.freshness_weight +
            tension_balance_score * self.settings.tension_balance_weight +
            dependency_score * self.settings.dependency_weight +
            context_relevance_score * self.settings.context_relevance_weight;

        let justification = self.generate_score_justification(
            obligation, urgency_score, salience_score, freshness_score,
            tension_balance_score, dependency_score, context_relevance_score
        );

        ObligationScore {
            obligation_id: obligation.id.clone(),
            total_score,
            urgency_score,
            salience_score,
            freshness_score,
            tension_balance_score,
            dependency_score,
            context_relevance_score,
            justification,
        }
    }

    /// Calculates urgency score based on obligation urgency level and chapter progression
    fn calculate_urgency_score(&self, obligation: &Obligation) -> f32 {
        let base_urgency = match obligation.urgency {
            ObligationUrgency::Critical => 1.0,
            ObligationUrgency::High => 0.75,
            ObligationUrgency::Medium => 0.5,
            ObligationUrgency::Low => 0.25,
        };

        // Increase urgency based on how long it's been since introduction
        let chapters_since_introduction = self.current_chapter.saturating_sub(obligation.chapter_introduced);
        let staleness_multiplier = 1.0 + (chapters_since_introduction as f32 * 0.1);

        (base_urgency * staleness_multiplier).min(1.0)
    }

    /// Calculates salience score based on manual boost and narrative importance
    fn calculate_salience_score(&self, obligation: &Obligation) -> f32 {
        let base_salience = obligation.salience_boost.clamp(0.0, 1.0);

        // Boost salience for unfulfilled obligations
        let fulfillment_penalty = 1.0 - obligation.fulfillment_progress;

        (base_salience + fulfillment_penalty * 0.3).clamp(0.0, 1.0)
    }

    /// Calculates freshness score based on recent injection history
    fn calculate_freshness_score(&self, obligation: &Obligation) -> f32 {
        if let Some(last_injection) = obligation.last_injection {
            let time_since_injection = Utc::now().signed_duration_since(last_injection);
            let hours_since = time_since_injection.num_hours() as f32;

            // Fresh if not injected recently, stale if overused
            let time_freshness = (hours_since / 24.0).min(1.0); // Max freshness after 24 hours
            let usage_penalty = if obligation.injection_count > self.settings.overuse_penalty_threshold {
                0.5
            } else {
                1.0
            };

            time_freshness * usage_penalty
        } else {
            1.0 // Never injected = maximum freshness
        }
    }

    /// Calculates tension balance score based on current narrative tension needs
    fn calculate_tension_balance_score(&self, obligation: &Obligation) -> f32 {
        let tension_diff = obligation.tension_vector - self.settings.tension_balance_target;
        let current_tension_diff = self.current_tension_level - self.settings.tension_balance_target;

        // Higher score if obligation helps balance tension toward target
        if (tension_diff > 0.0 && current_tension_diff < 0.0) ||
           (tension_diff < 0.0 && current_tension_diff > 0.0) {
            1.0 - tension_diff.abs().min(1.0)
        } else {
            0.5 // Neutral if doesn't help balance
        }
    }

    /// Calculates dependency score based on prerequisite fulfillment
    fn calculate_dependency_score(&self, obligation: &Obligation) -> f32 {
        if obligation.dependencies.is_empty() {
            1.0 // No dependencies = maximum score
        } else {
            let fulfilled_dependencies = obligation.dependencies.iter()
                .filter(|dep_id| {
                    self.obligations.get(*dep_id)
                        .map(|dep| dep.fulfillment_progress >= 0.8)
                        .unwrap_or(true) // Assume fulfilled if dependency not found
                })
                .count();

            fulfilled_dependencies as f32 / obligation.dependencies.len() as f32
        }
    }

    /// Calculates context relevance score based on character involvement and narrative context
    fn calculate_context_relevance_score(&self, obligation: &Obligation) -> f32 {
        let mut relevance = 0.5; // Base relevance

        // Boost if obligation involves recently active characters
        for character in &obligation.characters_involved {
            if self.recent_characters.contains(character) {
                relevance += 0.2;
            }
        }

        // Boost if obligation content relates to current narrative context
        if !self.narrative_context.is_empty() {
            let context_words: Vec<&str> = self.narrative_context.to_lowercase().split_whitespace().collect();
            let obligation_words: Vec<&str> = obligation.content.to_lowercase().split_whitespace().collect();

            let common_words = context_words.iter()
                .filter(|word| obligation_words.contains(word))
                .count();

            if common_words > 0 {
                relevance += (common_words as f32 / context_words.len() as f32) * 0.3;
            }
        }

        relevance.clamp(0.0, 1.0)
    }

    /// Adapts scoring weights based on current narrative context
    fn adapt_weighting_for_context(&mut self) {
        // Increase urgency weight if many critical obligations exist
        let critical_count = self.obligations.values()
            .filter(|o| o.urgency == ObligationUrgency::Critical)
            .count();

        if critical_count > 3 {
            self.settings.urgency_weight = (self.settings.urgency_weight * 1.2).min(0.4);
        }

        // Increase tension balance weight if tension is far from target
        let tension_deviation = (self.current_tension_level - self.settings.tension_balance_target).abs();
        if tension_deviation > 0.5 {
            self.settings.tension_balance_weight = (self.settings.tension_balance_weight * 1.3).min(0.3);
        }

        // Normalize weights to ensure they sum to 1.0
        let total_weight = self.settings.urgency_weight + self.settings.salience_weight +
                          self.settings.freshness_weight + self.settings.tension_balance_weight +
                          self.settings.dependency_weight + self.settings.context_relevance_weight;

        if total_weight > 0.0 {
            self.settings.urgency_weight /= total_weight;
            self.settings.salience_weight /= total_weight;
            self.settings.freshness_weight /= total_weight;
            self.settings.tension_balance_weight /= total_weight;
            self.settings.dependency_weight /= total_weight;
            self.settings.context_relevance_weight /= total_weight;
        }
    }

    /// Resolves dependency chains to ensure prerequisite obligations are selected first
    fn resolve_dependencies(&self, mut scored_obligations: Vec<ObligationScore>) -> Vec<ObligationScore> {
        let mut resolved = Vec::new();
        let mut processed_ids = std::collections::HashSet::new();

        // First pass: add obligations with no dependencies or all dependencies fulfilled
        for score in &scored_obligations {
            if let Some(obligation) = self.obligations.get(&score.obligation_id) {
                if obligation.dependencies.is_empty() ||
                   obligation.dependencies.iter().all(|dep| processed_ids.contains(dep)) {
                    resolved.push(score.clone());
                    processed_ids.insert(score.obligation_id.clone());
                }
            }
        }

        // Remove resolved obligations from the original list
        scored_obligations.retain(|s| !processed_ids.contains(&s.obligation_id));

        // Add remaining obligations (dependency resolution may be incomplete)
        resolved.extend(scored_obligations);

        resolved
    }

    /// Applies contextual filtering to remove obligations that don't fit current context
    fn apply_contextual_filtering(&self, obligations: Vec<ObligationScore>) -> Vec<ObligationScore> {
        obligations.into_iter()
            .filter(|score| {
                if let Some(obligation) = self.obligations.get(&score.obligation_id) {
                    // Filter out obligations that are too far from current context
                    if score.context_relevance_score < 0.3 {
                        return false;
                    }

                    // Filter out overused obligations unless they're critical
                    if obligation.injection_count > self.settings.overuse_penalty_threshold &&
                       obligation.urgency != ObligationUrgency::Critical {
                        return false;
                    }

                    true
                } else {
                    false
                }
            })
            .collect()
    }

    /// Generates human-readable justification for obligation scoring
    fn generate_score_justification(&self, obligation: &Obligation, urgency: f32, salience: f32,
                                  freshness: f32, tension: f32, dependency: f32, context: f32) -> String {
        let mut reasons = Vec::new();

        if urgency > 0.8 {
            reasons.push(format!("High urgency ({:.1})", urgency));
        }
        if salience > 0.7 {
            reasons.push(format!("Strong salience ({:.1})", salience));
        }
        if freshness > 0.8 {
            reasons.push("Fresh (not recently used)".to_string());
        } else if freshness < 0.3 {
            reasons.push("Overused recently".to_string());
        }
        if tension > 0.7 {
            reasons.push("Helps balance tension".to_string());
        }
        if dependency < 0.5 {
            reasons.push("Blocked by dependencies".to_string());
        }
        if context > 0.7 {
            reasons.push("Highly relevant to current context".to_string());
        }

        if reasons.is_empty() {
            "Standard obligation selection".to_string()
        } else {
            reasons.join(", ")
        }
    }

    /// Updates comprehensive metrics for obligation management performance
    fn update_metrics(&mut self) {
        self.metrics.total_obligations = self.obligations.len();

        // Category and urgency distributions
        self.metrics.obligations_by_category.clear();
        self.metrics.obligations_by_urgency.clear();

        let mut total_injections = 0;
        let mut total_fulfillment = 0.0;
        let mut stale_count = 0;
        let mut overused_count = 0;
        let mut tension_negative = 0;
        let mut tension_neutral = 0;
        let mut tension_positive = 0;

        for obligation in self.obligations.values() {
            // Category distribution
            *self.metrics.obligations_by_category.entry(obligation.category).or_insert(0) += 1;

            // Urgency distribution
            *self.metrics.obligations_by_urgency.entry(obligation.urgency).or_insert(0) += 1;

            // Injection and fulfillment metrics
            total_injections += obligation.injection_count;
            total_fulfillment += obligation.fulfillment_progress;

            // Stale obligations (not injected in staleness_penalty_threshold chapters)
            if let Some(last_injection) = obligation.last_injection {
                let chapters_since = self.current_chapter.saturating_sub(obligation.chapter_introduced);
                if chapters_since > self.settings.staleness_penalty_threshold {
                    stale_count += 1;
                }
            } else if self.current_chapter.saturating_sub(obligation.chapter_introduced) > self.settings.staleness_penalty_threshold {
                stale_count += 1;
            }

            // Overused obligations
            if obligation.injection_count > self.settings.overuse_penalty_threshold {
                overused_count += 1;
            }

            // Tension distribution
            if obligation.tension_vector < -0.1 {
                tension_negative += 1;
            } else if obligation.tension_vector > 0.1 {
                tension_positive += 1;
            } else {
                tension_neutral += 1;
            }
        }

        self.metrics.average_injection_count = if self.obligations.is_empty() {
            0.0
        } else {
            total_injections as f32 / self.obligations.len() as f32
        };

        self.metrics.fulfillment_progress_average = if self.obligations.is_empty() {
            0.0
        } else {
            total_fulfillment / self.obligations.len() as f32
        };

        self.metrics.stale_obligations = stale_count;
        self.metrics.overused_obligations = overused_count;

        let total_obligations = self.obligations.len() as f32;
        self.metrics.tension_distribution = if total_obligations > 0.0 {
            (
                tension_negative as f32 / total_obligations,
                tension_neutral as f32 / total_obligations,
                tension_positive as f32 / total_obligations,
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        // Calculate maximum dependency chain length
        self.metrics.dependency_chain_length_max = self.calculate_max_dependency_chain_length();
    }

    /// Calculates the maximum dependency chain length in the obligation graph
    fn calculate_max_dependency_chain_length(&self) -> usize {
        let mut max_length = 0;

        for obligation in self.obligations.values() {
            let length = self.calculate_dependency_chain_length(&obligation.id, &mut std::collections::HashSet::new());
            max_length = max_length.max(length);
        }

        max_length
    }

    /// Recursively calculates dependency chain length for a specific obligation
    fn calculate_dependency_chain_length(&self, obligation_id: &str, visited: &mut std::collections::HashSet<String>) -> usize {
        if visited.contains(obligation_id) {
            return 0; // Circular dependency detected, break the loop
        }

        visited.insert(obligation_id.to_string());

        if let Some(obligation) = self.obligations.get(obligation_id) {
            if obligation.dependencies.is_empty() {
                1
            } else {
                let max_child_length = obligation.dependencies.iter()
                    .map(|dep_id| self.calculate_dependency_chain_length(dep_id, visited))
                    .max()
                    .unwrap_or(0);
                1 + max_child_length
            }
        } else {
            0
        }
    }

    /// Gets current obligation management metrics
    pub fn get_metrics(&self) -> &ObligationMetrics {
        &self.metrics
    }

    /// Gets current settings
    pub fn get_settings(&self) -> &ObliSelectSettings {
        &self.settings
    }

    /// Updates settings
    pub fn update_settings(&mut self, settings: ObliSelectSettings) {
        self.settings = settings;
    }

    /// Gets all obligations (for debugging/inspection)
    pub fn get_all_obligations(&self) -> &HashMap<String, Obligation> {
        &self.obligations
    }

    /// Marks an obligation as partially or fully fulfilled
    pub fn update_fulfillment_progress(&mut self, obligation_id: &str, progress: f32) -> bool {
        if let Some(obligation) = self.obligations.get_mut(obligation_id) {
            obligation.fulfillment_progress = progress.clamp(0.0, 1.0);

            // If fully fulfilled, consider removing the obligation
            if progress >= 1.0 {
                // Could auto-remove here, but leaving it to manual removal for safety
            }

            self.update_metrics();
            true
        } else {
            false
        }
    }

    /// Gets obligations that haven't been injected recently (for staleness monitoring)
    pub fn get_stale_obligations(&self, threshold_chapters: u32) -> Vec<&Obligation> {
        self.obligations.values()
            .filter(|obligation| {
                if let Some(last_injection) = obligation.last_injection {
                    let chapters_since = self.current_chapter.saturating_sub(obligation.chapter_introduced);
                    chapters_since > threshold_chapters
                } else {
                    let chapters_since = self.current_chapter.saturating_sub(obligation.chapter_introduced);
                    chapters_since > threshold_chapters
                }
            })
            .collect()
    }

    /// Gets obligations that have been overused (injected too frequently)
    pub fn get_overused_obligations(&self, threshold_count: u32) -> Vec<&Obligation> {
        self.obligations.values()
            .filter(|obligation| obligation.injection_count > threshold_count)
            .collect()
    }

    /// Resets injection statistics (for testing or system resets)
    pub fn reset_injection_stats(&mut self) {
        for obligation in self.obligations.values_mut() {
            obligation.injection_count = 0;
            obligation.last_injection = None;
        }
        self.selection_history.clear();
        self.update_metrics();
    }
}