/// Recursive Integrity Core (RIC v1.0)
///
/// Provides fail-closed, sovereign-preserving, recursion-bounded operation
/// for all narrative intelligence systems in Shimmy-DS.
///
/// Philosophy: A recursive narrative system must breathe like an organism,
/// but protect like an immune system.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// RIC Assertiveness Protocol
/// Defines how aggressively the integrity core intervenes in recursive processes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RICMode {
    /// Log only - observe but never interfere
    Passive,
    /// Annotate issues - provide warnings and suggestions
    Moderate,
    /// Block or reroute recursion - actively prevent problematic patterns
    Active,
}

impl Default for RICMode {
    fn default() -> Self {
        RICMode::Passive
    }
}

/// Status returned by individual recursive subsystems
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsightStatus {
    /// Continue processing - system is healthy
    Continue,
    /// Block further recursion - potential infinite loop detected
    Block,
    /// Suggest intervention - minor issue detected
    Suggest,
    /// Stalled - system has reached saturation without progress
    Stalled,
}

/// Decision made by the arbitration engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RICDecision {
    /// Continue processing normally
    Continue,
    /// Halt all recursive processing immediately
    Halt,
    /// Inject continuity floor - force completion via summary
    InjectFloor,
    /// Reroute to different subsystem
    Reroute(String),
}

/// Status of RIC operations
#[derive(Debug, Clone)]
pub enum RICStatus {
    /// System is operating normally
    Healthy,
    /// System has reached saturation point
    Saturated(String),
    /// System requires intervention
    RequiresIntervention(String),
    /// System has been halted for safety
    Halted(String),
}

/// Loop Saturation Control (ZC Gate)
/// Prevents infinite recursion by tracking iteration budgets
#[derive(Debug, Clone)]
pub struct LoopSaturationController {
    pub recursion_budget: u8,
    pub initial_budget: u8,
    pub last_insight_time: Instant,
    pub total_iterations: u32,
    pub subsystem_name: String,
}

impl LoopSaturationController {
    /// Create new saturation controller with specified budget
    pub fn new(budget: u8, subsystem_name: String) -> Self {
        Self {
            recursion_budget: budget,
            initial_budget: budget,
            last_insight_time: Instant::now(),
            total_iterations: 0,
            subsystem_name,
        }
    }

    /// Attempt to consume budget for one iteration
    /// Returns true if iteration is allowed, false if saturated
    pub fn consume_iteration(&mut self) -> bool {
        self.total_iterations += 1;

        if self.recursion_budget == 0 {
            return false;
        }

        self.recursion_budget -= 1;
        true
    }

    /// Reset budget when new insight is gained
    pub fn reset_on_insight(&mut self) {
        self.recursion_budget = self.initial_budget;
        self.last_insight_time = Instant::now();
    }

    /// Check if system is saturated
    pub fn is_saturated(&self) -> bool {
        self.recursion_budget == 0
    }

    /// Get current status
    pub fn status(&self) -> RICStatus {
        if self.is_saturated() {
            RICStatus::Saturated(format!(
                "{} loop reached null recursion after {} iterations",
                self.subsystem_name, self.total_iterations
            ))
        } else {
            RICStatus::Healthy
        }
    }
}

/// CAPR Return Clamp
/// Prevents repetitive emotional loops that never progress
#[derive(Debug, Clone)]
pub struct CAPRReturnClamp {
    pub loop_counts: HashMap<String, u32>,
    pub max_returns: u32,
    pub transformation_states: HashMap<String, bool>,
}

impl CAPRReturnClamp {
    pub fn new(max_returns: u32) -> Self {
        Self {
            loop_counts: HashMap::new(),
            max_returns,
            transformation_states: HashMap::new(),
        }
    }

    /// Check if CAPR loop should be allowed to continue
    pub fn should_allow_return(&mut self, loop_id: &str) -> CAPRReturnDecision {
        let count = self.loop_counts.entry(loop_id.to_string()).or_insert(0);
        *count += 1;

        let has_transformation = self.transformation_states
            .get(loop_id)
            .copied()
            .unwrap_or(false);

        if *count > self.max_returns && !has_transformation {
            CAPRReturnDecision::Stalled
        } else {
            CAPRReturnDecision::Allow
        }
    }

    /// Mark loop as having achieved transformation
    pub fn mark_transformation(&mut self, loop_id: &str) {
        self.transformation_states.insert(loop_id.to_string(), true);
    }

