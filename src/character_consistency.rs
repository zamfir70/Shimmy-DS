/// 👥 Character Consistency Engine
///
/// Tracks character evolution and consistency with recursive state tracking.
/// This system monitors:
/// - Personality consistency across chapters
/// - Character-specific dialogue patterns
/// - Relationship evolution between characters
/// - Character arc progression and development
///
/// Prevents character inconsistencies while allowing for authentic growth.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Represents a character's core personality trait
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub name: String,
    pub description: String,
    pub intensity: f32, // 0.0 to 1.0
    pub stability: f32, // How resistant to change (0.0 = very fluid, 1.0 = very stable)
    pub manifestations: Vec<String>, // How this trait typically shows up
    pub contradictions: Vec<String>, // What behaviors would contradict this trait
    pub first_established: DateTime<Utc>,
    pub last_reinforced: DateTime<Utc>,
}

/// Tracks a character's dialogue patterns and voice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguePattern {
    pub vocabulary_level: String, // "formal", "casual", "technical", etc.
    pub sentence_structure: String, // "complex", "simple", "fragmented", etc.
    pub favorite_phrases: Vec<String>,
    pub speech_quirks: Vec<String>, // "uses 'like' frequently", "speaks in questions", etc.
    pub emotional_tells: HashMap<String, String>, // emotion -> how they speak when feeling it
    pub cultural_markers: Vec<String>, // accent, slang, cultural references
    pub consistency_score: f32, // How consistent their voice has been
}

/// Represents a relationship between two characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub character_a: String,
    pub character_b: String,
    pub relationship_type: RelationshipType,
    pub trust_level: f32, // -1.0 (complete distrust) to 1.0 (complete trust)
    pub emotional_intimacy: f32, // 0.0 (strangers) to 1.0 (soul mates)
    pub power_dynamic: f32, // -1.0 (B dominates A) to 1.0 (A dominates B)
    pub history_markers: Vec<RelationshipEvent>,
    pub current_tension: f32, // 0.0 (peaceful) to 1.0 (explosive)
    pub established_at: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
}

/// Types of relationships between characters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    Romantic,
    Familial,
    Friendship,
    Mentorship,
    Rivalry,
    Professional,
    Antagonistic,
    Unknown,
}

/// Significant event in a relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEvent {
    pub description: String,
    pub impact_on_trust: f32,
    pub impact_on_intimacy: f32,
    pub impact_on_power: f32,
    pub chapter: u32,
    pub scene: Option<u32>,
    pub timestamp: DateTime<Utc>,
}

/// Tracks a character's progression through their arc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArc {
    pub character_name: String,
    pub arc_theme: String, // "redemption", "coming_of_age", "fall_from_grace", etc.
    pub starting_state: String,
    pub desired_end_state: String,
    pub current_progress: f32, // 0.0 to 1.0
    pub arc_milestones: Vec<ArcMilestone>,
    pub obstacles_faced: Vec<String>,
    pub growth_moments: Vec<GrowthMoment>,
    pub regression_moments: Vec<RegressionMoment>,
    pub consistency_with_theme: f32, // How well actions align with arc theme
}

/// A milestone in character development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcMilestone {
    pub description: String,
    pub chapter: u32,
    pub scene: Option<u32>,
    pub progress_marker: f32, // What % of arc this represents
    pub achieved: bool,
    pub timestamp: DateTime<Utc>,
}

/// A moment of character growth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMoment {
    pub description: String,
    pub trait_affected: String,
    pub growth_direction: String, // "strengthened", "weakened", "transformed"
    pub growth_magnitude: f32,
    pub context: String,
    pub chapter: u32,
    pub timestamp: DateTime<Utc>,
}

/// A moment of character regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionMoment {
    pub description: String,
    pub trigger: String,
    pub traits_affected: Vec<String>,
    pub severity: f32, // How significant the regression
    pub temporary: bool, // Whether this is expected to be temporary
    pub chapter: u32,
    pub timestamp: DateTime<Utc>,
}

