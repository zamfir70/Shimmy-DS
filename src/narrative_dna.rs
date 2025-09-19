/// 🧬 CAPR Narrative DNA Tracking System
///
/// Implements the minimal recursive rules for narrative coherence:
/// - Contradiction: Initial story tension/character conflict
/// - Action: Attempts to resolve or escape contradiction
/// - Pressure: Unintended consequences that raise stakes
/// - Return: Recursive moments where elements come back transformed
///
/// This system tracks how meaning propagates recursively through the narrative
/// without dictating plot templates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// The four base units of narrative DNA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NarrativeDNAUnit {
    /// Character wants X but fears Y, or story establishes fundamental tension
    Contradiction {
        id: String,
        description: String,
        intensity: f32,
        characters_involved: Vec<String>,
        created_at: DateTime<Utc>,
        resolved: bool,
    },
    /// Attempt to resolve or escape a contradiction
    Action {
        id: String,
        description: String,
        resolves_contradiction: Option<String>, // References contradiction ID
        intensity: f32,
        created_at: DateTime<Utc>,
        consequences: Vec<String>, // References to pressure units created
    },
    /// Unintended consequence that raises stakes or creates new tension
    Pressure {
        id: String,
        description: String,
        caused_by_action: Option<String>, // References action ID
        intensity: f32,
        affects_characters: Vec<String>,
        created_at: DateTime<Utc>,
        resolved: bool,
    },
    /// Moment when something returns changed, providing recursive meaning
    Return {
        id: String,
        description: String,
        references_unit: String, // ID of unit being recursively referenced
        transformation_type: TransformationType,
        emotional_delta: f32, // How much emotional context changed
        created_at: DateTime<Utc>,
    },
}

/// Types of transformations in Return units
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformationType {
    /// Character faces same contradiction but with new understanding
    CharacterGrowth,
    /// Same situation recurs but with different stakes
    EscalatedRepetition,
    /// Theme returns with deeper meaning
    ThematicDeepening,
    /// Location/symbol returns with new significance
    SymbolicEvolution,
    /// Past action's consequences finally manifest
    DelayedConsequence,
    /// Mirror/inverse of earlier situation
    Inversion,
}

impl NarrativeDNAUnit {
    /// Creates a new Contradiction unit
    pub fn new_contradiction(
        id: String,
        description: String,
        intensity: f32,
        characters: Vec<String>,
    ) -> Self {
        Self::Contradiction {
            id,
            description,
            intensity: intensity.clamp(0.0, 1.0),
            characters_involved: characters,
            created_at: Utc::now(),
            resolved: false,
        }
    }

    /// Creates a new Action unit
    pub fn new_action(
        id: String,
        description: String,
        resolves_contradiction: Option<String>,
        intensity: f32,
    ) -> Self {
        Self::Action {
            id,
            description,
            resolves_contradiction,
            intensity: intensity.clamp(0.0, 1.0),
            created_at: Utc::now(),
            consequences: Vec::new(),
        }
    }

    /// Creates a new Pressure unit
    pub fn new_pressure(
        id: String,
        description: String,
        caused_by_action: Option<String>,
        intensity: f32,
        affects_characters: Vec<String>,
    ) -> Self {
        Self::Pressure {
            id,
            description,
            caused_by_action,
            intensity: intensity.clamp(0.0, 1.0),
            affects_characters,
            created_at: Utc::now(),
            resolved: false,
        }
    }

    /// Creates a new Return unit
    pub fn new_return(
        id: String,
        description: String,
        references_unit: String,
        transformation_type: TransformationType,
        emotional_delta: f32,
    ) -> Self {
        Self::Return {
            id,
            description,
            references_unit,
            transformation_type,
            emotional_delta,
            created_at: Utc::now(),
        }
    }

    /// Gets the ID of this unit
    pub fn id(&self) -> &str {
        match self {
            Self::Contradiction { id, .. } => id,
            Self::Action { id, .. } => id,
            Self::Pressure { id, .. } => id,
            Self::Return { id, .. } => id,
        }
    }

    /// Gets the intensity of this unit
    pub fn intensity(&self) -> f32 {
        match self {
            Self::Contradiction { intensity, .. } => *intensity,
            Self::Action { intensity, .. } => *intensity,
            Self::Pressure { intensity, .. } => *intensity,
            Self::Return { emotional_delta, .. } => emotional_delta.abs(),
        }
    }