    /// Reset loop tracking
    pub fn reset_loop(&mut self, loop_id: &str) {
        self.loop_counts.remove(loop_id);
        self.transformation_states.remove(loop_id);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CAPRReturnDecision {
    Allow,
    Stalled,
}

/// Character Drift Threshold Clamp
/// Filters out personality drifts below noise floor
#[derive(Debug, Clone)]
pub struct CharacterDriftClamp {
    pub drift_noise_floor: f32,
    pub drift_history: HashMap<String, Vec<f32>>,
}

impl CharacterDriftClamp {
    pub fn new(noise_floor: f32) -> Self {
        Self {
            drift_noise_floor: noise_floor,
            drift_history: HashMap::new(),
        }
    }

    /// Check if drift is significant enough to report
    pub fn is_significant_drift(&mut self, character_id: &str, drift_value: f32) -> bool {
        // Record drift value
        let history = self.drift_history
            .entry(character_id.to_string())
            .or_insert_with(Vec::new);
        history.push(drift_value);

        // Keep only recent history (last 10 measurements)
        if history.len() > 10 {
            history.remove(0);
        }

        // Check if drift exceeds noise floor
        drift_value.abs() > self.drift_noise_floor
    }

    /// Get average drift trend for character
    pub fn get_drift_trend(&self, character_id: &str) -> f32 {
        if let Some(history) = self.drift_history.get(character_id) {
            if history.is_empty() {
                return 0.0;
            }
            history.iter().sum::<f32>() / history.len() as f32
        } else {
            0.0
        }
    }
}

/// Recursive Arbitration Engine
/// Provides consensus-based decision making across subsystems
#[derive(Debug, Clone)]
pub struct InsightArbitrator {
    pub votes: Vec<InsightStatus>,
    pub vote_history: Vec<(String, InsightStatus, Instant)>,
    pub consensus_threshold: usize,
}

impl InsightArbitrator {
    pub fn new() -> Self {
        Self {
            votes: Vec::new(),
            vote_history: Vec::new(),
            consensus_threshold: 2, // At least 2 systems must agree to block
        }
    }

    /// Add vote from a subsystem
    pub fn add_vote(&mut self, subsystem: &str, status: InsightStatus) {
        self.votes.push(status.clone());
        self.vote_history.push((subsystem.to_string(), status, Instant::now()));
    }

    /// Clear current votes for new round
    pub fn clear_votes(&mut self) {
        self.votes.clear();
    }

    /// Make arbitration decision based on current votes
    pub fn arbitrate(&self) -> RICDecision {
        let block_count = self.votes.iter()
            .filter(|s| **s == InsightStatus::Block)
            .count();

        let stalled_count = self.votes.iter()
            .filter(|s| **s == InsightStatus::Stalled)
            .count();

        let suggest_count = self.votes.iter()
            .filter(|s| **s == InsightStatus::Suggest)
            .count();

        // Consensus logic
        if block_count >= self.consensus_threshold {
            RICDecision::Halt
        } else if self.votes.len() > 0 && stalled_count == self.votes.len() {
            RICDecision::InjectFloor
        } else if suggest_count > self.votes.len() / 2 {
            RICDecision::Reroute("suggested_alternative".to_string())
        } else {
            RICDecision::Continue
        }
    }

    /// Get recent vote history for logging
    pub fn get_recent_votes(&self, since: Duration) -> Vec<(String, InsightStatus, Instant)> {
        let cutoff = Instant::now() - since;
        self.vote_history.iter()
            .filter(|(_, _, time)| *time > cutoff)
            .cloned()
            .collect()
    }
}

/// Main Recursive Integrity Core
/// Coordinates all integrity mechanisms
#[derive(Debug)]
pub struct RecursiveIntegrityCore {
    pub mode: RICMode,
    pub arbitrator: InsightArbitrator,
    pub capr_clamp: CAPRReturnClamp,
    pub drift_clamp: CharacterDriftClamp,
    pub saturation_controllers: HashMap<String, LoopSaturationController>,
    pub intervention_count: u32,
    pub start_time: Instant,
}

impl RecursiveIntegrityCore {
    /// Create new RIC with specified mode
    pub fn new(mode: RICMode) -> Self {
        Self {
            mode,
            arbitrator: InsightArbitrator::new(),
            capr_clamp: CAPRReturnClamp::new(5), // Max 5 returns without transformation
            drift_clamp: CharacterDriftClamp::new(0.05), // 5% noise floor
            saturation_controllers: HashMap::new(),
            intervention_count: 0,
            start_time: Instant::now(),
        }
    }

    /// Register a new subsystem for saturation control
    pub fn register_subsystem(&mut self, name: &str, budget: u8) {
        let controller = LoopSaturationController::new(budget, name.to_string());
        self.saturation_controllers.insert(name.to_string(), controller);
    }

    /// Check if subsystem can continue iterating
    pub fn can_iterate(&mut self, subsystem: &str) -> bool {
        if let Some(controller) = self.saturation_controllers.get_mut(subsystem) {
            controller.consume_iteration()
        } else {
            // Unknown subsystem - register with default budget
            self.register_subsystem(subsystem, 10);
            true
        }
    }

    /// Reset subsystem budget when insight is gained
    pub fn reset_subsystem_on_insight(&mut self, subsystem: &str) {
        if let Some(controller) = self.saturation_controllers.get_mut(subsystem) {
            controller.reset_on_insight();
        }
    }

    /// Add vote from subsystem
    pub fn vote(&mut self, subsystem: &str, status: InsightStatus) {
        self.arbitrator.add_vote(subsystem, status);
    }

    /// Make final arbitration decision
    pub fn arbitrate(&mut self) -> RICDecision {
        let decision = self.arbitrator.arbitrate();

        // Count interventions
        match decision {
            RICDecision::Halt | RICDecision::InjectFloor | RICDecision::Reroute(_) => {
                self.intervention_count += 1;
            }
            _ => {}
        }

        self.arbitrator.clear_votes();
        decision
    }

    /// Check CAPR return permission
    pub fn check_capr_return(&mut self, loop_id: &str) -> CAPRReturnDecision {
        self.capr_clamp.should_allow_return(loop_id)
    }

    /// Mark CAPR transformation
    pub fn mark_capr_transformation(&mut self, loop_id: &str) {
        self.capr_clamp.mark_transformation(loop_id);
    }

    /// Check character drift significance
    pub fn is_significant_character_drift(&mut self, character_id: &str, drift: f32) -> bool {
        match self.mode {
            RICMode::Passive => {
                // Always record but never block
                self.drift_clamp.is_significant_drift(character_id, drift);
                false
            }
            RICMode::Moderate | RICMode::Active => {
                self.drift_clamp.is_significant_drift(character_id, drift)
            }
        }
    }

    /// Get system health summary
    pub fn health_summary(&self) -> RICHealthSummary {
        let saturated_systems: Vec<String> = self.saturation_controllers
            .iter()
            .filter(|(_, controller)| controller.is_saturated())
            .map(|(name, _)| name.clone())
            .collect();

        let total_iterations: u32 = self.saturation_controllers
            .values()
            .map(|c| c.total_iterations)
            .sum();

        RICHealthSummary {
            mode: self.mode,
            intervention_count: self.intervention_count,
            saturated_systems,
            total_iterations,
            uptime: self.start_time.elapsed(),
            active_subsystems: self.saturation_controllers.len(),
        }
    }
}

/// Health summary for RIC system
#[derive(Debug, Clone)]
pub struct RICHealthSummary {
    pub mode: RICMode,
    pub intervention_count: u32,
    pub saturated_systems: Vec<String>,
    pub total_iterations: u32,
    pub uptime: Duration,
    pub active_subsystems: usize,
}

/// Continuity floor response when recursion fails
#[derive(Debug, Clone)]
pub struct ContinuityFloorResponse {
    pub summary: String,
    pub completion_guarantee: bool,
    pub fallback_reason: String,
}

impl ContinuityFloorResponse {
    pub fn new(reason: &str) -> Self {
        Self {
            summary: "Recursive pressure saturated. Obligations fulfilled by summary.".to_string(),
            completion_guarantee: true,
            fallback_reason: reason.to_string(),
        }
    }

    pub fn with_summary(reason: &str, summary: String) -> Self {
        Self {
            summary,
            completion_guarantee: true,
            fallback_reason: reason.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ric_mode_default() {
        assert_eq!(RICMode::default(), RICMode::Passive);
    }

    #[test]
    fn test_loop_saturation_controller() {
        let mut controller = LoopSaturationController::new(3, "test".to_string());

        assert!(controller.consume_iteration()); // 1
        assert!(controller.consume_iteration()); // 2
        assert!(controller.consume_iteration()); // 3
        assert!(!controller.consume_iteration()); // 4 - should fail

        assert!(controller.is_saturated());
    }

    #[test]
    fn test_capr_return_clamp() {
        let mut clamp = CAPRReturnClamp::new(2);

        // Should allow initial returns
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);

        // Should stall after max returns without transformation
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Stalled);

        // Mark transformation and try again
        clamp.mark_transformation("loop1");
        clamp.reset_loop("loop1");
        assert_eq!(clamp.should_allow_return("loop1"), CAPRReturnDecision::Allow);
    }

    #[test]
    fn test_character_drift_clamp() {
        let mut clamp = CharacterDriftClamp::new(0.1);

        // Small drift should not be significant
        assert!(!clamp.is_significant_drift("char1", 0.05));

        // Large drift should be significant
        assert!(clamp.is_significant_drift("char1", 0.15));
    }

    #[test]
    fn test_insight_arbitrator() {
        let mut arbitrator = InsightArbitrator::new();

        // Single block vote should not halt
        arbitrator.add_vote("system1", InsightStatus::Block);
        assert_eq!(arbitrator.arbitrate(), RICDecision::Continue);

        // Two block votes should halt
        arbitrator.add_vote("system1", InsightStatus::Block);
        arbitrator.add_vote("system2", InsightStatus::Block);
        assert_eq!(arbitrator.arbitrate(), RICDecision::Halt);
    }

    #[test]
    fn test_recursive_integrity_core() {
        let mut ric = RecursiveIntegrityCore::new(RICMode::Active);

        // Register subsystem
        ric.register_subsystem("test_system", 2);

        // Should allow iterations until budget exhausted
        assert!(ric.can_iterate("test_system"));
        assert!(ric.can_iterate("test_system"));
        assert!(!ric.can_iterate("test_system"));
    }

    #[test]
    fn test_continuity_floor_response() {
        let response = ContinuityFloorResponse::new("saturation");
        assert!(response.completion_guarantee);
        assert_eq!(response.fallback_reason, "saturation");
    }
}