/// Complete character profile with recursive tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub name: String,
    pub role: String, // "protagonist", "antagonist", "supporting", etc.
    pub personality_traits: HashMap<String, PersonalityTrait>,
    pub dialogue_pattern: DialoguePattern,
    pub relationships: HashMap<String, String>, // character name -> relationship ID
    pub character_arc: Option<CharacterArc>,
    pub physical_description: String,
    pub background: String,
    pub motivations: Vec<String>,
    pub fears: Vec<String>,
    pub secrets: Vec<String>,
    pub introduced_chapter: u32,
    pub last_appearance: u32,
    pub consistency_score: f32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Main character consistency tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConsistencyEngine {
    /// All character profiles
    pub characters: HashMap<String, CharacterProfile>,
    /// All relationships between characters
    pub relationships: HashMap<String, CharacterRelationship>,
    /// Consistency violation history
    pub violations: Vec<ConsistencyViolation>,
    /// Current chapter context
    pub current_chapter: u32,
    /// Global consistency metrics
    pub global_consistency: f32,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// A detected consistency violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyViolation {
    pub character_name: String,
    pub violation_type: ViolationType,
    pub description: String,
    pub severity: f32, // 0.0 to 1.0
    pub chapter: u32,
    pub scene: Option<u32>,
    pub suggested_resolution: String,
    pub timestamp: DateTime<Utc>,
}

/// Types of consistency violations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViolationType {
    PersonalityContradiction,
    DialogueVoiceShift,
    RelationshipInconsistency,
    ArcRegression,
    MotivationConflict,
    UnexplainedBehaviorChange,
    ForgottenSecret,
    PhysicalInconsistency,
}