    /// Gets the creation timestamp
    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            Self::Contradiction { created_at, .. } => *created_at,
            Self::Action { created_at, .. } => *created_at,
            Self::Pressure { created_at, .. } => *created_at,
            Self::Return { created_at, .. } => *created_at,
        }
    }

    /// Checks if this unit is resolved (for Contradiction and Pressure types)
    pub fn is_resolved(&self) -> Option<bool> {
        match self {
            Self::Contradiction { resolved, .. } => Some(*resolved),
            Self::Pressure { resolved, .. } => Some(*resolved),
            _ => None,
        }
    }

    /// Marks a unit as resolved (if applicable)
    pub fn resolve(&mut self) {
        match self {
            Self::Contradiction { resolved, .. } => *resolved = true,
            Self::Pressure { resolved, .. } => *resolved = true,
            _ => {} // Actions and Returns don't have resolved state
        }
    }
}

/// Tracks the complete narrative DNA sequence and provides analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeDNATracker {
    /// All DNA units in chronological order
    pub units: Vec<NarrativeDNAUnit>,
    /// Mapping of IDs to unit indices for fast lookup
    pub id_index: HashMap<String, usize>,
    /// Current chapter/scene context
    pub current_chapter: u32,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl NarrativeDNATracker {
    /// Creates a new DNA tracker
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            id_index: HashMap::new(),
            current_chapter: 1,
            last_updated: Utc::now(),
        }
    }

    /// Adds a new DNA unit to the tracker
    pub fn add_unit(&mut self, unit: NarrativeDNAUnit) {
        let id = unit.id().to_string();
        self.id_index.insert(id, self.units.len());
        self.units.push(unit);
        self.last_updated = Utc::now();
    }

    /// Gets a unit by ID
    pub fn get_unit(&self, id: &str) -> Option<&NarrativeDNAUnit> {
        self.id_index.get(id).and_then(|&index| self.units.get(index))
    }

    /// Gets a mutable reference to a unit by ID
    pub fn get_unit_mut(&mut self, id: &str) -> Option<&mut NarrativeDNAUnit> {
        if let Some(&index) = self.id_index.get(id) {
            self.units.get_mut(index)
        } else {
            None
        }
    }

    /// Advances to the next chapter
    pub fn advance_chapter(&mut self) {
        self.current_chapter += 1;
        self.last_updated = Utc::now();
    }

    /// Gets all unresolved contradictions
    pub fn unresolved_contradictions(&self) -> Vec<&NarrativeDNAUnit> {
        self.units
            .iter()
            .filter(|unit| matches!(unit, NarrativeDNAUnit::Contradiction { resolved: false, .. }))
            .collect()
    }

    /// Gets all unresolved pressures
    pub fn unresolved_pressures(&self) -> Vec<&NarrativeDNAUnit> {
        self.units
            .iter()
            .filter(|unit| matches!(unit, NarrativeDNAUnit::Pressure { resolved: false, .. }))
            .collect()
    }

    /// Finds potential Return opportunities by analyzing recursive patterns
    pub fn find_return_opportunities(&self) -> Vec<ReturnOpportunity> {
        let mut opportunities = Vec::new();
        let current_time = Utc::now();

        // Look for unresolved contradictions that could return transformed
        for unit in &self.units {
            if let NarrativeDNAUnit::Contradiction {
                id, description, resolved: false, created_at, characters_involved, ..
            } = unit {
                let age_hours = (current_time - *created_at).num_hours();

                // Suggest return opportunities for contradictions that have had time to develop
                if age_hours > 2 { // More than 2 hours of story time
                    opportunities.push(ReturnOpportunity {
                        references_unit: id.clone(),
                        description: format!("Consider returning to: {}", description),
                        suggested_transformation: TransformationType::CharacterGrowth,
                        intensity_score: self.calculate_return_pressure(id),
                        characters_involved: characters_involved.clone(),
                    });
                }
            }
        }

        // Look for actions that might have delayed consequences
        for unit in &self.units {
            if let NarrativeDNAUnit::Action {
                id, description, created_at, consequences, ..
            } = unit {
                let age_hours = (current_time - *created_at).num_hours();

                // Suggest delayed consequences for significant actions
                if age_hours > 1 && consequences.is_empty() {
                    opportunities.push(ReturnOpportunity {
                        references_unit: id.clone(),
                        description: format!("Consider delayed consequence of: {}", description),
                        suggested_transformation: TransformationType::DelayedConsequence,
                        intensity_score: self.calculate_return_pressure(id),
                        characters_involved: Vec::new(),
                    });
                }
            }
        }

        // Sort by intensity score (highest pressure first)
        opportunities.sort_by(|a, b| b.intensity_score.partial_cmp(&a.intensity_score).unwrap_or(std::cmp::Ordering::Equal));
        opportunities
    }

    /// Calculates the pressure for a potential return based on unit age and intensity
    fn calculate_return_pressure(&self, unit_id: &str) -> f32 {
        if let Some(unit) = self.get_unit(unit_id) {
            let age_factor = (Utc::now() - unit.created_at()).num_hours() as f32 / 24.0; // Days since creation
            let intensity = unit.intensity();
            (intensity * (1.0 + age_factor * 0.5)).min(2.0) // Cap at 2.0
        } else {
            0.0
        }
    }

    /// Analyzes current DNA pattern health
    pub fn analyze_pattern_health(&self) -> DNAPatternHealth {
        let unresolved_contradictions = self.unresolved_contradictions().len();
        let unresolved_pressures = self.unresolved_pressures().len();

        let total_units = self.units.len();
        let return_count = self.units.iter().filter(|u| matches!(u, NarrativeDNAUnit::Return { .. })).count();

        let return_ratio = if total_units > 0 {
            return_count as f32 / total_units as f32
        } else {
            0.0
        };

        let health_score = {
            let mut score = 1.0;

            // Penalize too many unresolved units
            if unresolved_contradictions > 5 {
                score -= 0.3;
            }
            if unresolved_pressures > 8 {
                score -= 0.3;
            }

            // Penalize lack of recursive returns
            if return_ratio < 0.1 && total_units > 10 {
                score -= 0.2;
            }

            // Reward good balance
            if return_ratio >= 0.15 && return_ratio <= 0.35 {
                score += 0.1;
            }

            score.max(0.0).min(1.0)
        };

        DNAPatternHealth {
            health_score,
            unresolved_contradictions,
            unresolved_pressures,
            total_units,
            return_ratio,
            recommendations: self.generate_health_recommendations(health_score, unresolved_contradictions, unresolved_pressures, return_ratio),
        }
    }

    /// Generates recommendations based on pattern health
    fn generate_health_recommendations(&self, health_score: f32, contradictions: usize, pressures: usize, return_ratio: f32) -> Vec<String> {
        let mut recommendations = Vec::new();

        if health_score < 0.6 {
            recommendations.push("🚨 Narrative DNA health is degraded - consider resolution or return sequences".to_string());
        }

        if contradictions > 5 {
            recommendations.push(format!("⚠️ {} unresolved contradictions - consider resolving or evolving some", contradictions));
        }

        if pressures > 8 {
            recommendations.push(format!("⚠️ {} unresolved pressures - narrative may feel overwhelming", pressures));
        }

        if return_ratio < 0.1 && self.units.len() > 10 {
            recommendations.push("🔄 Low recursive return ratio - story may lack deeper meaning development".to_string());
        }

        if return_ratio > 0.4 {
            recommendations.push("🔄 High return ratio - ensure forward momentum isn't stalled".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("✅ Narrative DNA patterns appear healthy".to_string());
        }

        recommendations
    }

    /// Creates a comprehensive DNA analysis report
    pub fn generate_dna_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🧬 NARRATIVE DNA ANALYSIS REPORT\n");
        report.push_str("=================================\n\n");

        report.push_str(&format!("Current Chapter: {}\n", self.current_chapter));
        report.push_str(&format!("Total DNA Units: {}\n", self.units.len()));
        report.push_str(&format!("Last Updated: {}\n\n", self.last_updated.format("%Y-%m-%d %H:%M:%S UTC")));

        // Count each type
        let mut contradiction_count = 0;
        let mut action_count = 0;
        let mut pressure_count = 0;
        let mut return_count = 0;

        for unit in &self.units {
            match unit {
                NarrativeDNAUnit::Contradiction { .. } => contradiction_count += 1,
                NarrativeDNAUnit::Action { .. } => action_count += 1,
                NarrativeDNAUnit::Pressure { .. } => pressure_count += 1,
                NarrativeDNAUnit::Return { .. } => return_count += 1,
            }
        }

        report.push_str("🔬 DNA Unit Distribution:\n");
        report.push_str(&format!("  • Contradictions: {} ({} unresolved)\n", contradiction_count, self.unresolved_contradictions().len()));
        report.push_str(&format!("  • Actions: {}\n", action_count));
        report.push_str(&format!("  • Pressures: {} ({} unresolved)\n", pressure_count, self.unresolved_pressures().len()));
        report.push_str(&format!("  • Returns: {}\n\n", return_count));

        // Pattern health
        let health = self.analyze_pattern_health();
        report.push_str(&format!("📊 Pattern Health Score: {:.2}/1.0\n", health.health_score));
        report.push_str(&format!("📊 Return Ratio: {:.2}\n\n", health.return_ratio));

        // Recommendations
        report.push_str("💡 Recommendations:\n");
        for rec in &health.recommendations {
            report.push_str(&format!("  • {}\n", rec));
        }
        report.push('\n');

        // Return opportunities
        let opportunities = self.find_return_opportunities();
        if !opportunities.is_empty() {
            report.push_str("🔄 Return Opportunities:\n");
            for (i, opp) in opportunities.iter().take(5).enumerate() {
                report.push_str(&format!("  {}. {} (pressure: {:.2})\n", i + 1, opp.description, opp.intensity_score));
            }
            report.push('\n');
        }

        report
    }
}

