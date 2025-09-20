/// Stability Tracing - Telemetry Orchestrator for Recursive Narrative Intelligence
///
/// This module serves as the high-level telemetry hub for the Shimmy-DS recursive
/// narrative system. It aggregates stability data from all subsystems and provides
/// a unified API for monitoring system health.
///
/// Philosophy:
/// - stability_log.rs is the journal (records raw data)
/// - stability_tracing.rs is the dashboard (interprets and orchestrates)
/// Together they form the nervous system of the recursive author.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Re-export core logging functionality from stability_log
pub use crate::stability_log::{
    log_stability_update, log_ric_decision, log_rip_ric_fusion,
    StabilityLogger, RICLogEntry as CoreRICLogEntry, RIPRICFusionLogEntry as CoreRIPRICFusionLogEntry,
    global_stability_logger, obli_select_telemetry
};

use crate::recursive_integrity_core::{RICDecision, RICMode, InsightStatus};

/// High-level RIC arbitration log entry for telemetry aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RICLogEntry {
    /// Chapter when decision was made
    pub chapter: u32,
    /// RIC decision (Continue, Halt, InjectFloor, Reroute)
    pub decision: String,
    /// Subsystem votes
    pub votes: Vec<String>,
    /// Systems that reached saturation
    pub saturated_systems: Vec<String>,
    /// RIC operating mode
    pub ric_mode: String,
    /// Number of interventions so far
    pub intervention_count: u32,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl RICLogEntry {
    pub fn new(
        chapter: u32,
        decision: RICDecision,
        votes: &[(String, InsightStatus)],
        ric_mode: RICMode,
        intervention_count: u32,
        saturated_systems: Vec<String>,
    ) -> Self {
        let decision_str = match decision {
            RICDecision::Continue => "Continue".to_string(),
            RICDecision::Halt => "Halt".to_string(),
            RICDecision::InjectFloor => "InjectFloor".to_string(),
            RICDecision::Reroute(ref route) => format!("Reroute({})", route),
        };

        let ric_mode_str = match ric_mode {
            RICMode::Passive => "Passive".to_string(),
            RICMode::Moderate => "Moderate".to_string(),
            RICMode::Active => "Active".to_string(),
        };

        let vote_strings: Vec<String> = votes.iter()
            .map(|(system, status)| {
                let status_str = match status {
                    InsightStatus::Continue => "Continue",
                    InsightStatus::Block => "Block",
                    InsightStatus::Suggest => "Suggest",
                    InsightStatus::Stalled => "Stalled",
                };
                format!("{}:{}", system, status_str)
            })
            .collect();

        Self {
            chapter,
            decision: decision_str,
            votes: vote_strings,
            saturated_systems,
            ric_mode: ric_mode_str,
            intervention_count,
            timestamp: Utc::now(),
        }
    }
}

/// RIP+RIC fusion telemetry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RIPRICFusionLogEntry {
    /// Chapter when fusion event occurred
    pub chapter: u32,
    /// Scene within chapter (if applicable)
    pub scene: Option<u32>,
    /// RIP system status
    pub rip_status: String,
    /// RIC system status
    pub ric_status: String,
    /// Detected narrative pathogens
    pub pathogen_detections: Vec<String>,
    /// Overall system health score (0.0-1.0)
    pub health_score: f32,
    /// Whether RIP and RIC are in sync
    pub sync_ok: bool,
    /// Cross-system conflicts detected
    pub cross_system_conflicts: u32,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl RIPRICFusionLogEntry {
    pub fn new(
        chapter: u32,
        scene: Option<u32>,
        health_score: f32,
        pathogen_detections: Vec<String>,
        cross_system_conflicts: u32,
    ) -> Self {
        Self {
            chapter,
            scene,
            rip_status: "Active".to_string(), // Would be determined by actual RIP state
            ric_status: "Active".to_string(), // Would be determined by actual RIC state
            pathogen_detections,
            health_score,
            sync_ok: health_score > 0.7, // Simple heuristic
            cross_system_conflicts,
            timestamp: Utc::now(),
        }
    }
}

