/// 🗺️ Narrative Constraint Space Modeling
///
/// Models the dynamic constraint graph where each narrative choice:
/// - Narrows future paths
/// - Implies obligations
/// - Generates tension
/// - Opens contradictions
///
/// This system helps writers navigate constraint space without losing
/// the essential unpredictability that makes stories compelling.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

/// Types of narrative constraints that limit future story possibilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Character traits, fears, beliefs that prevent certain actions
    CharacterState {
        character: String,
        trait_name: String,
        prevents_actions: Vec<String>,
    },
    /// World rules, physics, culture that constrain possibilities
    WorldLogic {
        rule_name: String,
        description: String,
        blocked_scenarios: Vec<String>,
    },
    /// Genre expectations that guide but don't enforce structure
    GenreExpectation {
        genre: String,
        expectation: String,
        flexibility_score: f32, // 0.0 = rigid, 1.0 = flexible
    },
    /// Thematic commitments that keep arcs coherent
    ThematicCommitment {
        theme: String,
        stance: String, // The story's position on this theme
        requires_consistency: bool,
    },
    /// Unresolved threads that prevent narrative betrayal
    UnresolvedThread {
        thread_id: String,
        description: String,
        urgency: f32,
        must_resolve_by_chapter: Option<u32>,
    },
    /// Plot thread constraints that maintain narrative coherence
    PlotThread {
        thread_id: String,
        description: String,
        resolution_required: bool,
    },
    /// Temporal constraints that block certain sequences
    TemporalBlock {
        description: String,
        blocked_scenarios: Vec<String>,
        duration: Option<u32>,
    },
}

/// Represents a node in the constraint graph - a possible narrative state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintNode {
    pub id: String,
    pub description: String,
    pub chapter: u32,
    pub scene: Option<u32>,
    pub probability: f32, // 0.0 to 1.0 - how likely this state is
    pub constraints_applied: Vec<String>, // IDs of constraints that led to this state
    pub blocked_paths: Vec<String>, // IDs of paths that are no longer possible
    pub created_at: DateTime<Utc>,
}

/// Represents an edge in the constraint graph - a possible development path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintEdge {
    pub from_node: String,
    pub to_node: String,
    pub description: String,
    pub probability_weight: f32,
    pub constraint_cost: f32, // How much freedom this path removes
    pub is_blocked: bool,
    pub blocked_by: Vec<String>, // Constraint IDs that block this path
}

/// Tracks the evolving constraint space of the narrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSpaceTracker {
    /// All constraints currently active
    pub constraints: Vec<ConstraintType>,
    /// Constraint graph nodes
    pub nodes: HashMap<String, ConstraintNode>,
    /// Constraint graph edges
    pub edges: Vec<ConstraintEdge>,
    /// Current narrative state
    pub current_node: Option<String>,
    /// Constraint history for analysis
    pub constraint_history: Vec<ConstraintHistoryEntry>,
    /// Current chapter context
    pub current_chapter: u32,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Historical entry for constraint evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintHistoryEntry {
    pub chapter: u32,
    pub action_taken: String,
    pub constraints_added: Vec<String>,
    pub paths_blocked: Vec<String>,
    pub freedom_score_before: f32,
    pub freedom_score_after: f32,
    pub timestamp: DateTime<Utc>,
}

