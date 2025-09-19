/// 🔄 Multi-Level Recursion Tracker
///
/// Operates simultaneously across all narrative levels:
/// sentence → paragraph → scene → chapter → act → story
///
/// Each level can recursively affect others - a sentence might echo a theme
/// from the climax, a scene might recontextualize a throwaway line from the prologue.
/// This system maintains coherence while the author focuses on immediate creative flow.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Different levels of narrative scale
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NarrativeLevel {
    Sentence,
    Paragraph,
    Scene,
    Chapter,
    Act,
    Story,
}

/// Types of recursive relationships between narrative elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecursionType {
    /// Semantic ripple - a line reshapes paragraph intent
    SemanticRipple,
    /// Emotional cascade - tone builds scene pressure
    EmotionalCascade,
    /// Causal leverage - conflict outcome redirects arc
    CausalLeverage,
    /// Thematic escalation - scenes reinforce thesis
    ThematicEscalation,
    /// Structural symmetry - beginning/end mirror
    StructuralSymmetry,
    /// Symbolic recursion - final sentence echoes first metaphor
    SymbolicRecursion,
    /// Tonal echo - rhythm patterns repeat across scales
    TonalEcho,
    /// Motif return - elements reappear transformed
    MotifReturn,
}

/// A recursive element tracked across narrative levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveElement {
    pub id: String,
    pub content: String,
    pub level: NarrativeLevel,
    pub chapter: u32,
    pub scene: Option<u32>,
    pub paragraph: Option<u32>,
    pub sentence: Option<u32>,
    pub element_type: ElementType,
    pub intensity: f32,
    pub created_at: DateTime<Utc>,
    pub echoes: Vec<RecursiveEcho>, // Where this element has been referenced
}

/// Types of narrative elements that can create recursive patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    /// Metaphor, symbol, image
    Symbolic {
        symbol_name: String,
        meaning: String,
    },
    /// Dialogue line or phrase
    Dialogue {
        speaker: String,
        emotional_charge: f32,
    },
    /// Thematic statement or question
    Thematic {
        theme: String,
        position: String, // Story's stance on the theme
    },
    /// Character trait or behavior
    CharacterMoment {
        character: String,
        trait_revealed: String,
    },
    /// Tonal or atmospheric quality
    Atmospheric {
        mood: String,
        sensory_details: Vec<String>,
    },
    /// Structural element (pacing, transition, etc.)
    Structural {
        function: String, // "climax", "reversal", "revelation", etc.
        impact_level: f32,
    },
}

/// An echo/reference to a recursive element at a different scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveEcho {
    pub echo_id: String,
    pub original_element_id: String,
    pub echo_level: NarrativeLevel,
    pub recursion_type: RecursionType,
    pub transformation: String, // How the element changed in this echo
    pub intensity_delta: f32, // Change in emotional/thematic intensity
    pub chapter: u32,
    pub scene: Option<u32>,
    pub created_at: DateTime<Utc>,
}

/// Tracks recursive patterns across all narrative levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLevelRecursionTracker {
    /// All tracked recursive elements
    pub elements: HashMap<String, RecursiveElement>,
    /// Index by narrative level for fast lookup
    pub level_index: HashMap<NarrativeLevel, Vec<String>>,
    /// Index by element type
    pub type_index: HashMap<String, Vec<String>>, // ElementType as string -> element IDs
    /// Current narrative context
    pub current_chapter: u32,
    pub current_scene: Option<u32>,
    pub current_paragraph: Option<u32>,
    /// Recursion statistics
    pub recursion_stats: RecursionStats,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Statistics about recursive patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursionStats {
    pub total_elements: usize,
    pub total_echoes: usize,
    pub cross_level_connections: usize,
    pub recursion_density: f32, // Echoes per element
    pub thematic_coherence_score: f32,
    pub structural_symmetry_score: f32,
}