/// Represents a potential return opportunity
#[derive(Debug, Clone)]
pub struct ReturnOpportunity {
    pub references_unit: String,
    pub description: String,
    pub suggested_transformation: TransformationType,
    pub intensity_score: f32,
    pub characters_involved: Vec<String>,
}

/// Analysis of narrative DNA pattern health
#[derive(Debug, Clone)]
pub struct DNAPatternHealth {
    pub health_score: f32,
    pub unresolved_contradictions: usize,
    pub unresolved_pressures: usize,
    pub total_units: usize,
    pub return_ratio: f32,
    pub recommendations: Vec<String>,
}

impl Default for NarrativeDNATracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_narrative_dna_unit_creation() {
        let contradiction = NarrativeDNAUnit::new_contradiction(
            "c1".to_string(),
            "Hero fears commitment but wants love".to_string(),
            0.8,
            vec!["hero".to_string()],
        );

        if let NarrativeDNAUnit::Contradiction { id, intensity, resolved, .. } = contradiction {
            assert_eq!(id, "c1");
            assert_eq!(intensity, 0.8);
            assert!(!resolved);
        } else {
            panic!("Expected Contradiction unit");
        }
    }

    #[test]
    fn test_dna_tracker_operations() {
        let mut tracker = NarrativeDNATracker::new();

        let contradiction = NarrativeDNAUnit::new_contradiction(
            "c1".to_string(),
            "Test contradiction".to_string(),
            0.7,
            vec!["character1".to_string()],
        );

        tracker.add_unit(contradiction);

        assert_eq!(tracker.units.len(), 1);
        assert!(tracker.get_unit("c1").is_some());
        assert_eq!(tracker.unresolved_contradictions().len(), 1);
    }

    #[test]
    fn test_unit_resolution() {
        let mut contradiction = NarrativeDNAUnit::new_contradiction(
            "c1".to_string(),
            "Test".to_string(),
            0.7,
            vec![],
        );

        assert_eq!(contradiction.is_resolved(), Some(false));
        contradiction.resolve();
        assert_eq!(contradiction.is_resolved(), Some(true));
    }

    #[test]
    fn test_pattern_health_analysis() {
        let mut tracker = NarrativeDNATracker::new();

        // Add some units
        tracker.add_unit(NarrativeDNAUnit::new_contradiction("c1".to_string(), "Test".to_string(), 0.8, vec![]));
        tracker.add_unit(NarrativeDNAUnit::new_action("a1".to_string(), "Test action".to_string(), Some("c1".to_string()), 0.6));
        tracker.add_unit(NarrativeDNAUnit::new_return("r1".to_string(), "Test return".to_string(), "c1".to_string(), TransformationType::CharacterGrowth, 0.3));

        let health = tracker.analyze_pattern_health();
        assert!(health.health_score > 0.5);
        assert_eq!(health.total_units, 3);
    }

    #[test]
    fn test_intensity_clamping() {
        let contradiction = NarrativeDNAUnit::new_contradiction(
            "c1".to_string(),
            "Test".to_string(),
            1.5, // Should be clamped to 1.0
            vec![],
        );

        assert_eq!(contradiction.intensity(), 1.0);
    }

    #[test]
    fn test_return_opportunities() {
        let mut tracker = NarrativeDNATracker::new();

        // Add an old contradiction
        let mut contradiction = NarrativeDNAUnit::new_contradiction(
            "c1".to_string(),
            "Old contradiction".to_string(),
            0.8,
            vec!["hero".to_string()],
        );

        // Manually set an older timestamp for testing
        if let NarrativeDNAUnit::Contradiction { created_at, .. } = &mut contradiction {
            *created_at = Utc::now() - chrono::Duration::hours(3);
        }

        tracker.add_unit(contradiction);

        let opportunities = tracker.find_return_opportunities();
        assert!(!opportunities.is_empty());
        assert_eq!(opportunities[0].references_unit, "c1");
    }

    #[test]
    fn test_generate_dna_report() {
        let mut tracker = NarrativeDNATracker::new();

        tracker.add_unit(NarrativeDNAUnit::new_contradiction("c1".to_string(), "Test contradiction".to_string(), 0.8, vec![]));
        tracker.add_unit(NarrativeDNAUnit::new_action("a1".to_string(), "Test action".to_string(), None, 0.6));

        let report = tracker.generate_dna_report();

        assert!(report.contains("NARRATIVE DNA ANALYSIS REPORT"));
        assert!(report.contains("Total DNA Units: 2"));
        assert!(report.contains("Contradictions: 1"));
        assert!(report.contains("Actions: 1"));
    }
}