impl CharacterConsistencyEngine {
    /// Creates a new character consistency engine
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
            relationships: HashMap::new(),
            violations: Vec::new(),
            current_chapter: 1,
            global_consistency: 1.0,
            last_updated: Utc::now(),
        }
    }

    /// Adds a new character to the system
    pub fn add_character(&mut self, character: CharacterProfile) {
        self.characters.insert(character.name.clone(), character);
        self.recalculate_global_consistency();
        self.last_updated = Utc::now();
    }

    /// Updates an existing character profile
    pub fn update_character(&mut self, name: &str, updates: CharacterProfileUpdate) {
        if let Some(character) = self.characters.get_mut(name) {
            // Apply updates
            if let Some(new_traits) = updates.personality_traits {
                for (trait_name, trait_data) in new_traits {
                    character.personality_traits.insert(trait_name, trait_data);
                }
            }

            if let Some(dialogue_update) = updates.dialogue_pattern {
                character.dialogue_pattern = dialogue_update;
            }

            if let Some(arc_update) = updates.character_arc {
                character.character_arc = Some(arc_update);
            }

            character.last_updated = Utc::now();
            character.last_appearance = self.current_chapter;

            // Check for consistency violations
            self.check_character_consistency(name);
        }

        self.recalculate_global_consistency();
        self.last_updated = Utc::now();
    }

    /// Adds or updates a relationship between characters
    pub fn add_relationship(&mut self, relationship: CharacterRelationship) {
        let relationship_id = format!("{}__{}", relationship.character_a, relationship.character_b);

        // Update character profiles to reference this relationship
        if let Some(char_a) = self.characters.get_mut(&relationship.character_a) {
            char_a.relationships.insert(relationship.character_b.clone(), relationship_id.clone());
        }
        if let Some(char_b) = self.characters.get_mut(&relationship.character_b) {
            char_b.relationships.insert(relationship.character_a.clone(), relationship_id.clone());
        }

        self.relationships.insert(relationship_id, relationship);
        self.last_updated = Utc::now();
    }

    /// Records a dialogue sample for voice consistency analysis
    pub fn record_dialogue(&mut self, character_name: &str, dialogue: &str, emotional_context: &str) {
        if let Some(character) = self.characters.get_mut(character_name) {
            let voice_consistency = self.analyze_voice_consistency(character_name, dialogue);

            // Update dialogue pattern
            character.dialogue_pattern.consistency_score = voice_consistency;

            // Check for voice violations
            if voice_consistency < 0.7 {
                self.violations.push(ConsistencyViolation {
                    character_name: character_name.to_string(),
                    violation_type: ViolationType::DialogueVoiceShift,
                    description: format!("Voice consistency dropped to {:.2} in recent dialogue", voice_consistency),
                    severity: 1.0 - voice_consistency,
                    chapter: self.current_chapter,
                    scene: None,
                    suggested_resolution: "Review character's established speech patterns and adjust dialogue".to_string(),
                    timestamp: Utc::now(),
                });
            }

            character.last_updated = Utc::now();
        }

        self.last_updated = Utc::now();
    }

    /// Analyzes voice consistency for a character's dialogue
    fn analyze_voice_consistency(&self, character_name: &str, dialogue: &str) -> f32 {
        // This is a simplified implementation - real version would use NLP
        if let Some(character) = self.characters.get(character_name) {
            let pattern = &character.dialogue_pattern;
            let mut consistency_score = 1.0;

            // Check vocabulary level consistency
            let dialogue_lower = dialogue.to_lowercase();
            let complex_words = dialogue_lower.split_whitespace()
                .filter(|word| word.len() > 8)
                .count();
            let total_words = dialogue_lower.split_whitespace().count();

            let complexity_ratio = if total_words > 0 {
                complex_words as f32 / total_words as f32
            } else {
                0.0
            };

            match pattern.vocabulary_level.as_str() {
                "formal" if complexity_ratio < 0.1 => consistency_score -= 0.2,
                "casual" if complexity_ratio > 0.3 => consistency_score -= 0.2,
                _ => {}
            }

            // Check for favorite phrases
            for phrase in &pattern.favorite_phrases {
                if dialogue_lower.contains(&phrase.to_lowercase()) {
                    consistency_score += 0.1;
                }
            }

            consistency_score.max(0.0).min(1.0)
        } else {
            0.0
        }
    }

    /// Records a character action and checks for consistency
    pub fn record_action(&mut self, character_name: &str, action_description: &str, motivation: &str) {
        if let Some(_character) = self.characters.get(character_name) {
            // Check action against personality traits
            let action_consistency = self.check_action_consistency(character_name, action_description, motivation);

            if action_consistency < 0.6 {
                self.violations.push(ConsistencyViolation {
                    character_name: character_name.to_string(),
                    violation_type: ViolationType::PersonalityContradiction,
                    description: format!("Action '{}' may contradict established personality", action_description),
                    severity: 1.0 - action_consistency,
                    chapter: self.current_chapter,
                    scene: None,
                    suggested_resolution: "Consider providing character motivation or growth moment to justify this action".to_string(),
                    timestamp: Utc::now(),
                });
            }
        }

        self.last_updated = Utc::now();
    }

    /// Checks if an action is consistent with character's established personality
    fn check_action_consistency(&self, character_name: &str, action: &str, motivation: &str) -> f32 {
        if let Some(character) = self.characters.get(character_name) {
            let mut consistency_score = 0.5; // Neutral baseline

            for trait in character.personality_traits.values() {
                // Check if action aligns with trait manifestations
                for manifestation in &trait.manifestations {
                    if action.to_lowercase().contains(&manifestation.to_lowercase()) {
                        consistency_score += trait.intensity * 0.2;
                    }
                }

                // Check if action contradicts trait
                for contradiction in &trait.contradictions {
                    if action.to_lowercase().contains(&contradiction.to_lowercase()) {
                        consistency_score -= trait.intensity * trait.stability * 0.3;
                    }
                }
            }

            // Check motivation alignment
            for char_motivation in &character.motivations {
                if motivation.to_lowercase().contains(&char_motivation.to_lowercase()) {
                    consistency_score += 0.2;
                }
            }

            consistency_score.max(0.0).min(1.0)
        } else {
            0.0
        }
    }

    /// Advances to the next chapter
    pub fn advance_chapter(&mut self) {
        self.current_chapter += 1;
        self.last_updated = Utc::now();
    }

    /// Checks overall consistency for a character
    fn check_character_consistency(&mut self, character_name: &str) {
        if let Some(character) = self.characters.get(character_name) {
            let mut consistency_issues = Vec::new();

            // Check for stale traits (not reinforced recently)
            for (trait_name, trait_data) in &character.personality_traits {
                let chapters_since_reinforcement = self.current_chapter.saturating_sub(
                    character.introduced_chapter
                );

                if chapters_since_reinforcement > 5 && trait_data.stability > 0.7 {
                    consistency_issues.push(format!("Trait '{}' hasn't been reinforced in {} chapters", trait_name, chapters_since_reinforcement));
                }
            }

            // Check arc progression
            if let Some(arc) = &character.character_arc {
                if arc.current_progress < 0.1 && self.current_chapter > character.introduced_chapter + 3 {
                    consistency_issues.push("Character arc shows minimal progression after several chapters".to_string());
                }
            }

            // Generate violations for consistency issues
            for issue in consistency_issues {
                self.violations.push(ConsistencyViolation {
                    character_name: character_name.to_string(),
                    violation_type: ViolationType::ArcRegression,
                    description: issue,
                    severity: 0.4,
                    chapter: self.current_chapter,
                    scene: None,
                    suggested_resolution: "Consider adding character development moments or trait reinforcement".to_string(),
                    timestamp: Utc::now(),
                });
            }
        }
    }

    /// Recalculates global consistency score
    fn recalculate_global_consistency(&mut self) {
        if self.characters.is_empty() {
            self.global_consistency = 1.0;
            return;
        }

        let total_consistency: f32 = self.characters.values()
            .map(|c| c.consistency_score)
            .sum();

        self.global_consistency = total_consistency / self.characters.len() as f32;
    }

    /// Gets all characters with consistency issues
    pub fn get_characters_with_issues(&self) -> Vec<CharacterIssueReport> {
        let mut reports = Vec::new();

        for (name, character) in &self.characters {
            let character_violations: Vec<&ConsistencyViolation> = self.violations
                .iter()
                .filter(|v| v.character_name == *name)
                .collect();

            if !character_violations.is_empty() || character.consistency_score < 0.7 {
                reports.push(CharacterIssueReport {
                    character_name: name.clone(),
                    consistency_score: character.consistency_score,
                    violation_count: character_violations.len(),
                    most_severe_violation: character_violations
                        .iter()
                        .max_by(|a, b| a.severity.partial_cmp(&b.severity).unwrap_or(std::cmp::Ordering::Equal))
                        .map(|v| v.description.clone()),
                    recommendations: self.generate_character_recommendations(name, &character_violations),
                });
            }
        }

        reports.sort_by(|a, b| a.consistency_score.partial_cmp(&b.consistency_score).unwrap_or(std::cmp::Ordering::Equal));
        reports
    }

    /// Generates recommendations for improving character consistency
    fn generate_character_recommendations(&self, character_name: &str, violations: &[&ConsistencyViolation]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(character) = self.characters.get(character_name) {
            // Analyze violation patterns
            let personality_violations = violations.iter().filter(|v| v.violation_type == ViolationType::PersonalityContradiction).count();
            let dialogue_violations = violations.iter().filter(|v| v.violation_type == ViolationType::DialogueVoiceShift).count();
            let arc_violations = violations.iter().filter(|v| v.violation_type == ViolationType::ArcRegression).count();

            if personality_violations > 0 {
                recommendations.push("Review character's core personality traits and ensure actions align with established patterns".to_string());
            }

            if dialogue_violations > 0 {
                recommendations.push("Strengthen dialogue voice consistency by reviewing speech patterns and favorite phrases".to_string());
            }

            if arc_violations > 0 {
                recommendations.push("Consider adding character development moments to advance their arc progression".to_string());
            }

            if character.consistency_score < 0.5 {
                recommendations.push("Character may need significant attention to restore consistency".to_string());
            }

            if character.last_appearance < self.current_chapter - 3 {
                recommendations.push("Character hasn't appeared recently - consider reintroducing them to maintain presence".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Continue monitoring character consistency".to_string());
        }

        recommendations
    }

    /// Generates a comprehensive character consistency report
    pub fn generate_consistency_report(&self) -> String {
        let mut report = String::new();

        report.push_str("👥 CHARACTER CONSISTENCY ANALYSIS\n");
        report.push_str("==================================\n\n");

        report.push_str(&format!("Current Chapter: {}\n", self.current_chapter));
        report.push_str(&format!("Total Characters: {}\n", self.characters.len()));
        report.push_str(&format!("Total Relationships: {}\n", self.relationships.len()));
        report.push_str(&format!("Global Consistency: {:.2}\n", self.global_consistency));
        report.push_str(&format!("Total Violations: {}\n\n", self.violations.len()));

        // Character overview
        report.push_str("📊 Character Overview:\n");
        for (name, character) in &self.characters {
            report.push_str(&format!("  • {}: {:.2} consistency, {} traits, last seen ch. {}\n",
                name,
                character.consistency_score,
                character.personality_traits.len(),
                character.last_appearance));
        }
        report.push('\n');

        // Recent violations
        if !self.violations.is_empty() {
            report.push_str("⚠️ Recent Violations:\n");
            let recent_violations: Vec<&ConsistencyViolation> = self.violations
                .iter()
                .filter(|v| v.chapter >= self.current_chapter.saturating_sub(2))
                .collect();

            for violation in recent_violations.iter().take(5) {
                report.push_str(&format!("  • {}: {} (severity: {:.2})\n",
                    violation.character_name,
                    violation.description,
                    violation.severity));
            }
            report.push('\n');
        }

        // Characters with issues
        let issues = self.get_characters_with_issues();
        if !issues.is_empty() {
            report.push_str("🔧 Characters Needing Attention:\n");
            for issue in issues.iter().take(5) {
                report.push_str(&format!("  • {}: {:.2} consistency, {} violations\n",
                    issue.character_name,
                    issue.consistency_score,
                    issue.violation_count));
            }
            report.push('\n');
        }

        // Global recommendations
        let avg_consistency = if !self.characters.is_empty() {
            self.characters.values().map(|c| c.consistency_score).sum::<f32>() / self.characters.len() as f32
        } else {
            1.0
        };

        report.push_str("💡 Global Recommendations:\n");
        if avg_consistency < 0.7 {
            report.push_str("  • Overall character consistency needs improvement\n");
        }
        if self.violations.len() > 10 {
            report.push_str("  • High violation count - consider reviewing character development approach\n");
        }
        if self.characters.len() > 8 && avg_consistency > 0.8 {
            report.push_str("  • Good consistency management with large cast\n");
        }

        report
    }
}

/// Update structure for character profiles
#[derive(Debug, Clone)]
pub struct CharacterProfileUpdate {
    pub personality_traits: Option<HashMap<String, PersonalityTrait>>,
    pub dialogue_pattern: Option<DialoguePattern>,
    pub character_arc: Option<CharacterArc>,
    pub motivations: Option<Vec<String>>,
    pub fears: Option<Vec<String>>,
    pub secrets: Option<Vec<String>>,
}

/// Report of character consistency issues
#[derive(Debug, Clone)]
pub struct CharacterIssueReport {
    pub character_name: String,
    pub consistency_score: f32,
    pub violation_count: usize,
    pub most_severe_violation: Option<String>,
    pub recommendations: Vec<String>,
}

impl Default for CharacterConsistencyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_consistency_engine_creation() {
        let engine = CharacterConsistencyEngine::new();
        assert_eq!(engine.current_chapter, 1);
        assert!(engine.characters.is_empty());
        assert_eq!(engine.global_consistency, 1.0);
    }

    #[test]
    fn test_add_character() {
        let mut engine = CharacterConsistencyEngine::new();

        let character = CharacterProfile {
            name: "Hero".to_string(),
            role: "protagonist".to_string(),
            personality_traits: HashMap::new(),
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec!["you know".to_string()],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 1.0,
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: "Tall and lean".to_string(),
            background: "Grew up in small town".to_string(),
            motivations: vec!["save the village".to_string()],
            fears: vec!["failure".to_string()],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 1.0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        assert_eq!(engine.characters.len(), 1);
        assert!(engine.characters.contains_key("Hero"));
    }

    #[test]
    fn test_personality_trait() {
        let trait_data = PersonalityTrait {
            name: "Brave".to_string(),
            description: "Faces danger without hesitation".to_string(),
            intensity: 0.8,
            stability: 0.9,
            manifestations: vec!["stands up to bullies".to_string(), "volunteers for dangerous missions".to_string()],
            contradictions: vec!["runs away from conflict".to_string(), "avoids responsibility".to_string()],
            first_established: Utc::now(),
            last_reinforced: Utc::now(),
        };

        assert_eq!(trait_data.name, "Brave");
        assert_eq!(trait_data.intensity, 0.8);
        assert_eq!(trait_data.manifestations.len(), 2);
    }

    #[test]
    fn test_dialogue_consistency_analysis() {
        let mut engine = CharacterConsistencyEngine::new();

        let mut character = CharacterProfile {
            name: "Scholar".to_string(),
            role: "supporting".to_string(),
            personality_traits: HashMap::new(),
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "formal".to_string(),
                sentence_structure: "complex".to_string(),
                favorite_phrases: vec!["indeed".to_string(), "furthermore".to_string()],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 1.0,
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec![],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 1.0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        // Test consistent dialogue
        let consistent_dialogue = "Indeed, the philosophical ramifications are quite extraordinary, furthermore...";
        let consistency = engine.analyze_voice_consistency("Scholar", consistent_dialogue);
        assert!(consistency > 0.8);

        // Test inconsistent dialogue
        let inconsistent_dialogue = "Yeah, whatever dude, like totally.";
        let inconsistency = engine.analyze_voice_consistency("Scholar", inconsistent_dialogue);
        assert!(inconsistency < 0.6);
    }

    #[test]
    fn test_action_consistency_check() {
        let mut engine = CharacterConsistencyEngine::new();

        let mut traits = HashMap::new();
        traits.insert("brave".to_string(), PersonalityTrait {
            name: "Brave".to_string(),
            description: "Courageous in face of danger".to_string(),
            intensity: 0.9,
            stability: 0.8,
            manifestations: vec!["fights monsters".to_string(), "protects others".to_string()],
            contradictions: vec!["runs away".to_string(), "hides".to_string()],
            first_established: Utc::now(),
            last_reinforced: Utc::now(),
        });

        let character = CharacterProfile {
            name: "Warrior".to_string(),
            role: "protagonist".to_string(),
            personality_traits: traits,
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec![],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 1.0,
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec!["protect innocent".to_string()],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 1.0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        // Test consistent action
        let consistent_action = "fights the dragon to protect the village";
        let consistency = engine.check_action_consistency("Warrior", consistent_action, "protect innocent");
        assert!(consistency > 0.7);

        // Test inconsistent action
        let inconsistent_action = "runs away and hides in the forest";
        let inconsistency = engine.check_action_consistency("Warrior", inconsistent_action, "self-preservation");
        assert!(inconsistency < 0.5);
    }

    #[test]
    fn test_relationship_management() {
        let mut engine = CharacterConsistencyEngine::new();

        let relationship = CharacterRelationship {
            character_a: "Hero".to_string(),
            character_b: "Mentor".to_string(),
            relationship_type: RelationshipType::Mentorship,
            trust_level: 0.8,
            emotional_intimacy: 0.6,
            power_dynamic: -0.3, // Mentor dominates
            history_markers: vec![],
            current_tension: 0.1,
            established_at: Utc::now(),
            last_interaction: Utc::now(),
        };

        engine.add_relationship(relationship);

        assert_eq!(engine.relationships.len(), 1);
        assert!(engine.relationships.contains_key("Hero__Mentor"));
    }

    #[test]
    fn test_consistency_violation_recording() {
        let mut engine = CharacterConsistencyEngine::new();

        let character = CharacterProfile {
            name: "TestChar".to_string(),
            role: "test".to_string(),
            personality_traits: HashMap::new(),
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "formal".to_string(),
                sentence_structure: "complex".to_string(),
                favorite_phrases: vec![],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.5, // Low consistency to trigger violations
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec![],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 0.5,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        // Record dialogue that should trigger a violation
        engine.record_dialogue("TestChar", "yo what's up", "casual");

        assert!(!engine.violations.is_empty());
        assert_eq!(engine.violations[0].violation_type, ViolationType::DialogueVoiceShift);
    }

    #[test]
    fn test_character_issue_reporting() {
        let mut engine = CharacterConsistencyEngine::new();

        // Add a character with low consistency
        let character = CharacterProfile {
            name: "ProblemChar".to_string(),
            role: "test".to_string(),
            personality_traits: HashMap::new(),
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec![],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.6,
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec![],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 0.6, // Below threshold
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        let issues = engine.get_characters_with_issues();
        assert!(!issues.is_empty());
        assert_eq!(issues[0].character_name, "ProblemChar");
    }

    #[test]
    fn test_advance_chapter() {
        let mut engine = CharacterConsistencyEngine::new();
        assert_eq!(engine.current_chapter, 1);

        engine.advance_chapter();
        assert_eq!(engine.current_chapter, 2);
    }

    #[test]
    fn test_generate_consistency_report() {
        let mut engine = CharacterConsistencyEngine::new();

        let character = CharacterProfile {
            name: "ReportChar".to_string(),
            role: "test".to_string(),
            personality_traits: HashMap::new(),
            dialogue_pattern: DialoguePattern {
                vocabulary_level: "casual".to_string(),
                sentence_structure: "simple".to_string(),
                favorite_phrases: vec![],
                speech_quirks: vec![],
                emotional_tells: HashMap::new(),
                cultural_markers: vec![],
                consistency_score: 0.8,
            },
            relationships: HashMap::new(),
            character_arc: None,
            physical_description: String::new(),
            background: String::new(),
            motivations: vec![],
            fears: vec![],
            secrets: vec![],
            introduced_chapter: 1,
            last_appearance: 1,
            consistency_score: 0.8,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        engine.add_character(character);

        let report = engine.generate_consistency_report();
        assert!(report.contains("CHARACTER CONSISTENCY ANALYSIS"));
        assert!(report.contains("ReportChar"));
        assert!(report.contains("Global Consistency"));
    }
}