impl MultiLevelRecursionTracker {
    /// Creates a new multi-level recursion tracker
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            level_index: HashMap::new(),
            type_index: HashMap::new(),
            current_chapter: 1,
            current_scene: None,
            current_paragraph: None,
            recursion_stats: RecursionStats {
                total_elements: 0,
                total_echoes: 0,
                cross_level_connections: 0,
                recursion_density: 0.0,
                thematic_coherence_score: 0.0,
                structural_symmetry_score: 0.0,
            },
            last_updated: Utc::now(),
        }
    }

    /// Adds a new recursive element
    pub fn add_element(&mut self, element: RecursiveElement) {
        let element_id = element.id.clone();
        let level = element.level.clone();
        let element_type_key = self.element_type_to_string(&element.element_type);

        // Update indices
        self.level_index
            .entry(level)
            .or_insert_with(Vec::new)
            .push(element_id.clone());

        self.type_index
            .entry(element_type_key)
            .or_insert_with(Vec::new)
            .push(element_id.clone());

        self.elements.insert(element_id, element);
        self.update_stats();
        self.last_updated = Utc::now();
    }

    /// Converts ElementType to string for indexing
    fn element_type_to_string(&self, element_type: &ElementType) -> String {
        match element_type {
            ElementType::Symbolic { .. } => "symbolic".to_string(),
            ElementType::Dialogue { .. } => "dialogue".to_string(),
            ElementType::Thematic { .. } => "thematic".to_string(),
            ElementType::CharacterMoment { .. } => "character".to_string(),
            ElementType::Atmospheric { .. } => "atmospheric".to_string(),
            ElementType::Structural { .. } => "structural".to_string(),
        }
    }

    /// Adds an echo/reference to an existing element
    pub fn add_echo(&mut self, echo: RecursiveEcho) {
        if let Some(element) = self.elements.get_mut(&echo.original_element_id) {
            element.echoes.push(echo);
            self.update_stats();
            self.last_updated = Utc::now();
        }
    }

    /// Updates recursion statistics
    fn update_stats(&mut self) {
        self.recursion_stats.total_elements = self.elements.len();
        self.recursion_stats.total_echoes = self.elements
            .values()
            .map(|e| e.echoes.len())
            .sum();

        self.recursion_stats.recursion_density = if self.recursion_stats.total_elements > 0 {
            self.recursion_stats.total_echoes as f32 / self.recursion_stats.total_elements as f32
        } else {
            0.0
        };

        // Count cross-level connections (echoes at different levels than original)
        self.recursion_stats.cross_level_connections = self.elements
            .values()
            .flat_map(|e| &e.echoes)
            .filter(|echo| {
                if let Some(original) = self.elements.get(&echo.original_element_id) {
                    echo.echo_level != original.level
                } else {
                    false
                }
            })
            .count();

        // Calculate thematic coherence
        self.recursion_stats.thematic_coherence_score = self.calculate_thematic_coherence();

        // Calculate structural symmetry
        self.recursion_stats.structural_symmetry_score = self.calculate_structural_symmetry();
    }

    /// Calculates thematic coherence across levels
    fn calculate_thematic_coherence(&self) -> f32 {
        let thematic_elements = self.type_index.get("thematic").map(|ids| ids.len()).unwrap_or(0);
        let thematic_echoes = self.elements
            .values()
            .filter(|e| matches!(e.element_type, ElementType::Thematic { .. }))
            .map(|e| e.echoes.len())
            .sum::<usize>();

        if thematic_elements > 0 {
            (thematic_echoes as f32 / thematic_elements as f32).min(1.0)
        } else {
            0.0
        }
    }

    /// Calculates structural symmetry score
    fn calculate_structural_symmetry(&self) -> f32 {
        let structural_echoes = self.elements
            .values()
            .flat_map(|e| &e.echoes)
            .filter(|echo| matches!(echo.recursion_type, RecursionType::StructuralSymmetry))
            .count();

        let total_structural = self.type_index.get("structural").map(|ids| ids.len()).unwrap_or(0);

        if total_structural > 0 {
            (structural_echoes as f32 / total_structural as f32).min(1.0)
        } else {
            0.0
        }
    }

    /// Advances narrative context
    pub fn advance_context(&mut self, chapter: Option<u32>, scene: Option<u32>, paragraph: Option<u32>) {
        if let Some(ch) = chapter {
            self.current_chapter = ch;
        }
        self.current_scene = scene;
        self.current_paragraph = paragraph;
        self.last_updated = Utc::now();
    }

    /// Finds potential recursive connections for a new element
    pub fn find_potential_echoes(&self, content: &str, element_type: &ElementType, level: &NarrativeLevel) -> Vec<RecursiveConnection> {
        let mut connections = Vec::new();

        // Look for thematic resonance
        if matches!(element_type, ElementType::Thematic { .. }) {
            for (id, element) in &self.elements {
                if matches!(element.element_type, ElementType::Thematic { .. }) && element.level != *level {
                    if self.has_thematic_resonance(content, &element.content) {
                        connections.push(RecursiveConnection {
                            target_element_id: id.clone(),
                            suggested_recursion_type: RecursionType::ThematicEscalation,
                            resonance_strength: self.calculate_content_similarity(content, &element.content),
                            description: format!("Thematic echo between {} and {}", level_to_string(level), level_to_string(&element.level)),
                        });
                    }
                }
            }
        }

        // Look for symbolic echoes
        if matches!(element_type, ElementType::Symbolic { .. }) {
            for (id, element) in &self.elements {
                if matches!(element.element_type, ElementType::Symbolic { .. }) && element.level != *level {
                    if self.has_symbolic_resonance(content, &element.content) {
                        connections.push(RecursiveConnection {
                            target_element_id: id.clone(),
                            suggested_recursion_type: RecursionType::SymbolicRecursion,
                            resonance_strength: self.calculate_content_similarity(content, &element.content),
                            description: format!("Symbolic echo between {} and {}", level_to_string(level), level_to_string(&element.level)),
                        });
                    }
                }
            }
        }

        // Look for tonal echoes
        if matches!(element_type, ElementType::Atmospheric { .. }) {
            for (id, element) in &self.elements {
                if matches!(element.element_type, ElementType::Atmospheric { .. }) && element.level != *level {
                    if self.has_tonal_resonance(content, &element.content) {
                        connections.push(RecursiveConnection {
                            target_element_id: id.clone(),
                            suggested_recursion_type: RecursionType::TonalEcho,
                            resonance_strength: self.calculate_content_similarity(content, &element.content),
                            description: format!("Tonal echo between {} and {}", level_to_string(level), level_to_string(&element.level)),
                        });
                    }
                }
            }
        }

        // Sort by resonance strength
        connections.sort_by(|a, b| b.resonance_strength.partial_cmp(&a.resonance_strength).unwrap_or(std::cmp::Ordering::Equal));
        connections
    }

    /// Checks for thematic resonance between content pieces
    fn has_thematic_resonance(&self, content1: &str, content2: &str) -> bool {
        let theme_keywords = vec!["love", "death", "power", "freedom", "truth", "justice", "redemption", "betrayal", "hope", "fear"];

        let content1_lower = content1.to_lowercase();
        let content2_lower = content2.to_lowercase();

        // Simple keyword overlap detection
        theme_keywords.iter().any(|keyword| {
            content1_lower.contains(keyword) && content2_lower.contains(keyword)
        })
    }

    /// Checks for symbolic resonance
    fn has_symbolic_resonance(&self, content1: &str, content2: &str) -> bool {
        let symbol_keywords = vec!["light", "dark", "fire", "water", "mirror", "door", "key", "bridge", "mountain", "ocean"];

        let content1_lower = content1.to_lowercase();
        let content2_lower = content2.to_lowercase();

        symbol_keywords.iter().any(|keyword| {
            content1_lower.contains(keyword) && content2_lower.contains(keyword)
        })
    }

    /// Checks for tonal resonance
    fn has_tonal_resonance(&self, content1: &str, content2: &str) -> bool {
        let tone_keywords = vec!["quiet", "loud", "fast", "slow", "gentle", "harsh", "warm", "cold", "bright", "dim"];

        let content1_lower = content1.to_lowercase();
        let content2_lower = content2.to_lowercase();

        tone_keywords.iter().any(|keyword| {
            content1_lower.contains(keyword) && content2_lower.contains(keyword)
        })
    }

    /// Calculates content similarity (simple implementation)
    fn calculate_content_similarity(&self, content1: &str, content2: &str) -> f32 {
        let words1: HashSet<&str> = content1.split_whitespace().collect();
        let words2: HashSet<&str> = content2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union > 0 {
            intersection as f32 / union as f32
        } else {
            0.0
        }
    }

    /// Gets elements at a specific narrative level
    pub fn get_elements_at_level(&self, level: &NarrativeLevel) -> Vec<&RecursiveElement> {
        if let Some(element_ids) = self.level_index.get(level) {
            element_ids.iter()
                .filter_map(|id| self.elements.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets elements of a specific type
    pub fn get_elements_of_type(&self, element_type: &str) -> Vec<&RecursiveElement> {
        if let Some(element_ids) = self.type_index.get(element_type) {
            element_ids.iter()
                .filter_map(|id| self.elements.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Analyzes recursion health across levels
    pub fn analyze_recursion_health(&self) -> RecursionHealthReport {
        let total_elements = self.recursion_stats.total_elements;
        let cross_level_ratio = if total_elements > 0 {
            self.recursion_stats.cross_level_connections as f32 / total_elements as f32
        } else {
            0.0
        };

        let health_score = {
            let mut score = 0.5; // Base score

            // Reward good recursion density
            if self.recursion_stats.recursion_density >= 0.3 && self.recursion_stats.recursion_density <= 0.7 {
                score += 0.2;
            }

            // Reward cross-level connections
            if cross_level_ratio >= 0.2 {
                score += 0.2;
            }

            // Reward thematic coherence
            score += self.recursion_stats.thematic_coherence_score * 0.15;

            // Reward structural symmetry
            score += self.recursion_stats.structural_symmetry_score * 0.15;

            score.min(1.0).max(0.0)
        };

        RecursionHealthReport {
            health_score,
            recursion_density: self.recursion_stats.recursion_density,
            cross_level_ratio,
            thematic_coherence: self.recursion_stats.thematic_coherence_score,
            structural_symmetry: self.recursion_stats.structural_symmetry_score,
            recommendations: self.generate_recursion_recommendations(health_score, cross_level_ratio),
        }
    }

    /// Generates recommendations for improving recursion
    fn generate_recursion_recommendations(&self, health_score: f32, cross_level_ratio: f32) -> Vec<String> {
        let mut recommendations = Vec::new();

        if health_score < 0.6 {
            recommendations.push("🔄 Recursion health needs attention - consider adding echoes or returns".to_string());
        }

        if self.recursion_stats.recursion_density < 0.2 {
            recommendations.push("🔗 Low recursion density - story may lack deeper meaning connections".to_string());
        }

        if self.recursion_stats.recursion_density > 0.8 {
            recommendations.push("🔗 High recursion density - ensure forward momentum isn't stalled".to_string());
        }

        if cross_level_ratio < 0.15 {
            recommendations.push("📏 Few cross-level connections - consider echoing themes across scales".to_string());
        }

        if self.recursion_stats.thematic_coherence_score < 0.3 {
            recommendations.push("🎭 Low thematic coherence - themes may need stronger reinforcement".to_string());
        }

        if self.recursion_stats.structural_symmetry_score < 0.2 && self.elements.len() > 20 {
            recommendations.push("🏗️ Limited structural symmetry - consider mirroring elements between beginning/end".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("✅ Recursion patterns appear healthy across all levels".to_string());
        }

        recommendations
    }

    /// Generates a comprehensive recursion report
    pub fn generate_recursion_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🔄 MULTI-LEVEL RECURSION ANALYSIS\n");
        report.push_str("==================================\n\n");

        report.push_str(&format!("Current Context: Chapter {}", self.current_chapter));
        if let Some(scene) = self.current_scene {
            report.push_str(&format!(", Scene {}", scene));
        }
        if let Some(paragraph) = self.current_paragraph {
            report.push_str(&format!(", Paragraph {}", paragraph));
        }
        report.push_str("\n\n");

        // Statistics
        report.push_str("📊 Recursion Statistics:\n");
        report.push_str(&format!("  • Total Elements: {}\n", self.recursion_stats.total_elements));
        report.push_str(&format!("  • Total Echoes: {}\n", self.recursion_stats.total_echoes));
        report.push_str(&format!("  • Cross-Level Connections: {}\n", self.recursion_stats.cross_level_connections));
        report.push_str(&format!("  • Recursion Density: {:.2}\n", self.recursion_stats.recursion_density));
        report.push_str(&format!("  • Thematic Coherence: {:.2}\n", self.recursion_stats.thematic_coherence_score));
        report.push_str(&format!("  • Structural Symmetry: {:.2}\n\n", self.recursion_stats.structural_symmetry_score));

        // Distribution by level
        report.push_str("📏 Distribution by Level:\n");
        for level in [NarrativeLevel::Sentence, NarrativeLevel::Paragraph, NarrativeLevel::Scene,
                     NarrativeLevel::Chapter, NarrativeLevel::Act, NarrativeLevel::Story] {
            let count = self.level_index.get(&level).map(|ids| ids.len()).unwrap_or(0);
            report.push_str(&format!("  • {}: {}\n", level_to_string(&level), count));
        }
        report.push('\n');

        // Distribution by type
        report.push_str("🎨 Distribution by Type:\n");
        for element_type in ["symbolic", "dialogue", "thematic", "character", "atmospheric", "structural"] {
            let count = self.type_index.get(element_type).map(|ids| ids.len()).unwrap_or(0);
            report.push_str(&format!("  • {}: {}\n", element_type, count));
        }
        report.push('\n');

        // Health analysis
        let health = self.analyze_recursion_health();
        report.push_str(&format!("🏥 Recursion Health Score: {:.2}/1.0\n\n", health.health_score));

        // Recommendations
        report.push_str("💡 Recommendations:\n");
        for rec in &health.recommendations {
            report.push_str(&format!("  • {}\n", rec));
        }

        report
    }
}

use std::collections::HashSet;

/// A potential recursive connection
#[derive(Debug, Clone)]
pub struct RecursiveConnection {
    pub target_element_id: String,
    pub suggested_recursion_type: RecursionType,
    pub resonance_strength: f32,
    pub description: String,
}

/// Health analysis of recursion patterns
#[derive(Debug, Clone)]
pub struct RecursionHealthReport {
    pub health_score: f32,
    pub recursion_density: f32,
    pub cross_level_ratio: f32,
    pub thematic_coherence: f32,
    pub structural_symmetry: f32,
    pub recommendations: Vec<String>,
}

/// Helper function to convert NarrativeLevel to string
fn level_to_string(level: &NarrativeLevel) -> &'static str {
    match level {
        NarrativeLevel::Sentence => "Sentence",
        NarrativeLevel::Paragraph => "Paragraph",
        NarrativeLevel::Scene => "Scene",
        NarrativeLevel::Chapter => "Chapter",
        NarrativeLevel::Act => "Act",
        NarrativeLevel::Story => "Story",
    }
}

impl Default for MultiLevelRecursionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_level_tracker_creation() {
        let tracker = MultiLevelRecursionTracker::new();
        assert_eq!(tracker.current_chapter, 1);
        assert!(tracker.elements.is_empty());
        assert_eq!(tracker.recursion_stats.total_elements, 0);
    }

    #[test]
    fn test_add_element() {
        let mut tracker = MultiLevelRecursionTracker::new();

        let element = RecursiveElement {
            id: "test_element".to_string(),
            content: "The mirror reflected more than light".to_string(),
            level: NarrativeLevel::Sentence,
            chapter: 1,
            scene: Some(1),
            paragraph: Some(1),
            sentence: Some(1),
            element_type: ElementType::Symbolic {
                symbol_name: "mirror".to_string(),
                meaning: "self-reflection".to_string(),
            },
            intensity: 0.8,
            created_at: Utc::now(),
            echoes: Vec::new(),
        };

        tracker.add_element(element);

        assert_eq!(tracker.elements.len(), 1);
        assert_eq!(tracker.recursion_stats.total_elements, 1);
        assert!(tracker.level_index.get(&NarrativeLevel::Sentence).is_some());
        assert!(tracker.type_index.get("symbolic").is_some());
    }

    #[test]
    fn test_add_echo() {
        let mut tracker = MultiLevelRecursionTracker::new();

        let element = RecursiveElement {
            id: "original".to_string(),
            content: "The door stood open".to_string(),
            level: NarrativeLevel::Sentence,
            chapter: 1,
            scene: Some(1),
            paragraph: Some(1),
            sentence: Some(1),
            element_type: ElementType::Symbolic {
                symbol_name: "door".to_string(),
                meaning: "opportunity".to_string(),
            },
            intensity: 0.6,
            created_at: Utc::now(),
            echoes: Vec::new(),
        };

        tracker.add_element(element);

        let echo = RecursiveEcho {
            echo_id: "echo1".to_string(),
            original_element_id: "original".to_string(),
            echo_level: NarrativeLevel::Chapter,
            recursion_type: RecursionType::SymbolicRecursion,
            transformation: "The door now represents final choice".to_string(),
            intensity_delta: 0.2,
            chapter: 5,
            scene: Some(10),
            created_at: Utc::now(),
        };

        tracker.add_echo(echo);

        assert_eq!(tracker.recursion_stats.total_echoes, 1);
        assert_eq!(tracker.recursion_stats.cross_level_connections, 1);
        assert!(tracker.elements.get("original").unwrap().echoes.len() == 1);
    }

    #[test]
    fn test_find_potential_echoes() {
        let mut tracker = MultiLevelRecursionTracker::new();

        // Add a thematic element
        let theme_element = RecursiveElement {
            id: "theme1".to_string(),
            content: "Love conquers fear".to_string(),
            level: NarrativeLevel::Sentence,
            chapter: 1,
            scene: Some(1),
            paragraph: Some(1),
            sentence: Some(1),
            element_type: ElementType::Thematic {
                theme: "love".to_string(),
                position: "positive".to_string(),
            },
            intensity: 0.8,
            created_at: Utc::now(),
            echoes: Vec::new(),
        };

        tracker.add_element(theme_element);

        // Look for connections to new thematic content at a different level
        let connections = tracker.find_potential_echoes(
            "Fear cannot defeat love",
            &ElementType::Thematic {
                theme: "love".to_string(),
                position: "positive".to_string(),
            },
            &NarrativeLevel::Chapter,
        );

        assert!(!connections.is_empty());
        assert_eq!(connections[0].target_element_id, "theme1");
    }

    #[test]
    fn test_recursion_stats_calculation() {
        let mut tracker = MultiLevelRecursionTracker::new();

        // Add elements
        for i in 0..5 {
            let element = RecursiveElement {
                id: format!("element_{}", i),
                content: format!("Content {}", i),
                level: NarrativeLevel::Sentence,
                chapter: 1,
                scene: Some(1),
                paragraph: Some(1),
                sentence: Some(i as u32),
                element_type: ElementType::Thematic {
                    theme: "test".to_string(),
                    position: "neutral".to_string(),
                },
                intensity: 0.5,
                created_at: Utc::now(),
                echoes: Vec::new(),
            };
            tracker.add_element(element);
        }

        // Add some echoes
        for i in 0..3 {
            let echo = RecursiveEcho {
                echo_id: format!("echo_{}", i),
                original_element_id: format!("element_{}", i),
                echo_level: NarrativeLevel::Chapter,
                recursion_type: RecursionType::ThematicEscalation,
                transformation: "Enhanced".to_string(),
                intensity_delta: 0.1,
                chapter: 2,
                scene: Some(1),
                created_at: Utc::now(),
            };
            tracker.add_echo(echo);
        }

        assert_eq!(tracker.recursion_stats.total_elements, 5);
        assert_eq!(tracker.recursion_stats.total_echoes, 3);
        assert_eq!(tracker.recursion_stats.recursion_density, 0.6); // 3/5
    }

    #[test]
    fn test_get_elements_at_level() {
        let mut tracker = MultiLevelRecursionTracker::new();

        let sentence_element = RecursiveElement {
            id: "sentence1".to_string(),
            content: "Test sentence".to_string(),
            level: NarrativeLevel::Sentence,
            chapter: 1,
            scene: Some(1),
            paragraph: Some(1),
            sentence: Some(1),
            element_type: ElementType::Dialogue {
                speaker: "hero".to_string(),
                emotional_charge: 0.5,
            },
            intensity: 0.5,
            created_at: Utc::now(),
            echoes: Vec::new(),
        };

        let chapter_element = RecursiveElement {
            id: "chapter1".to_string(),
            content: "Test chapter".to_string(),
            level: NarrativeLevel::Chapter,
            chapter: 1,
            scene: None,
            paragraph: None,
            sentence: None,
            element_type: ElementType::Structural {
                function: "climax".to_string(),
                impact_level: 0.9,
            },
            intensity: 0.9,
            created_at: Utc::now(),
            echoes: Vec::new(),
        };

        tracker.add_element(sentence_element);
        tracker.add_element(chapter_element);

        let sentence_elements = tracker.get_elements_at_level(&NarrativeLevel::Sentence);
        let chapter_elements = tracker.get_elements_at_level(&NarrativeLevel::Chapter);

        assert_eq!(sentence_elements.len(), 1);
        assert_eq!(chapter_elements.len(), 1);
        assert_eq!(sentence_elements[0].id, "sentence1");
        assert_eq!(chapter_elements[0].id, "chapter1");
    }

    #[test]
    fn test_analyze_recursion_health() {
        let mut tracker = MultiLevelRecursionTracker::new();

        // Add a good mix of elements and echoes
        for i in 0..10 {
            let element = RecursiveElement {
                id: format!("element_{}", i),
                content: format!("Test content {}", i),
                level: if i % 2 == 0 { NarrativeLevel::Sentence } else { NarrativeLevel::Chapter },
                chapter: 1,
                scene: Some(1),
                paragraph: Some(1),
                sentence: Some(i as u32),
                element_type: ElementType::Thematic {
                    theme: "test".to_string(),
                    position: "positive".to_string(),
                },
                intensity: 0.5,
                created_at: Utc::now(),
                echoes: Vec::new(),
            };
            tracker.add_element(element);
        }

        // Add cross-level echoes
        for i in 0..5 {
            let echo = RecursiveEcho {
                echo_id: format!("echo_{}", i),
                original_element_id: format!("element_{}", i),
                echo_level: NarrativeLevel::Act,
                recursion_type: RecursionType::ThematicEscalation,
                transformation: "Transformed".to_string(),
                intensity_delta: 0.1,
                chapter: 2,
                scene: Some(1),
                created_at: Utc::now(),
            };
            tracker.add_echo(echo);
        }

        let health = tracker.analyze_recursion_health();
        assert!(health.health_score > 0.5);
        assert!(health.cross_level_ratio > 0.0);
        assert!(!health.recommendations.is_empty());
    }

    #[test]
    fn test_advance_context() {
        let mut tracker = MultiLevelRecursionTracker::new();

        tracker.advance_context(Some(5), Some(3), Some(2));

        assert_eq!(tracker.current_chapter, 5);
        assert_eq!(tracker.current_scene, Some(3));
        assert_eq!(tracker.current_paragraph, Some(2));
    }

    #[test]
    fn test_content_similarity() {
        let tracker = MultiLevelRecursionTracker::new();

        let similarity1 = tracker.calculate_content_similarity("the quick brown fox", "the lazy brown dog");
        let similarity2 = tracker.calculate_content_similarity("hello world", "goodbye universe");

        assert!(similarity1 > similarity2); // First pair shares more words
    }
}