impl ConstraintSpaceTracker {
    /// Creates a new constraint space tracker
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            current_node: None,
            constraint_history: Vec::new(),
            current_chapter: 1,
            last_updated: Utc::now(),
        }
    }

    /// Adds a new constraint to the space
    pub fn add_constraint(&mut self, constraint: ConstraintType) {
        let constraint_id = self.generate_constraint_id(&constraint);
        self.constraints.push(constraint);
        self.recalculate_blocked_paths(&constraint_id);
        self.last_updated = Utc::now();
    }

    /// Generates a unique ID for a constraint
    fn generate_constraint_id(&self, constraint: &ConstraintType) -> String {
        match constraint {
            ConstraintType::CharacterState { character, trait_name, .. } => {
                format!("char_{}_{}", character, trait_name)
            }
            ConstraintType::WorldLogic { rule_name, .. } => {
                format!("world_{}", rule_name)
            }
            ConstraintType::GenreExpectation { genre, expectation, .. } => {
                format!("genre_{}_{}", genre, expectation.chars().take(10).collect::<String>())
            }
            ConstraintType::ThematicCommitment { theme, .. } => {
                format!("theme_{}", theme)
            }
            ConstraintType::UnresolvedThread { thread_id, .. } => {
                format!("thread_{}", thread_id)
            }
            ConstraintType::PlotThread { thread_id, .. } => {
                format!("plot_{}", thread_id)
            }
            ConstraintType::TemporalBlock { description, .. } => {
                format!("temporal_{}", description.chars().take(10).collect::<String>())
            }
        }
    }

    /// Recalculates which paths are blocked by the new constraint
    fn recalculate_blocked_paths(&mut self, constraint_id: &str) {
        // Update edge blocking based on new constraint
        // First collect edge descriptions to check
        let edge_descriptions: Vec<String> = self.edges.iter()
            .map(|edge| edge.description.clone())
            .collect();

        // Then update edges based on constraint checks
        for (i, description) in edge_descriptions.iter().enumerate() {
            if self.edge_blocked_by_constraint(description, constraint_id) {
                self.edges[i].is_blocked = true;
                self.edges[i].blocked_by.push(constraint_id.to_string());
            }
        }
    }

    /// Checks if a specific development path is blocked by a constraint
    fn edge_blocked_by_constraint(&self, edge_description: &str, constraint_id: &str) -> bool {
        // This would be enhanced with more sophisticated logic
        // For now, simple keyword matching
        if let Some(constraint) = self.constraints.iter().find(|c| self.generate_constraint_id(c) == constraint_id) {
            match constraint {
                ConstraintType::CharacterState { prevents_actions, .. } => {
                    prevents_actions.iter().any(|action| edge_description.contains(action))
                }
                ConstraintType::WorldLogic { blocked_scenarios, .. } => {
                    blocked_scenarios.iter().any(|scenario| edge_description.contains(scenario))
                }
                _ => false, // Other constraint types don't directly block paths
            }
        } else {
            false
        }
    }

    /// Adds a new node to the constraint graph
    pub fn add_node(&mut self, node: ConstraintNode) {
        self.nodes.insert(node.id.clone(), node);
        self.last_updated = Utc::now();
    }

    /// Adds a new edge to the constraint graph
    pub fn add_edge(&mut self, edge: ConstraintEdge) {
        self.edges.push(edge);
        self.last_updated = Utc::now();
    }

    /// Sets the current narrative state
    pub fn set_current_state(&mut self, node_id: String) {
        self.current_node = Some(node_id);
        self.last_updated = Utc::now();
    }

    /// Calculates the current narrative freedom score
    pub fn calculate_freedom_score(&self) -> f32 {
        if self.edges.is_empty() {
            return 1.0;
        }

        let total_edges = self.edges.len() as f32;
        let blocked_edges = self.edges.iter().filter(|e| e.is_blocked).count() as f32;
        let available_paths = total_edges - blocked_edges;

        (available_paths / total_edges).max(0.0)
    }

    /// Gets all currently available development paths from current state
    pub fn get_available_paths(&self) -> Vec<&ConstraintEdge> {
        if let Some(current) = &self.current_node {
            self.edges
                .iter()
                .filter(|edge| edge.from_node == *current && !edge.is_blocked)
                .collect()
        } else {
            self.edges.iter().filter(|edge| !edge.is_blocked).collect()
        }
    }

    /// Gets all blocked paths and the constraints that block them
    pub fn get_blocked_paths(&self) -> Vec<BlockedPathReport> {
        self.edges
            .iter()
            .filter(|edge| edge.is_blocked)
            .map(|edge| BlockedPathReport {
                path_description: edge.description.clone(),
                blocked_by_constraints: edge.blocked_by.clone(),
                constraint_cost: edge.constraint_cost,
            })
            .collect()
    }

    /// Analyzes constraint pressure - how much freedom has been lost
    pub fn analyze_constraint_pressure(&self) -> ConstraintPressureAnalysis {
        let freedom_score = self.calculate_freedom_score();
        let total_constraints = self.constraints.len();
        let unresolved_threads = self.constraints
            .iter()
            .filter(|c| matches!(c, ConstraintType::UnresolvedThread { .. }))
            .count();

        let pressure_level = match freedom_score {
            f if f > 0.8 => PressureLevel::Low,
            f if f > 0.6 => PressureLevel::Moderate,
            f if f > 0.4 => PressureLevel::High,
            f if f > 0.2 => PressureLevel::Critical,
            _ => PressureLevel::Extreme,
        };

        ConstraintPressureAnalysis {
            freedom_score,
            pressure_level,
            total_constraints,
            unresolved_threads,
            blocked_paths_count: self.edges.iter().filter(|e| e.is_blocked).count(),
            available_paths_count: self.edges.iter().filter(|e| !e.is_blocked).count(),
            recommendations: self.generate_pressure_recommendations(freedom_score, total_constraints, unresolved_threads),
        }
    }

    /// Generates recommendations based on constraint pressure
    fn generate_pressure_recommendations(&self, freedom_score: f32, total_constraints: usize, unresolved_threads: usize) -> Vec<String> {
        let mut recommendations = Vec::new();

        if freedom_score < 0.3 {
            recommendations.push("🚨 Constraint space severely limited - consider resolution or constraint removal".to_string());
        }

        if freedom_score < 0.5 {
            recommendations.push("⚠️ Limited narrative freedom - approaching constraint singularity".to_string());
        }

        if total_constraints > 15 {
            recommendations.push("📊 High constraint count - monitor for over-constraint".to_string());
        }

        if unresolved_threads > 8 {
            recommendations.push(format!("🧵 {} unresolved threads creating obligation pressure", unresolved_threads));
        }

        if freedom_score > 0.9 && total_constraints < 3 {
            recommendations.push("🔄 High freedom with few constraints - consider adding narrative tension".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("✅ Constraint space appears well-balanced".to_string());
        }

        recommendations
    }

    /// Records a narrative choice and its constraint impact
    pub fn record_choice(&mut self, description: String, constraints_added: Vec<String>, paths_blocked: Vec<String>) {
        let freedom_before = self.calculate_freedom_score();

        // This would update the constraint space based on the choice
        // For now, we just record the history

        let freedom_after = self.calculate_freedom_score();

        self.constraint_history.push(ConstraintHistoryEntry {
            chapter: self.current_chapter,
            action_taken: description,
            constraints_added,
            paths_blocked,
            freedom_score_before: freedom_before,
            freedom_score_after: freedom_after,
            timestamp: Utc::now(),
        });

        self.last_updated = Utc::now();
    }

    /// Advances to the next chapter
    pub fn advance_chapter(&mut self) {
        self.current_chapter += 1;
        self.last_updated = Utc::now();
    }

    /// Finds constraints that could be resolved to reopen narrative paths
    pub fn find_resolvable_constraints(&self) -> Vec<ResolvableConstraint> {
        let mut resolvable = Vec::new();

        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                ConstraintType::UnresolvedThread { thread_id, description, urgency, .. } => {
                    resolvable.push(ResolvableConstraint {
                        constraint_index: i,
                        description: format!("Resolve thread: {}", description),
                        resolution_benefit: urgency * 0.5, // How much freedom would be gained
                        thread_id: Some(thread_id.clone()),
                    });
                }
                ConstraintType::CharacterState { character, trait_name, prevents_actions, .. } => {
                    if prevents_actions.len() > 2 {
                        resolvable.push(ResolvableConstraint {
                            constraint_index: i,
                            description: format!("Evolve {}'s {} trait", character, trait_name),
                            resolution_benefit: prevents_actions.len() as f32 * 0.1,
                            thread_id: None,
                        });
                    }
                }
                _ => {} // Other constraints are typically structural and shouldn't be removed
            }
        }

        // Sort by resolution benefit (highest first)
        resolvable.sort_by(|a, b| b.resolution_benefit.partial_cmp(&a.resolution_benefit).unwrap_or(std::cmp::Ordering::Equal));
        resolvable
    }

    /// Generates a comprehensive constraint space report
    pub fn generate_constraint_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🗺️ CONSTRAINT SPACE ANALYSIS\n");
        report.push_str("=============================\n\n");

        report.push_str(&format!("Current Chapter: {}\n", self.current_chapter));
        report.push_str(&format!("Current Freedom Score: {:.2}\n", self.calculate_freedom_score()));
        report.push_str(&format!("Total Constraints: {}\n", self.constraints.len()));
        report.push_str(&format!("Graph Nodes: {}\n", self.nodes.len()));
        report.push_str(&format!("Graph Edges: {} ({} blocked)\n\n",
            self.edges.len(),
            self.edges.iter().filter(|e| e.is_blocked).count()));

        // Pressure analysis
        let pressure = self.analyze_constraint_pressure();
        report.push_str(&format!("📊 Constraint Pressure: {:?}\n", pressure.pressure_level));
        report.push_str(&format!("📊 Available Paths: {}\n", pressure.available_paths_count));
        report.push_str(&format!("📊 Blocked Paths: {}\n\n", pressure.blocked_paths_count));

        // Constraint breakdown
        report.push_str("🔒 Active Constraints:\n");
        let mut char_constraints = 0;
        let mut world_constraints = 0;
        let mut genre_constraints = 0;
        let mut theme_constraints = 0;
        let mut thread_constraints = 0;

        for constraint in &self.constraints {
            match constraint {
                ConstraintType::CharacterState { .. } => char_constraints += 1,
                ConstraintType::WorldLogic { .. } => world_constraints += 1,
                ConstraintType::GenreExpectation { .. } => genre_constraints += 1,
                ConstraintType::ThematicCommitment { .. } => theme_constraints += 1,
                ConstraintType::UnresolvedThread { .. } => thread_constraints += 1,
                ConstraintType::PlotThread { .. } => thread_constraints += 1,
                ConstraintType::TemporalBlock { .. } => world_constraints += 1,
            }
        }

        report.push_str(&format!("  • Character State: {}\n", char_constraints));
        report.push_str(&format!("  • World Logic: {}\n", world_constraints));
        report.push_str(&format!("  • Genre Expectations: {}\n", genre_constraints));
        report.push_str(&format!("  • Thematic Commitments: {}\n", theme_constraints));
        report.push_str(&format!("  • Unresolved Threads: {}\n\n", thread_constraints));

        // Recommendations
        report.push_str("💡 Recommendations:\n");
        for rec in &pressure.recommendations {
            report.push_str(&format!("  • {}\n", rec));
        }
        report.push('\n');

        // Resolvable constraints
        let resolvable = self.find_resolvable_constraints();
        if !resolvable.is_empty() {
            report.push_str("🔓 Resolvable Constraints:\n");
            for (i, res) in resolvable.iter().take(5).enumerate() {
                report.push_str(&format!("  {}. {} (benefit: {:.2})\n", i + 1, res.description, res.resolution_benefit));
            }
            report.push('\n');
        }

        report
    }

    /// Get names of all active constraints
    pub fn get_active_constraint_names(&self) -> Vec<String> {
        self.constraints.iter()
            .map(|c| match c {
                ConstraintType::CharacterState { character, trait_name, .. } => {
                    format!("{}_{}", character, trait_name)
                },
                ConstraintType::WorldLogic { rule_name, .. } => {
                    rule_name.clone()
                },
                ConstraintType::GenreExpectation { genre, expectation, .. } => {
                    format!("{}_{}", genre, expectation)
                },
                ConstraintType::PlotThread { thread_id, .. } => {
                    thread_id.clone()
                },
                ConstraintType::TemporalBlock { description, .. } => {
                    description.clone()
                },
                ConstraintType::ThematicCommitment { theme, .. } => {
                    theme.clone()
                },
                ConstraintType::UnresolvedThread { thread_id, .. } => {
                    thread_id.clone()
                },
            })
            .collect()
    }

    /// Get pressure values for all constraints
    pub fn get_constraint_pressures(&self) -> HashMap<String, f32> {
        // Generate pressure values based on constraint history and nodes
        let mut pressures = HashMap::new();

        for (i, constraint) in self.constraints.iter().enumerate() {
            // Simple pressure calculation based on number of blocked paths
            let blocked_count = self.nodes.values()
                .map(|node| node.blocked_paths.len())
                .sum::<usize>() as f32;
            let pressure = (blocked_count / (i + 1) as f32).min(1.0);

            // Generate constraint name based on type
            let constraint_name = match constraint {
                ConstraintType::CharacterState { character, trait_name, .. } => {
                    format!("{}_{}", character, trait_name)
                }
                ConstraintType::PlotThread { thread_id, .. } => {
                    format!("plot_{}", thread_id)
                }
                ConstraintType::TemporalBlock { description, .. } => {
                    format!("temporal_{}", description)
                }
                ConstraintType::ThematicCommitment { theme, .. } => {
                    format!("theme_{}", theme)
                }
                ConstraintType::UnresolvedThread { thread_id, .. } => {
                    format!("unresolved_{}", thread_id)
                }
                ConstraintType::WorldLogic { rule_name, .. } => {
                    format!("world_{}", rule_name)
                }
                ConstraintType::GenreExpectation { genre, expectation, .. } => {
                    format!("genre_{}_{}", genre, expectation.chars().take(10).collect::<String>())
                }
            };

            pressures.insert(constraint_name, pressure);
        }

        pressures
    }
}