/// ObliSelect obligation management telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObliSelectTelemetry {
    /// Event type (ADD, REMOVE, FULFILL, SELECT, etc.)
    pub event: String,
    /// Obligation identifier
    pub obligation_id: String,
    /// Additional notes or context
    pub notes: String,
    /// Chapter when event occurred
    pub chapter: u32,
    /// Performance metrics (if applicable)
    pub performance_ms: Option<u64>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ObliSelectTelemetry {
    pub fn new(event: &str, obligation_id: &str, notes: &str, chapter: u32) -> Self {
        Self {
            event: event.to_string(),
            obligation_id: obligation_id.to_string(),
            notes: notes.to_string(),
            chapter,
            performance_ms: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_performance(mut self, performance_ms: u64) -> Self {
        self.performance_ms = Some(performance_ms);
        self
    }
}

/// Aggregated stability report for a chapter/scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityReport {
    /// Chapter number
    pub chapter: u32,
    /// Scene number (if applicable)
    pub scene: Option<u32>,
    /// RIC arbitration entry (if any decisions made)
    pub ric: Option<RICLogEntry>,
    /// RIP+RIC fusion entry (if fusion events occurred)
    pub ripric: Option<RIPRICFusionLogEntry>,
    /// Obligation management events
    pub obligations: Vec<ObliSelectTelemetry>,
    /// Drift stabilizer warnings
    pub drift_warnings: Vec<String>,
    /// General stability context
    pub context: HashMap<String, Value>,
    /// Timestamp when report was generated
    pub timestamp: DateTime<Utc>,
}

impl StabilityReport {
    pub fn new(chapter: u32, scene: Option<u32>) -> Self {
        Self {
            chapter,
            scene,
            ric: None,
            ripric: None,
            obligations: Vec::new(),
            drift_warnings: Vec::new(),
            context: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    /// Add RIC arbitration data to the report
    pub fn with_ric(mut self, ric_entry: RICLogEntry) -> Self {
        self.ric = Some(ric_entry);
        self
    }

    /// Add RIP+RIC fusion data to the report
    pub fn with_ripric(mut self, ripric_entry: RIPRICFusionLogEntry) -> Self {
        self.ripric = Some(ripric_entry);
        self
    }

    /// Add obligation telemetry
    pub fn add_obligation_event(&mut self, event: ObliSelectTelemetry) {
        self.obligations.push(event);
    }

    /// Add drift warning
    pub fn add_drift_warning(&mut self, warning: String) {
        self.drift_warnings.push(warning);
    }

    /// Add context data
    pub fn add_context(&mut self, key: &str, value: Value) {
        self.context.insert(key.to_string(), value);
    }

    /// Generate a summary string of the stability report
    pub fn summarize(&self) -> String {
        let ric_summary = self.ric.as_ref()
            .map(|r| r.decision.as_str())
            .unwrap_or("None");

        let ripric_summary = self.ripric.as_ref()
            .map(|r| format!("{:.2}", r.health_score))
            .unwrap_or("None".to_string());

        let scene_str = self.scene
            .map(|s| format!(".{}", s))
            .unwrap_or_default();

        format!(
            "Chapter{}{} | RIC: {} | RIPRIC: {} | Obligations: {} | Drift Warnings: {}",
            self.chapter,
            scene_str,
            ric_summary,
            ripric_summary,
            self.obligations.len(),
            self.drift_warnings.len(),
        )
    }

    /// Check if the system appears stable based on the report
    pub fn is_stable(&self) -> bool {
        // Simple stability heuristic
        let ric_stable = self.ric.as_ref()
            .map(|r| r.decision == "Continue")
            .unwrap_or(true);

        let ripric_stable = self.ripric.as_ref()
            .map(|r| r.health_score > 0.7)
            .unwrap_or(true);

        let few_warnings = self.drift_warnings.len() < 3;

        ric_stable && ripric_stable && few_warnings
    }
}

/// Central entrypoint for stability tracing
pub fn trace_stability(report: &StabilityReport) -> Result<(), Box<dyn std::error::Error>> {
    // Forward to low-level stability logger
    log_stability_update(
        report.chapter,
        &crate::recursive_drift_stabilizer::DriftStabilityState::new(report.chapter),
        if report.drift_warnings.is_empty() {
            None
        } else {
            Some(report.drift_warnings.join("; "))
        },
        false, // injection_performed - would need actual context
        Some(serde_json::to_value(&report.context)?),
    )?;

    // Log RIC decisions if present
    if let Some(ref ric_entry) = report.ric {
        let votes: Vec<(String, InsightStatus)> = ric_entry.votes.iter()
            .filter_map(|vote_str| {
                let parts: Vec<&str> = vote_str.split(':').collect();
                if parts.len() == 2 {
                    let status = match parts[1] {
                        "Continue" => InsightStatus::Continue,
                        "Block" => InsightStatus::Block,
                        "Suggest" => InsightStatus::Suggest,
                        "Stalled" => InsightStatus::Stalled,
                        _ => return None,
                    };
                    Some((parts[0].to_string(), status))
                } else {
                    None
                }
            })
            .collect();

        let decision = match ric_entry.decision.as_str() {
            "Continue" => RICDecision::Continue,
            "Halt" => RICDecision::Halt,
            "InjectFloor" => RICDecision::InjectFloor,
            s if s.starts_with("Reroute(") => {
                let route = s.trim_start_matches("Reroute(").trim_end_matches(')');
                RICDecision::Reroute(route.to_string())
            },
            _ => RICDecision::Continue,
        };

        let ric_mode = match ric_entry.ric_mode.as_str() {
            "Passive" => RICMode::Passive,
            "Moderate" => RICMode::Moderate,
            "Active" => RICMode::Active,
            _ => RICMode::Passive,
        };

        log_ric_decision(
            report.chapter,
            decision,
            &votes,
            ric_mode,
            ric_entry.intervention_count,
            ric_entry.saturated_systems.clone(),
            format!("RIC arbitration decision: {}", ric_entry.decision), // reason
            Some(serde_json::json!({"timestamp": ric_entry.timestamp})), // context
        )?;
    }

    // Optional: forward to metrics, dashboards, or console output for debugging
    if cfg!(debug_assertions) {
        println!("[STABILITY TRACE] {}", report.summarize());
        if !report.is_stable() {
            println!("[STABILITY WARNING] System instability detected in {}", report.summarize());
        }
    }

    Ok(())
}

/// Create a new stability report for the current chapter/scene
pub fn new_stability_report(chapter: u32, scene: Option<u32>) -> StabilityReport {
    StabilityReport::new(chapter, scene)
}

/// Convenience function for logging obligation events
pub fn log_obligation_event(
    event: &str,
    obligation_id: &str,
    notes: &str,
    chapter: u32,
) {
    let telemetry = ObliSelectTelemetry::new(event, obligation_id, notes, chapter);

    // Forward to ObliSelect telemetry subsystem
    obli_select_telemetry::log_obligation_lifecycle_event(
        event,
        obligation_id,
        Some(notes),
        None, // category - would need actual context
        None, // urgency - would need actual context
        None, // fulfillment_progress - would need actual context
        chapter,
    );
}

/// Convenience function for logging performance warnings
pub fn log_performance_warning(
    operation: &str,
    actual_ms: u64,
    threshold_ms: u64,
    chapter: u32,
    obligation_count: usize,
) {
    obli_select_telemetry::log_performance_warning(
        operation,
        actual_ms,
        threshold_ms,
        chapter,
        obligation_count,
    );
}

/// Get a JSON summary of recent telemetry for the current chapter
pub fn get_telemetry_summary(chapter: u32) -> Value {
    obli_select_telemetry::get_logging_summary(chapter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stability_report_creation() {
        let report = StabilityReport::new(5, Some(2));
        assert_eq!(report.chapter, 5);
        assert_eq!(report.scene, Some(2));
        assert!(report.ric.is_none());
        assert!(report.ripric.is_none());
        assert!(report.obligations.is_empty());
        assert!(report.drift_warnings.is_empty());
    }

    #[test]
    fn test_stability_report_summary() {
        let mut report = StabilityReport::new(3, None);
        report.add_drift_warning("Test warning".to_string());

        let summary = report.summarize();
        assert!(summary.contains("Chapter3"));
        assert!(summary.contains("RIC: None"));
        assert!(summary.contains("Drift Warnings: 1"));
    }

    #[test]
    fn test_stability_check() {
        let report = StabilityReport::new(1, None);
        assert!(report.is_stable()); // Empty report should be stable

        let mut unstable_report = StabilityReport::new(1, None);
        unstable_report.add_drift_warning("Warning 1".to_string());
        unstable_report.add_drift_warning("Warning 2".to_string());
        unstable_report.add_drift_warning("Warning 3".to_string());
        assert!(unstable_report.is_stable()); // Still stable at exactly 3 warnings

        unstable_report.add_drift_warning("Warning 4".to_string());
        assert!(!unstable_report.is_stable()); // Now unstable with > 3 warnings
    }

    #[test]
    fn test_ric_log_entry_creation() {
        let votes = vec![
            ("system1".to_string(), InsightStatus::Continue),
            ("system2".to_string(), InsightStatus::Block),
        ];

        let entry = RICLogEntry::new(
            5,
            RICDecision::Halt,
            &votes,
            RICMode::Active,
            3,
            vec!["saturated_system".to_string()],
        );

        assert_eq!(entry.chapter, 5);
        assert_eq!(entry.decision, "Halt");
        assert_eq!(entry.ric_mode, "Active");
        assert_eq!(entry.intervention_count, 3);
        assert_eq!(entry.votes.len(), 2);
        assert!(entry.votes.contains(&"system1:Continue".to_string()));
        assert!(entry.votes.contains(&"system2:Block".to_string()));
    }

    #[test]
    fn test_obligation_telemetry() {
        let telemetry = ObliSelectTelemetry::new(
            "ADD",
            "test_obligation_001",
            "Test obligation added",
            5,
        ).with_performance(150);

        assert_eq!(telemetry.event, "ADD");
        assert_eq!(telemetry.obligation_id, "test_obligation_001");
        assert_eq!(telemetry.chapter, 5);
        assert_eq!(telemetry.performance_ms, Some(150));
    }
}