/// Analysis of current constraint pressure
#[derive(Debug, Clone)]
pub struct ConstraintPressureAnalysis {
    pub freedom_score: f32,
    pub pressure_level: PressureLevel,
    pub total_constraints: usize,
    pub unresolved_threads: usize,
    pub blocked_paths_count: usize,
    pub available_paths_count: usize,
    pub recommendations: Vec<String>,
}

/// Levels of constraint pressure
#[derive(Debug, Clone, PartialEq)]
pub enum PressureLevel {
    Low,      // 0.8+ freedom
    Moderate, // 0.6+ freedom
    High,     // 0.4+ freedom
    Critical, // 0.2+ freedom
    Extreme,  // <0.2 freedom
}

/// Report of a blocked narrative path
#[derive(Debug, Clone)]
pub struct BlockedPathReport {
    pub path_description: String,
    pub blocked_by_constraints: Vec<String>,
    pub constraint_cost: f32,
}

/// A constraint that could be resolved to reopen paths
#[derive(Debug, Clone)]
pub struct ResolvableConstraint {
    pub constraint_index: usize,
    pub description: String,
    pub resolution_benefit: f32,
    pub thread_id: Option<String>,
}

impl Default for ConstraintSpaceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_space_tracker_creation() {
        let tracker = ConstraintSpaceTracker::new();
        assert_eq!(tracker.current_chapter, 1);
        assert!(tracker.constraints.is_empty());
        assert!(tracker.nodes.is_empty());
        assert_eq!(tracker.calculate_freedom_score(), 1.0);
    }

    #[test]
    fn test_constraint_addition() {
        let mut tracker = ConstraintSpaceTracker::new();

        let constraint = ConstraintType::CharacterState {
            character: "hero".to_string(),
            trait_name: "fear_of_heights".to_string(),
            prevents_actions: vec!["climbing".to_string(), "flying".to_string()],
        };

        tracker.add_constraint(constraint);
        assert_eq!(tracker.constraints.len(), 1);
    }

    #[test]
    fn test_freedom_score_calculation() {
        let mut tracker = ConstraintSpaceTracker::new();

        // Add some edges
        tracker.add_edge(ConstraintEdge {
            from_node: "start".to_string(),
            to_node: "end1".to_string(),
            description: "path1".to_string(),
            probability_weight: 1.0,
            constraint_cost: 0.1,
            is_blocked: false,
            blocked_by: vec![],
        });

        tracker.add_edge(ConstraintEdge {
            from_node: "start".to_string(),
            to_node: "end2".to_string(),
            description: "path2".to_string(),
            probability_weight: 1.0,
            constraint_cost: 0.1,
            is_blocked: true,
            blocked_by: vec!["constraint1".to_string()],
        });

        let freedom = tracker.calculate_freedom_score();
        assert_eq!(freedom, 0.5); // 1 out of 2 paths available
    }

    #[test]
    fn test_available_paths() {
        let mut tracker = ConstraintSpaceTracker::new();

        tracker.add_edge(ConstraintEdge {
            from_node: "current".to_string(),
            to_node: "next".to_string(),
            description: "available_path".to_string(),
            probability_weight: 1.0,
            constraint_cost: 0.1,
            is_blocked: false,
            blocked_by: vec![],
        });

        tracker.add_edge(ConstraintEdge {
            from_node: "current".to_string(),
            to_node: "blocked".to_string(),
            description: "blocked_path".to_string(),
            probability_weight: 1.0,
            constraint_cost: 0.1,
            is_blocked: true,
            blocked_by: vec!["constraint1".to_string()],
        });

        tracker.set_current_state("current".to_string());

        let available = tracker.get_available_paths();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].description, "available_path");
    }

    #[test]
    fn test_constraint_pressure_analysis() {
        let mut tracker = ConstraintSpaceTracker::new();

        // Add constraints to increase pressure
        for i in 0..10 {
            tracker.add_constraint(ConstraintType::UnresolvedThread {
                thread_id: format!("thread_{}", i),
                description: format!("Test thread {}", i),
                urgency: 0.5,
                must_resolve_by_chapter: None,
            });
        }

        let analysis = tracker.analyze_constraint_pressure();
        assert_eq!(analysis.total_constraints, 10);
        assert_eq!(analysis.unresolved_threads, 10);
        assert!(!analysis.recommendations.is_empty());
    }

    #[test]
    fn test_blocked_paths_report() {
        let mut tracker = ConstraintSpaceTracker::new();

        tracker.add_edge(ConstraintEdge {
            from_node: "start".to_string(),
            to_node: "end".to_string(),
            description: "blocked_path".to_string(),
            probability_weight: 1.0,
            constraint_cost: 0.5,
            is_blocked: true,
            blocked_by: vec!["constraint1".to_string(), "constraint2".to_string()],
        });

        let blocked = tracker.get_blocked_paths();
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].path_description, "blocked_path");
        assert_eq!(blocked[0].blocked_by_constraints.len(), 2);
    }

    #[test]
    fn test_resolvable_constraints() {
        let mut tracker = ConstraintSpaceTracker::new();

        tracker.add_constraint(ConstraintType::UnresolvedThread {
            thread_id: "thread1".to_string(),
            description: "High urgency thread".to_string(),
            urgency: 0.9,
            must_resolve_by_chapter: Some(5),
        });

        tracker.add_constraint(ConstraintType::CharacterState {
            character: "hero".to_string(),
            trait_name: "indecisive".to_string(),
            prevents_actions: vec!["quick_decision".to_string(), "leadership".to_string(), "commitment".to_string()],
        });

        let resolvable = tracker.find_resolvable_constraints();
        assert!(!resolvable.is_empty());

        // Thread should have higher benefit due to urgency
        let thread_resolution = resolvable.iter().find(|r| r.thread_id.is_some());
        assert!(thread_resolution.is_some());
    }

    #[test]
    fn test_constraint_history() {
        let mut tracker = ConstraintSpaceTracker::new();

        tracker.record_choice(
            "Hero chooses to trust the stranger".to_string(),
            vec!["trust_constraint".to_string()],
            vec!["betrayal_path".to_string()],
        );

        assert_eq!(tracker.constraint_history.len(), 1);
        assert_eq!(tracker.constraint_history[0].action_taken, "Hero chooses to trust the stranger");
    }

    #[test]
    fn test_generate_constraint_report() {
        let mut tracker = ConstraintSpaceTracker::new();

        tracker.add_constraint(ConstraintType::CharacterState {
            character: "hero".to_string(),
            trait_name: "fear".to_string(),
            prevents_actions: vec!["confrontation".to_string()],
        });

        let report = tracker.generate_constraint_report();
        assert!(report.contains("CONSTRAINT SPACE ANALYSIS"));
        assert!(report.contains("Current Chapter: 1"));
        assert!(report.contains("Character State: 1"));
    }
}