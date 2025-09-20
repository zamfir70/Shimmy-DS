/// 📊 Stability Log - Advanced Reporting for Recursive Drift Stabilizer
///
/// This module provides specialized logging and reporting capabilities
/// for the Recursive Drift Stabilizer system.

use crate::recursive_drift_stabilizer::{DriftStabilityState, DriftStabilizerConfig};
use crate::recursive_integrity_core::{
    RICDecision, RICHealthSummary, RICMode, InsightStatus
};
use crate::recursive_narrative_assistant::{
    UnifiedArbitrationDecision, RIPRICFusionHealth
};
use crate::telemetry::{PulseTrace, Pulse, PulseTraceHealthStats};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Represents a stability log entry with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityLogEntry {
    /// Timestamp of the log entry
    pub timestamp: DateTime<Utc>,
    /// Chapter number when this entry was created
    pub chapter: u32,
    /// The complete drift stability state at this point
    pub stability_state: DriftStabilityState,
    /// Any warnings that were active
    pub warnings: Option<String>,
    /// Whether drift injection was performed
    pub injection_performed: bool,
    /// Additional context information
    pub context: serde_json::Value,
}

impl StabilityLogEntry {
    /// Creates a new stability log entry
    pub fn new(
        chapter: u32,
        stability_state: DriftStabilityState,
        warnings: Option<String>,
        injection_performed: bool,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            chapter,
            stability_state,
            warnings,
            injection_performed,
            context: serde_json::Value::Null,
        }
    }

    /// Adds context information to the log entry
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    /// Formats the entry as a human-readable summary line
    pub fn format_summary(&self) -> String {
        let warning_status = match &self.warnings {
            Some(_) => "⚠️ WARNINGS",
            None => "✅ STABLE",
        };

        let injection_status = if self.injection_performed {
            "🔧 INJECTED"
        } else {
            ""
        };

        format!(
            "[{}] Ch.{} - {} {} - Stale:{} Decay:{:.2} Theme:{:.2}",
            self.timestamp.format("%m-%d %H:%M"),
            self.chapter,
            warning_status,
            injection_status,
            self.stability_state.stale_obligations,
            self.stability_state.emotional_decay_sum,
            self.stability_state.theme_drift_score
        )
    }
}

/// RIC-specific log entry for tracking integrity core decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RICLogEntry {
    /// Timestamp of the log entry
    pub timestamp: DateTime<Utc>,
    /// Chapter number when this entry was created
    pub chapter: u32,
    /// RIC decision made
    pub decision: String, // Serialized version of RICDecision
    /// Votes from each subsystem
    pub subsystem_votes: HashMap<String, String>, // subsystem -> InsightStatus as string
    /// Mode RIC was operating in
    pub ric_mode: RICMode,
    /// Number of interventions so far
    pub intervention_count: u32,
    /// Saturated systems
    pub saturated_systems: Vec<String>,
    /// Description of what caused this decision
    pub reason: String,
    /// Additional context
    pub context: serde_json::Value,
}

impl RICLogEntry {
    /// Creates a new RIC log entry
    pub fn new(
        chapter: u32,
        decision: RICDecision,
        votes: &[(String, InsightStatus)],
        ric_mode: RICMode,
        intervention_count: u32,
        reason: String,
    ) -> Self {
        let decision_str = match decision {
            RICDecision::Continue => "Continue".to_string(),
            RICDecision::Halt => "Halt".to_string(),
            RICDecision::InjectFloor => "InjectFloor".to_string(),
            RICDecision::Reroute(ref alt) => format!("Reroute({})", alt),
        };

        let votes_map: HashMap<String, String> = votes.iter()
            .map(|(system, status)| {
                let status_str = match status {
                    InsightStatus::Continue => "Continue",
                    InsightStatus::Block => "Block",
                    InsightStatus::Suggest => "Suggest",
                    InsightStatus::Stalled => "Stalled",
                };
                (system.clone(), status_str.to_string())
            })
            .collect();

        Self {
            timestamp: Utc::now(),
            chapter,
            decision: decision_str,
            subsystem_votes: votes_map,
            ric_mode,
            intervention_count,
            saturated_systems: Vec::new(),
            reason,
            context: serde_json::Value::Null,
        }
    }

    /// Adds saturated systems info
    pub fn with_saturated_systems(mut self, systems: Vec<String>) -> Self {
        self.saturated_systems = systems;
        self
    }

    /// Adds context information
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    /// Formats this entry for text logging
    pub fn format_for_text(&self) -> String {
        let votes_str = self.subsystem_votes.iter()
            .map(|(system, vote)| format!("{}:{}", system, vote))
            .collect::<Vec<_>>()
            .join(", ");

        let saturated_str = if self.saturated_systems.is_empty() {
            "none".to_string()
        } else {
            self.saturated_systems.join(", ")
        };

        format!(
            "[RIC] {} | Ch:{} | Decision:{} | Votes:[{}] | Mode:{:?} | Interventions:{} | Saturated:[{}] | {}",
            self.timestamp.format("%H:%M:%S"),
            self.chapter,
            self.decision,
            votes_str,
            self.ric_mode,
            self.intervention_count,
            saturated_str,
            self.reason
        )
    }
}

/// RIP+RIC unified fusion log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RIPRICFusionLogEntry {
    /// Timestamp of the log entry
    pub timestamp: DateTime<Utc>,
    /// Chapter number when this entry was created
    pub chapter: u32,
    /// Scene number if available
    pub scene: Option<u32>,
    /// Unified arbitration decision made
    pub unified_decision: String, // Serialized UnifiedArbitrationDecision
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
    /// Current recursion budget from ZC gates
    pub current_recursion_budget: u32,
    /// Loop saturation detection state
    pub loop_saturation_detected: bool,
    /// RIP Python process connection state
    pub rip_process_healthy: bool,
    /// Narrative text analyzed
    pub analyzed_text: String,
    /// Narrative context
    pub narrative_context: String,
    /// Overall fusion health score
    pub overall_fusion_health: f32,
    /// Additional context
    pub context: serde_json::Value,
}

impl RIPRICFusionLogEntry {
    /// Creates a new RIP+RIC fusion log entry
    pub fn new(
        chapter: u32,
        scene: Option<u32>,
        decision: UnifiedArbitrationDecision,
        fusion_health: RIPRICFusionHealth,
        analyzed_text: String,
        narrative_context: String,
    ) -> Self {
        let decision_str = match decision {
            UnifiedArbitrationDecision::ContinueRecursion { consensus_confidence, .. } => {
                format!("ContinueRecursion(confidence:{:.2})", consensus_confidence)
            }
            UnifiedArbitrationDecision::RIPConstraintHalt { ref failed_ligands, .. } => {
                format!("RIPConstraintHalt(ligands:{})", failed_ligands.len())
            }
            UnifiedArbitrationDecision::RICConsensusHalt { ref halt_reason, .. } => {
                format!("RICConsensusHalt({})", halt_reason)
            }
            UnifiedArbitrationDecision::PathogenDetectionHalt { threat_level, .. } => {
                format!("PathogenDetectionHalt(threat:{:.2})", threat_level)
            }
            UnifiedArbitrationDecision::LoopSaturationHalt { budget_exhausted, .. } => {
                format!("LoopSaturationHalt(budget_exhausted:{})", budget_exhausted)
            }
            UnifiedArbitrationDecision::UnifiedContinuityFloor { ref fusion_reason, .. } => {
                format!("UnifiedContinuityFloor({})", fusion_reason)
            }
        };

        Self {
            timestamp: Utc::now(),
            chapter,
            scene,
            unified_decision: decision_str,
            rip_genome_health: fusion_health.rip_genome_health,
            rip_guard_health: fusion_health.rip_guard_health,
            rip_pathogen_threat: fusion_health.rip_pathogen_threat,
            ric_consensus_health: fusion_health.ric_consensus_health,
            ric_saturation_level: fusion_health.ric_saturation_level,
            current_recursion_budget: fusion_health.current_recursion_budget,
            loop_saturation_detected: fusion_health.loop_saturation_detected,
            rip_process_healthy: fusion_health.rip_process_healthy,
            analyzed_text,
            narrative_context,
            overall_fusion_health: fusion_health.overall_fusion_health,
            context: serde_json::Value::Null,
        }
    }

    /// Adds context information
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    /// Formats this entry for text logging
    pub fn format_for_text(&self) -> String {
        let health_status = match self.overall_fusion_health {
            h if h >= 0.8 => "🟢 EXCELLENT",
            h if h >= 0.6 => "🟡 GOOD",
            h if h >= 0.4 => "🟠 MODERATE",
            h if h >= 0.2 => "🔴 POOR",
            _ => "🚨 CRITICAL",
        };

        let process_status = if self.rip_process_healthy { "✅" } else { "❌" };
        let saturation_status = if self.loop_saturation_detected { "⚠️ SAT" } else { "✅" };

        format!(
            "[RIP+RIC FUSION] {} | Ch:{}{} | Decision:{} | Health:{} ({:.2}) | RIP:{:.2}/{:.2}/{:.2} | RIC:{:.2}/{:.2} | Budget:{} | Process:{} Sat:{} | Text:{}...",
            self.timestamp.format("%H:%M:%S"),
            self.chapter,
            self.scene.map_or("".to_string(), |s| format!(".{}", s)),
            self.unified_decision,
            health_status,
            self.overall_fusion_health,
            self.rip_genome_health,
            self.rip_guard_health,
            self.rip_pathogen_threat,
            self.ric_consensus_health,
            self.ric_saturation_level,
            self.current_recursion_budget,
            process_status,
            saturation_status,
            self.analyzed_text.chars().take(50).collect::<String>()
        )
    }
}

/// Manages stability logging for the drift stabilizer
pub struct StabilityLogger {
    /// Path to the stability log file
    log_path: PathBuf,
    /// Path to the JSON stability log
    json_log_path: PathBuf,
    /// Path to the RIC-specific log file
    ric_log_path: PathBuf,
    /// Path to the RIC JSON log
    ric_json_log_path: PathBuf,
    /// Path to the unified RIP+RIC fusion log
    rip_ric_fusion_log_path: PathBuf,
    /// Path to the RIP+RIC fusion JSON log
    rip_ric_fusion_json_path: PathBuf,
    /// Whether logging is enabled
    enabled: bool,
    /// Maximum number of entries to keep in memory
    max_memory_entries: usize,
    /// In-memory cache of recent entries
    recent_entries: Vec<StabilityLogEntry>,
    /// In-memory cache of recent RIC entries
    recent_ric_entries: Vec<RICLogEntry>,
    /// In-memory cache of recent RIP+RIC fusion entries
    recent_fusion_entries: Vec<RIPRICFusionLogEntry>,
}

impl StabilityLogger {
    /// Creates a new stability logger with default paths
    pub fn new() -> Self {
        Self {
            log_path: PathBuf::from("logs/stability.log"),
            json_log_path: PathBuf::from("logs/stability.json"),
            ric_log_path: PathBuf::from("logs/ric.log"),
            ric_json_log_path: PathBuf::from("logs/ric.json"),
            rip_ric_fusion_log_path: PathBuf::from("logs/rip_ric_fusion.log"),
            rip_ric_fusion_json_path: PathBuf::from("logs/rip_ric_fusion.json"),
            enabled: true,
            max_memory_entries: 100,
            recent_entries: Vec::new(),
            recent_ric_entries: Vec::new(),
            recent_fusion_entries: Vec::new(),
        }
    }

    /// Creates a new stability logger with custom paths
    pub fn with_paths(text_path: impl AsRef<Path>, json_path: impl AsRef<Path>) -> Self {
        let base_dir = text_path.as_ref().parent().unwrap_or(Path::new("logs"));
        Self {
            log_path: text_path.as_ref().to_path_buf(),
            json_log_path: json_path.as_ref().to_path_buf(),
            ric_log_path: base_dir.join("ric.log"),
            ric_json_log_path: base_dir.join("ric.json"),
            rip_ric_fusion_log_path: base_dir.join("rip_ric_fusion.log"),
            rip_ric_fusion_json_path: base_dir.join("rip_ric_fusion.json"),
            enabled: true,
            max_memory_entries: 100,
            recent_entries: Vec::new(),
            recent_ric_entries: Vec::new(),
            recent_fusion_entries: Vec::new(),
        }
    }

    /// Enables or disables logging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Logs a stability state update
    pub fn log_stability_update(
        &mut self,
        chapter: u32,
        state: &DriftStabilityState,
        warnings: Option<String>,
        injection_performed: bool,
        context: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let mut entry = StabilityLogEntry::new(chapter, state.clone(), warnings, injection_performed);
        if let Some(ctx) = context {
            entry = entry.with_context(ctx);
        }

        // Add to memory cache
        self.recent_entries.push(entry.clone());
        if self.recent_entries.len() > self.max_memory_entries {
            self.recent_entries.remove(0);
        }

        // Write to files
        self.write_entry(&entry)?;

        Ok(())
    }

    /// Writes an entry to both text and JSON logs
    fn write_entry(&self, entry: &StabilityLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directories exist
        if let Some(parent) = self.log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write to text log
        let mut text_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        writeln!(text_file, "{}", entry.format_summary())?;

        // Write detailed warnings if present
        if let Some(ref warnings) = entry.warnings {
            for line in warnings.lines() {
                writeln!(text_file, "    {}", line)?;
            }
        }

        // Write to JSON log
        let mut json_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.json_log_path)?;
        writeln!(json_file, "{}", serde_json::to_string(entry)?)?;

        Ok(())
    }

    /// Reads all stability log entries from the JSON file
    pub fn read_stability_history(&self) -> Result<Vec<StabilityLogEntry>, Box<dyn std::error::Error>> {
        if !self.json_log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.json_log_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if !line.trim().is_empty() {
                let entry: StabilityLogEntry = serde_json::from_str(line)?;
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Gets recent entries from memory cache
    pub fn get_recent_entries(&self, count: usize) -> &[StabilityLogEntry] {
        let start = self.recent_entries.len().saturating_sub(count);
        &self.recent_entries[start..]
    }

    /// Generates a stability trend analysis report
    pub fn generate_trend_analysis(&self, lookback_chapters: u32) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.read_stability_history()?;

        if history.is_empty() {
            return Ok("No stability history available for analysis.".to_string());
        }

        let recent_history: Vec<_> = history
            .iter()
            .filter(|entry| {
                entry.chapter >= history.last().unwrap().chapter.saturating_sub(lookback_chapters)
            })
            .collect();

        let mut report = String::new();
        report.push_str("📈 STABILITY TREND ANALYSIS\n");
        report.push_str("===========================\n\n");

        report.push_str(&format!("Analysis Period: {} chapters\n", lookback_chapters));
        report.push_str(&format!("Total Entries: {}\n", recent_history.len()));
        report.push_str(&format!("Recent Entries: {}\n\n", recent_history.len()));

        if recent_history.is_empty() {
            report.push_str("No recent entries found in the specified lookback period.\n");
            return Ok(report);
        }

        // Calculate trend statistics
        let warning_rate = recent_history
            .iter()
            .filter(|entry| entry.warnings.is_some())
            .count() as f32
            / recent_history.len() as f32;

        let injection_rate = recent_history
            .iter()
            .filter(|entry| entry.injection_performed)
            .count() as f32
            / recent_history.len() as f32;

        let avg_stale_obligations: f32 = recent_history
            .iter()
            .map(|entry| entry.stability_state.stale_obligations as f32)
            .sum::<f32>()
            / recent_history.len() as f32;

        let avg_emotional_decay: f32 = recent_history
            .iter()
            .map(|entry| entry.stability_state.emotional_decay_sum)
            .sum::<f32>()
            / recent_history.len() as f32;

        let avg_theme_drift: f32 = recent_history
            .iter()
            .map(|entry| entry.stability_state.theme_drift_score)
            .sum::<f32>()
            / recent_history.len() as f32;

        // Generate trend indicators
        report.push_str("📊 Trend Metrics:\n");
        report.push_str(&format!("  • Warning Rate: {:.1}% ({} warnings)\n",
            warning_rate * 100.0, recent_history.iter().filter(|e| e.warnings.is_some()).count()));
        report.push_str(&format!("  • Injection Rate: {:.1}% ({} injections)\n",
            injection_rate * 100.0, recent_history.iter().filter(|e| e.injection_performed).count()));
        report.push_str(&format!("  • Avg Stale Obligations: {:.1}\n", avg_stale_obligations));
        report.push_str(&format!("  • Avg Emotional Decay: {:.2}\n", avg_emotional_decay));
        report.push_str(&format!("  • Avg Theme Drift: {:.2}\n\n", avg_theme_drift));

        // Stability assessment
        let stability_score = 1.0 - (warning_rate * 0.4 + injection_rate * 0.3 +
            (avg_stale_obligations / 10.0).min(1.0) * 0.3);

        let stability_status = match stability_score {
            s if s >= 0.8 => "🟢 EXCELLENT",
            s if s >= 0.6 => "🟡 GOOD",
            s if s >= 0.4 => "🟠 MODERATE",
            s if s >= 0.2 => "🔴 POOR",
            _ => "🚨 CRITICAL",
        };

        report.push_str(&format!("🎯 Overall Stability: {} ({:.2}/1.0)\n\n", stability_status, stability_score));

        // Recent activity summary
        report.push_str("📋 Recent Activity:\n");
        for entry in recent_history.iter().rev().take(5) {
            report.push_str(&format!("  • {}\n", entry.format_summary()));
        }

        Ok(report)
    }

    /// Generates a chapter-by-chapter drift analysis
    pub fn generate_chapter_analysis(&self, start_chapter: u32, end_chapter: u32) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.read_stability_history()?;

        let chapter_entries: Vec<_> = history
            .iter()
            .filter(|entry| entry.chapter >= start_chapter && entry.chapter <= end_chapter)
            .collect();

        let mut report = String::new();
        report.push_str("📖 CHAPTER-BY-CHAPTER ANALYSIS\n");
        report.push_str("===============================\n\n");

        report.push_str(&format!("Chapter Range: {} - {}\n", start_chapter, end_chapter));
        report.push_str(&format!("Entries Found: {}\n\n", chapter_entries.len()));

        if chapter_entries.is_empty() {
            report.push_str("No entries found in the specified chapter range.\n");
            return Ok(report);
        }

        // Group by chapter
        let mut chapter_groups: std::collections::HashMap<u32, Vec<&StabilityLogEntry>> =
            std::collections::HashMap::new();

        for entry in chapter_entries {
            chapter_groups.entry(entry.chapter).or_default().push(entry);
        }

        // Sort chapters
        let mut chapters: Vec<_> = chapter_groups.keys().collect();
        chapters.sort();

        for &chapter in chapters {
            let entries = chapter_groups.get(&chapter).unwrap();
            let latest_entry = entries.iter().max_by_key(|e| e.timestamp).unwrap();

            report.push_str(&format!("Chapter {}: ", chapter));

            if latest_entry.warnings.is_some() {
                report.push_str("⚠️ WARNINGS");
            } else {
                report.push_str("✅ STABLE");
            }

            if latest_entry.injection_performed {
                report.push_str(" + INJECTION");
            }

            report.push_str(&format!(
                "\n  Metrics: Stale:{} Decay:{:.2} Theme:{:.2}\n",
                latest_entry.stability_state.stale_obligations,
                latest_entry.stability_state.emotional_decay_sum,
                latest_entry.stability_state.theme_drift_score
            ));

            if let Some(ref warnings) = latest_entry.warnings {
                report.push_str("  Warnings:\n");
                for line in warnings.lines() {
                    report.push_str(&format!("    {}\n", line));
                }
            }

            report.push('\n');
        }

        Ok(report)
    }

    /// Clears all stability logs
    pub fn clear_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.log_path.exists() {
            std::fs::remove_file(&self.log_path)?;
        }
        if self.json_log_path.exists() {
            std::fs::remove_file(&self.json_log_path)?;
        }
        Ok(())
    }

    /// Logs a RIC decision and voting information
    pub fn log_ric_decision(
        &mut self,
        chapter: u32,
        decision: RICDecision,
        votes: &[(String, InsightStatus)],
        ric_mode: RICMode,
        intervention_count: u32,
        saturated_systems: Vec<String>,
        reason: String,
        context: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let mut entry = RICLogEntry::new(
            chapter,
            decision,
            votes,
            ric_mode,
            intervention_count,
            reason
        ).with_saturated_systems(saturated_systems);

        if let Some(ctx) = context {
            entry = entry.with_context(ctx);
        }

        // Add to memory cache
        self.recent_ric_entries.push(entry.clone());
        if self.recent_ric_entries.len() > self.max_memory_entries {
            self.recent_ric_entries.remove(0);
        }

        // Write to text log
        self.write_ric_text_log(&entry)?;

        // Write to JSON log
        self.write_ric_json_log(&entry)?;

        Ok(())
    }

    /// Writes RIC entry to text log
    fn write_ric_text_log(&self, entry: &RICLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directory exists
        if let Some(parent) = self.ric_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ric_log_path)?;

        writeln!(file, "{}", entry.format_for_text())?;
        Ok(())
    }

    /// Writes RIC entry to JSON log
    fn write_ric_json_log(&self, entry: &RICLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directory exists
        if let Some(parent) = self.ric_json_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ric_json_log_path)?;

        let json_str = serde_json::to_string(entry)?;
        writeln!(file, "{}", json_str)?;
        Ok(())
    }

    /// Gets recent RIC entries
    pub fn get_recent_ric_entries(&self, count: usize) -> Vec<RICLogEntry> {
        self.recent_ric_entries
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Reads RIC history from JSON log file
    pub fn read_ric_history(&self) -> Result<Vec<RICLogEntry>, Box<dyn std::error::Error>> {
        if !self.ric_json_log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.ric_json_log_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if !line.trim().is_empty() {
                if let Ok(entry) = serde_json::from_str::<RICLogEntry>(line) {
                    entries.push(entry);
                }
            }
        }

        Ok(entries)
    }

    /// Generates RIC trend analysis report
    pub fn generate_ric_analysis(&self, entry_limit: usize) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.read_ric_history()?;
        let recent_history: Vec<_> = history.iter().rev().take(entry_limit).collect();

        let mut report = String::new();
        report.push_str("🔒 RIC INTEGRITY ANALYSIS REPORT\n");
        report.push_str("==================================\n\n");

        report.push_str(&format!("Total RIC Entries: {}\n", history.len()));
        report.push_str(&format!("Analyzed Recent Entries: {}\n\n", recent_history.len()));

        if recent_history.is_empty() {
            report.push_str("No RIC activity recorded.\n");
            return Ok(report);
        }

        // Decision type analysis
        let total_decisions = recent_history.len() as f32;
        let continue_count = recent_history.iter().filter(|e| e.decision == "Continue").count();
        let halt_count = recent_history.iter().filter(|e| e.decision == "Halt").count();
        let floor_count = recent_history.iter().filter(|e| e.decision == "InjectFloor").count();
        let reroute_count = recent_history.iter().filter(|e| e.decision.starts_with("Reroute")).count();

        report.push_str("📊 Decision Distribution:\n");
        report.push_str(&format!("  • Continue: {} ({:.1}%)\n", continue_count, (continue_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Halt: {} ({:.1}%)\n", halt_count, (halt_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Inject Floor: {} ({:.1}%)\n", floor_count, (floor_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Reroute: {} ({:.1}%)\n\n", reroute_count, (reroute_count as f32 / total_decisions) * 100.0));

        // Intervention analysis
        let intervention_rate = (halt_count + floor_count + reroute_count) as f32 / total_decisions;
        let avg_interventions = recent_history.iter().map(|e| e.intervention_count as f32).sum::<f32>() / total_decisions;

        report.push_str("🛡️ Intervention Analysis:\n");
        report.push_str(&format!("  • Intervention Rate: {:.1}%\n", intervention_rate * 100.0));
        report.push_str(&format!("  • Average Intervention Count: {:.1}\n\n", avg_interventions));

        // System saturation analysis
        let mut saturation_frequency: HashMap<String, usize> = HashMap::new();
        for entry in &recent_history {
            for system in &entry.saturated_systems {
                *saturation_frequency.entry(system.clone()).or_insert(0) += 1;
            }
        }

        if !saturation_frequency.is_empty() {
            report.push_str("⚠️ System Saturation Frequency:\n");
            let mut sorted_saturations: Vec<_> = saturation_frequency.iter().collect();
            sorted_saturations.sort_by(|a, b| b.1.cmp(a.1));
            for (system, count) in sorted_saturations.iter().take(5) {
                report.push_str(&format!("  • {}: {} times\n", system, count));
            }
            report.push('\n');
        }

        // Mode analysis
        let mut mode_distribution: HashMap<String, usize> = HashMap::new();
        for entry in &recent_history {
            let mode_str = format!("{:?}", entry.ric_mode);
            *mode_distribution.entry(mode_str).or_insert(0) += 1;
        }

        report.push_str("⚙️ Mode Distribution:\n");
        for (mode, count) in mode_distribution {
            report.push_str(&format!("  • {}: {} ({:.1}%)\n", mode, count, (count as f32 / total_decisions) * 100.0));
        }
        report.push('\n');

        // Recent activity
        report.push_str("📋 Recent RIC Activity:\n");
        for entry in recent_history.iter().rev().take(5) {
            report.push_str(&format!("  • {}\n", entry.format_for_text()));
        }

        Ok(report)
    }

    /// Clears all RIC logs
    pub fn clear_ric_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.ric_log_path.exists() {
            std::fs::remove_file(&self.ric_log_path)?;
        }
        if self.ric_json_log_path.exists() {
            std::fs::remove_file(&self.ric_json_log_path)?;
        }
        Ok(())
    }

    /// RIP+RIC FUSION LOGGING: Logs a unified arbitration decision
    pub fn log_rip_ric_fusion(
        &mut self,
        chapter: u32,
        scene: Option<u32>,
        decision: UnifiedArbitrationDecision,
        fusion_health: RIPRICFusionHealth,
        analyzed_text: String,
        narrative_context: String,
        context: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let mut entry = RIPRICFusionLogEntry::new(
            chapter,
            scene,
            decision,
            fusion_health,
            analyzed_text,
            narrative_context,
        );

        if let Some(ctx) = context {
            entry = entry.with_context(ctx);
        }

        // Add to memory cache
        self.recent_fusion_entries.push(entry.clone());
        if self.recent_fusion_entries.len() > self.max_memory_entries {
            self.recent_fusion_entries.remove(0);
        }

        // Write to text log
        self.write_fusion_text_log(&entry)?;

        // Write to JSON log
        self.write_fusion_json_log(&entry)?;

        Ok(())
    }

    /// Writes RIP+RIC fusion entry to text log
    fn write_fusion_text_log(&self, entry: &RIPRICFusionLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directory exists
        if let Some(parent) = self.rip_ric_fusion_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.rip_ric_fusion_log_path)?;

        writeln!(file, "{}", entry.format_for_text())?;

        // Add detailed breakdown for complex decisions
        match entry.unified_decision.as_str() {
            s if s.starts_with("PathogenDetectionHalt") => {
                writeln!(file, "    🦠 PATHOGEN DETAILS: Threat:{:.2} | RIP Health:{:.2}/{:.2} | Process:{}",
                    entry.rip_pathogen_threat, entry.rip_genome_health, entry.rip_guard_health,
                    if entry.rip_process_healthy { "OK" } else { "FAIL" })?;
            }
            s if s.starts_with("LoopSaturationHalt") => {
                writeln!(file, "    🔄 SATURATION DETAILS: Budget:{} | RIC Health:{:.2}/{:.2} | Detected:{}",
                    entry.current_recursion_budget, entry.ric_consensus_health, entry.ric_saturation_level,
                    if entry.loop_saturation_detected { "YES" } else { "NO" })?;
            }
            s if s.starts_with("UnifiedContinuityFloor") => {
                writeln!(file, "    🛡️ CONTINUITY FLOOR: Overall Health:{:.2} | Context: {}",
                    entry.overall_fusion_health,
                    entry.narrative_context.chars().take(100).collect::<String>())?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Writes RIP+RIC fusion entry to JSON log
    fn write_fusion_json_log(&self, entry: &RIPRICFusionLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directory exists
        if let Some(parent) = self.rip_ric_fusion_json_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.rip_ric_fusion_json_path)?;

        let json_str = serde_json::to_string(entry)?;
        writeln!(file, "{}", json_str)?;
        Ok(())
    }

    /// Gets recent RIP+RIC fusion entries
    pub fn get_recent_fusion_entries(&self, count: usize) -> Vec<RIPRICFusionLogEntry> {
        self.recent_fusion_entries
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Reads RIP+RIC fusion history from JSON log file
    pub fn read_fusion_history(&self) -> Result<Vec<RIPRICFusionLogEntry>, Box<dyn std::error::Error>> {
        if !self.rip_ric_fusion_json_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.rip_ric_fusion_json_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if !line.trim().is_empty() {
                if let Ok(entry) = serde_json::from_str::<RIPRICFusionLogEntry>(line) {
                    entries.push(entry);
                }
            }
        }

        Ok(entries)
    }

    /// Generates comprehensive RIP+RIC fusion analysis report
    pub fn generate_fusion_analysis(&self, entry_limit: usize) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.read_fusion_history()?;
        let recent_history: Vec<_> = history.iter().rev().take(entry_limit).collect();

        let mut report = String::new();
        report.push_str("🔗 RIP+RIC UNIFIED PROTOCOL ANALYSIS\n");
        report.push_str("=====================================\n\n");

        report.push_str(&format!("Total Fusion Entries: {}\n", history.len()));
        report.push_str(&format!("Analyzed Recent Entries: {}\n\n", recent_history.len()));

        if recent_history.is_empty() {
            report.push_str("No RIP+RIC fusion activity recorded.\n");
            return Ok(report);
        }

        // Decision type analysis
        let total_decisions = recent_history.len() as f32;
        let continue_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("ContinueRecursion")).count();
        let rip_halt_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("RIPConstraintHalt")).count();
        let ric_halt_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("RICConsensusHalt")).count();
        let pathogen_halt_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("PathogenDetectionHalt")).count();
        let saturation_halt_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("LoopSaturationHalt")).count();
        let floor_count = recent_history.iter().filter(|e| e.unified_decision.starts_with("UnifiedContinuityFloor")).count();

        report.push_str("📊 Unified Decision Distribution:\n");
        report.push_str(&format!("  • Continue Recursion: {} ({:.1}%)\n", continue_count, (continue_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • RIP Constraint Halt: {} ({:.1}%)\n", rip_halt_count, (rip_halt_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • RIC Consensus Halt: {} ({:.1}%)\n", ric_halt_count, (ric_halt_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Pathogen Detection Halt: {} ({:.1}%)\n", pathogen_halt_count, (pathogen_halt_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Loop Saturation Halt: {} ({:.1}%)\n", saturation_halt_count, (saturation_halt_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Unified Continuity Floor: {} ({:.1}%)\n\n", floor_count, (floor_count as f32 / total_decisions) * 100.0));

        // Protocol health analysis
        let avg_fusion_health = recent_history.iter().map(|e| e.overall_fusion_health).sum::<f32>() / total_decisions;
        let avg_rip_genome = recent_history.iter().map(|e| e.rip_genome_health).sum::<f32>() / total_decisions;
        let avg_rip_guard = recent_history.iter().map(|e| e.rip_guard_health).sum::<f32>() / total_decisions;
        let avg_pathogen_threat = recent_history.iter().map(|e| e.rip_pathogen_threat).sum::<f32>() / total_decisions;
        let avg_ric_consensus = recent_history.iter().map(|e| e.ric_consensus_health).sum::<f32>() / total_decisions;
        let avg_ric_saturation = recent_history.iter().map(|e| e.ric_saturation_level).sum::<f32>() / total_decisions;

        report.push_str("🏥 Protocol Health Metrics:\n");
        report.push_str(&format!("  • Overall Fusion Health: {:.2}\n", avg_fusion_health));
        report.push_str(&format!("  • RIP Constraint Genome: {:.2}\n", avg_rip_genome));
        report.push_str(&format!("  • RIP Guard Chain: {:.2}\n", avg_rip_guard));
        report.push_str(&format!("  • RIP Pathogen Threat: {:.2}\n", avg_pathogen_threat));
        report.push_str(&format!("  • RIC Consensus Health: {:.2}\n", avg_ric_consensus));
        report.push_str(&format!("  • RIC Saturation Level: {:.2}\n\n", avg_ric_saturation));

        // Recursion budget analysis
        let avg_budget = recent_history.iter().map(|e| e.current_recursion_budget as f32).sum::<f32>() / total_decisions;
        let budget_exhausted_count = recent_history.iter().filter(|e| e.current_recursion_budget == 0).count();
        let process_failure_count = recent_history.iter().filter(|e| !e.rip_process_healthy).count();
        let saturation_detected_count = recent_history.iter().filter(|e| e.loop_saturation_detected).count();

        report.push_str("⚡ System Performance:\n");
        report.push_str(&format!("  • Average Recursion Budget: {:.1}\n", avg_budget));
        report.push_str(&format!("  • Budget Exhausted: {} ({:.1}%)\n", budget_exhausted_count, (budget_exhausted_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • RIP Process Failures: {} ({:.1}%)\n", process_failure_count, (process_failure_count as f32 / total_decisions) * 100.0));
        report.push_str(&format!("  • Loop Saturation Detected: {} ({:.1}%)\n\n", saturation_detected_count, (saturation_detected_count as f32 / total_decisions) * 100.0));

        // Protocol effectiveness assessment
        let halt_rate = (rip_halt_count + ric_halt_count + pathogen_halt_count + saturation_halt_count) as f32 / total_decisions;
        let protection_efficacy = if pathogen_halt_count > 0 || saturation_halt_count > 0 {
            "🛡️ ACTIVE"
        } else {
            "🟢 STABLE"
        };

        report.push_str("🎯 Protocol Effectiveness:\n");
        report.push_str(&format!("  • Intervention Rate: {:.1}%\n", halt_rate * 100.0));
        report.push_str(&format!("  • Protection Status: {}\n", protection_efficacy));
        report.push_str(&format!("  • Overall Protocol Status: {}\n\n",
            match avg_fusion_health {
                h if h >= 0.8 => "🟢 EXCELLENT",
                h if h >= 0.6 => "🟡 GOOD",
                h if h >= 0.4 => "🟠 MODERATE",
                h if h >= 0.2 => "🔴 POOR",
                _ => "🚨 CRITICAL"
            }));

        // Recent fusion activity
        report.push_str("📋 Recent Fusion Activity:\n");
        for entry in recent_history.iter().rev().take(5) {
            report.push_str(&format!("  • {}\n", entry.format_for_text()));
        }

        Ok(report)
    }

    /// Clears all RIP+RIC fusion logs
    pub fn clear_fusion_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.rip_ric_fusion_log_path.exists() {
            std::fs::remove_file(&self.rip_ric_fusion_log_path)?;
        }
        if self.rip_ric_fusion_json_path.exists() {
            std::fs::remove_file(&self.rip_ric_fusion_json_path)?;
        }
        Ok(())
    }
}

impl Default for StabilityLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Global stability logger instance
static mut GLOBAL_STABILITY_LOGGER: Option<StabilityLogger> = None;
static STABILITY_LOGGER_INIT: std::sync::Once = std::sync::Once::new();

/// Gets the global stability logger instance
pub fn global_stability_logger() -> &'static mut StabilityLogger {
    unsafe {
        STABILITY_LOGGER_INIT.call_once(|| {
            GLOBAL_STABILITY_LOGGER = Some(StabilityLogger::new());
        });
        GLOBAL_STABILITY_LOGGER.as_mut().unwrap()
    }
}

/// Convenience function to log stability update using global logger
pub fn log_stability_update(
    chapter: u32,
    state: &DriftStabilityState,
    warnings: Option<String>,
    injection_performed: bool,
    context: Option<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    global_stability_logger().log_stability_update(chapter, state, warnings, injection_performed, context)
}

/// Convenience function to log RIC decision using global logger
pub fn log_ric_decision(
    chapter: u32,
    decision: RICDecision,
    votes: &[(String, InsightStatus)],
    ric_mode: RICMode,
    intervention_count: u32,
    saturated_systems: Vec<String>,
    reason: String,
    context: Option<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    global_stability_logger().log_ric_decision(
        chapter, decision, votes, ric_mode, intervention_count,
        saturated_systems, reason, context
    )
}

/// Convenience function to log RIP+RIC fusion decision using global logger
pub fn log_rip_ric_fusion(
    chapter: u32,
    scene: Option<u32>,
    decision: UnifiedArbitrationDecision,
    fusion_health: RIPRICFusionHealth,
    analyzed_text: String,
    narrative_context: String,
    context: Option<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    global_stability_logger().log_rip_ric_fusion(
        chapter, scene, decision, fusion_health, analyzed_text,
        narrative_context, context
    )
}

/// Telemetry integration helpers for stability logging
pub mod telemetry_integration {
    use super::*;

    /// Creates a pulse from stability log entry data
    pub fn create_pulse_from_stability_entry(
        entry: &StabilityLogEntry,
        zc_tick: usize,
    ) -> Pulse {
        let drift_hits = if entry.warnings.is_some() { 1 } else { 0 };
        let pathogens_detected = entry.stability_state.stale_obligations;

        // Calculate ADI score based on stability metrics
        let adi_score = calculate_stability_adi_score(&entry.stability_state);

        // Affect signals based on stability
        let affect_pleasure = if entry.injection_performed { -0.2 } else { 0.1 };
        let affect_coherence = 1.0 - entry.stability_state.theme_drift_score;

        Pulse::with_data(
            zc_tick,
            pathogens_detected,
            drift_hits,
            adi_score,
            50.0, // Default memory estimate
            affect_pleasure,
            affect_coherence.max(0.0).min(1.0),
            Some(format!("Stability log Ch.{} - injection:{}", entry.chapter, entry.injection_performed))
        )
    }

    /// Creates a pulse from RIC log entry data
    pub fn create_pulse_from_ric_entry(
        entry: &RICLogEntry,
        zc_tick: usize,
    ) -> Pulse {
        let drift_hits = entry.saturated_systems.len();
        let pathogens_detected = entry.intervention_count as usize;

        // RIC-based ADI score
        let adi_score = match entry.decision.as_str() {
            "CONTINUE" => 0.8,
            "CAUTION" => 0.5,
            "HALT" => 0.2,
            _ => 0.4,
        };

        // Affect based on RIC decision
        let affect_pleasure = match entry.decision.as_str() {
            "CONTINUE" => 0.3,
            "CAUTION" => 0.0,
            "HALT" => -0.5,
            _ => 0.0,
        };

        let affect_coherence = 1.0 - (drift_hits as f32 * 0.1);

        Pulse::with_data(
            zc_tick,
            pathogens_detected,
            drift_hits,
            adi_score,
            55.0, // Slightly higher memory estimate for RIC
            affect_pleasure,
            affect_coherence.max(0.0).min(1.0),
            Some(format!("RIC {} - Ch.{} - interventions:{}", entry.decision, entry.chapter, entry.intervention_count))
        )
    }

    /// Creates a pulse from RIP+RIC fusion data
    pub fn create_pulse_from_fusion_data(
        decision: &UnifiedArbitrationDecision,
        fusion_health: &RIPRICFusionHealth,
        chapter: u32,
        zc_tick: usize,
    ) -> Pulse {
        let drift_hits = fusion_health.cross_system_conflicts;
        let pathogens_detected = fusion_health.pathogen_detections;

        // Fusion-based ADI score
        let adi_score = fusion_health.overall_health_score * 0.8 +
                       fusion_health.python_rust_sync_score * 0.2;

        // Affect based on fusion health
        let affect_pleasure = (fusion_health.overall_health_score - 0.5) * 2.0; // -1 to 1 range
        let affect_coherence = fusion_health.python_rust_sync_score;

        Pulse::with_data(
            zc_tick,
            pathogens_detected,
            drift_hits,
            adi_score,
            60.0, // Higher memory estimate for fusion operations
            affect_pleasure,
            affect_coherence,
            Some(format!("RIP+RIC fusion Ch.{} - health:{:.2}", chapter, fusion_health.overall_health_score))
        )
    }

    /// Calculate ADI score from stability state
    fn calculate_stability_adi_score(state: &DriftStabilityState) -> f32 {
        let base_score = 0.6;

        // Factor in drift metrics
        let drift_penalty = state.theme_drift_score * 0.3;
        let decay_penalty = (state.emotional_decay_sum / 10.0).min(0.2);
        let stale_penalty = (state.stale_obligations as f32 * 0.05).min(0.2);

        (base_score - drift_penalty - decay_penalty - stale_penalty).max(0.0).min(1.0)
    }

    /// Log Qualitier quality level change
    pub fn log_quality_change(
        old_level: crate::adaptive::QualityLevel,
        new_level: crate::adaptive::QualityLevel,
        chapter: u32,
        memory_pressure: f32,
        narrative_stress: f32,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "event_type": "qualitier_quality_change",
            "chapter": chapter,
            "old_level": format!("{:?}", old_level),
            "new_level": format!("{:?}", new_level),
            "memory_pressure": memory_pressure,
            "narrative_stress": narrative_stress,
            "direction": if (new_level as u8) > (old_level as u8) { "upgrade" } else { "downgrade" }
        });

        // Log to console
        tracing::info!("Qualitier quality change: {:?} -> {:?} (Ch.{}, mem_pressure:{:.2}, narrative_stress:{:.2})",
            old_level, new_level, chapter, memory_pressure, narrative_stress);

        // Write to stability log file if enabled
        if let Err(e) = write_qualitier_log_entry(&log_entry) {
            tracing::warn!("Failed to write Qualitier log entry: {}", e);
        }
    }

    /// Log Qualitier performance degradation
    pub fn log_performance_degradation(
        reason: &str,
        current_level: crate::adaptive::QualityLevel,
        chapter: u32,
        memory_usage_mb: u64,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "event_type": "qualitier_performance_degradation",
            "chapter": chapter,
            "current_level": format!("{:?}", current_level),
            "reason": reason,
            "memory_usage_mb": memory_usage_mb
        });

        tracing::warn!("Qualitier performance degradation: {} (Ch.{}, level:{:?}, memory:{}MB)",
            reason, chapter, current_level, memory_usage_mb);

        if let Err(e) = write_qualitier_log_entry(&log_entry) {
            tracing::warn!("Failed to write Qualitier degradation log: {}", e);
        }
    }

    /// Log Qualitier narrative stress upgrade
    pub fn log_narrative_stress_upgrade(
        current_level: crate::adaptive::QualityLevel,
        chapter: u32,
        pathogen_count: usize,
        adi_score: f32,
        drift_hits: usize,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "event_type": "qualitier_narrative_stress_upgrade",
            "chapter": chapter,
            "current_level": format!("{:?}", current_level),
            "pathogen_count": pathogen_count,
            "adi_score": adi_score,
            "drift_hits": drift_hits
        });

        tracing::info!("Qualitier narrative stress upgrade: {:?} (Ch.{}, pathogens:{}, ADI:{:.2}, drift:{})",
            current_level, chapter, pathogen_count, adi_score, drift_hits);

        if let Err(e) = write_qualitier_log_entry(&log_entry) {
            tracing::warn!("Failed to write Qualitier stress upgrade log: {}", e);
        }
    }

    /// Create a pulse from Qualitier status data
    pub fn create_pulse_from_qualitier_status(
        status: &crate::adaptive::QualitierStatusReport,
        chapter: u32,
        zc_tick: usize,
    ) -> Pulse {
        // Convert quality level to numeric pathogen count
        let pathogens_detected = match status.current_level {
            crate::adaptive::QualityLevel::Minimal => 5,   // High pathogen inference
            crate::adaptive::QualityLevel::Standard => 2,  // Moderate
            crate::adaptive::QualityLevel::Enhanced => 1,  // Low
            crate::adaptive::QualityLevel::Premium => 0,   // Clean
        };

        // Drift hits based on memory degradations
        let drift_hits = if status.memory_degradations > 0 {
            (status.memory_degradations as usize).min(5)
        } else {
            0
        };

        // ADI score inverse to memory pressure
        let adi_score = (1.0 - status.memory_pressure).max(0.0).min(1.0);

        // Affect based on quality level
        let affect_pleasure = match status.current_level {
            crate::adaptive::QualityLevel::Minimal => -0.4,
            crate::adaptive::QualityLevel::Standard => 0.0,
            crate::adaptive::QualityLevel::Enhanced => 0.3,
            crate::adaptive::QualityLevel::Premium => 0.6,
        };

        // Coherence based on adaptive functioning
        let affect_coherence = if status.adaptive_enabled { 0.8 } else { 0.5 };

        Pulse::with_data(
            zc_tick,
            pathogens_detected,
            drift_hits,
            adi_score,
            (status.memory_pressure * 100.0), // Memory usage estimate
            affect_pleasure,
            affect_coherence,
            Some(format!("Qualitier {:?} Ch.{} - pressure:{:.2}",
                status.current_level, chapter, status.memory_pressure))
        )
    }

    /// Write Qualitier log entry to file
    fn write_qualitier_log_entry(entry: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let log_path = "logs/qualitier.json";

        // Ensure logs directory exists
        if let Some(parent) = Path::new(log_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        writeln!(file, "{}", entry)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recursive_drift_stabilizer::DriftStabilityState;
    use tempfile::TempDir;

    fn create_test_logger() -> (StabilityLogger, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let text_path = temp_dir.path().join("test_stability.log");
        let json_path = temp_dir.path().join("test_stability.json");
        let logger = StabilityLogger::with_paths(text_path, json_path);
        (logger, temp_dir)
    }

    #[test]
    fn test_stability_log_entry_creation() {
        let state = DriftStabilityState::new(5);
        let warnings = Some("Test warning".to_string());
        let entry = StabilityLogEntry::new(5, state, warnings, true);

        assert_eq!(entry.chapter, 5);
        assert!(entry.warnings.is_some());
        assert!(entry.injection_performed);
    }

    #[test]
    fn test_stability_log_entry_format_summary() {
        let mut state = DriftStabilityState::new(3);
        state.stale_obligations = 2;
        state.emotional_decay_sum = 1.5;
        state.theme_drift_score = 0.8;

        let entry = StabilityLogEntry::new(3, state, Some("Warning".to_string()), true);
        let summary = entry.format_summary();

        assert!(summary.contains("Ch.3"));
        assert!(summary.contains("⚠️ WARNINGS"));
        assert!(summary.contains("🔧 INJECTED"));
        assert!(summary.contains("Stale:2"));
        assert!(summary.contains("Decay:1.50"));
        assert!(summary.contains("Theme:0.80"));
    }

    #[test]
    fn test_stability_logger_creation() {
        let (logger, _temp_dir) = create_test_logger();
        assert!(logger.enabled);
        assert_eq!(logger.max_memory_entries, 100);
    }

    #[test]
    fn test_log_stability_update() {
        let (mut logger, _temp_dir) = create_test_logger();
        let state = DriftStabilityState::new(1);

        let result = logger.log_stability_update(
            1,
            &state,
            Some("Test warning".to_string()),
            false,
            None,
        );

        assert!(result.is_ok());
        assert_eq!(logger.recent_entries.len(), 1);
        assert!(logger.log_path.exists());
        assert!(logger.json_log_path.exists());
    }

    #[test]
    fn test_disabled_logger() {
        let (mut logger, _temp_dir) = create_test_logger();
        logger.set_enabled(false);

        let state = DriftStabilityState::new(1);
        let result = logger.log_stability_update(1, &state, None, false, None);

        assert!(result.is_ok());
        assert_eq!(logger.recent_entries.len(), 0);
        assert!(!logger.log_path.exists());
    }

    #[test]
    fn test_read_stability_history() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Log some entries
        let state1 = DriftStabilityState::new(1);
        let state2 = DriftStabilityState::new(2);

        logger.log_stability_update(1, &state1, None, false, None).unwrap();
        logger.log_stability_update(2, &state2, Some("Warning".to_string()), true, None).unwrap();

        // Read back
        let history = logger.read_stability_history().unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].chapter, 1);
        assert_eq!(history[1].chapter, 2);
        assert!(history[1].warnings.is_some());
    }

    #[test]
    fn test_get_recent_entries() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Add multiple entries
        for i in 1..6 {
            let state = DriftStabilityState::new(i);
            logger.log_stability_update(i, &state, None, false, None).unwrap();
        }

        let recent = logger.get_recent_entries(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].chapter, 3);
        assert_eq!(recent[2].chapter, 5);
    }

    #[test]
    fn test_memory_limit() {
        let (mut logger, _temp_dir) = create_test_logger();
        logger.max_memory_entries = 3;

        // Add more entries than the limit
        for i in 1..6 {
            let state = DriftStabilityState::new(i);
            logger.log_stability_update(i, &state, None, false, None).unwrap();
        }

        assert_eq!(logger.recent_entries.len(), 3);
        assert_eq!(logger.recent_entries[0].chapter, 3); // Oldest kept entry
        assert_eq!(logger.recent_entries[2].chapter, 5); // Newest entry
    }

    #[test]
    fn test_generate_trend_analysis() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Add entries with varying stability
        for i in 1..4 {
            let mut state = DriftStabilityState::new(i);
            state.stale_obligations = i as usize;
            state.emotional_decay_sum = i as f32 * 0.5;

            let warnings = if i > 2 { Some("Warning".to_string()) } else { None };
            let injection = i > 2;

            logger.log_stability_update(i, &state, warnings, injection, None).unwrap();
        }

        let analysis = logger.generate_trend_analysis(5).unwrap();

        assert!(analysis.contains("STABILITY TREND ANALYSIS"));
        assert!(analysis.contains("Total Entries: 3"));
        assert!(analysis.contains("Warning Rate:"));
        assert!(analysis.contains("Injection Rate:"));
        assert!(analysis.contains("Overall Stability:"));
    }

    #[test]
    fn test_generate_chapter_analysis() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Add entries for different chapters
        for i in 1..4 {
            let mut state = DriftStabilityState::new(i);
            state.stale_obligations = i as usize;

            let warnings = if i == 2 { Some("Chapter 2 warning".to_string()) } else { None };

            logger.log_stability_update(i, &state, warnings, false, None).unwrap();
        }

        let analysis = logger.generate_chapter_analysis(1, 3).unwrap();

        assert!(analysis.contains("CHAPTER-BY-CHAPTER ANALYSIS"));
        assert!(analysis.contains("Chapter Range: 1 - 3"));
        assert!(analysis.contains("Chapter 1:"));
        assert!(analysis.contains("Chapter 2:"));
        assert!(analysis.contains("Chapter 3:"));
        assert!(analysis.contains("Chapter 2 warning"));
    }

    #[test]
    fn test_clear_logs() {
        let (mut logger, _temp_dir) = create_test_logger();

        // Create some logs
        let state = DriftStabilityState::new(1);
        logger.log_stability_update(1, &state, None, false, None).unwrap();

        assert!(logger.log_path.exists());
        assert!(logger.json_log_path.exists());

        // Clear logs
        logger.clear_logs().unwrap();

        assert!(!logger.log_path.exists());
        assert!(!logger.json_log_path.exists());
    }

    #[test]
    fn test_with_context() {
        let state = DriftStabilityState::new(1);
        let context = serde_json::json!({
            "user_action": "generated_chapter",
            "word_count": 1500
        });

        let entry = StabilityLogEntry::new(1, state, None, false)
            .with_context(context.clone());

        assert_eq!(entry.context, context);
    }
}

/// ObliSelect Smart Obligation Management Telemetry Integration
pub mod obli_select_telemetry {
    use super::*;
    use crate::obligations::{ObligationScore, ObligationMetrics, ObliSelectSettings, ObligationCategory, ObligationUrgency};

    /// Logs obligation selection events with detailed scoring information
    pub fn log_obligation_selection(
        selected_obligations: &[ObligationScore],
        selection_performance_ms: u64,
        chapter: u32,
        context: &str,
    ) {
        let log_entry = format!(
            "[{}] Ch.{} ObliSelect Selection: {} obligations selected in {}ms | Context: {}",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            chapter,
            selected_obligations.len(),
            selection_performance_ms,
            context
        );

        tracing::info!("{}", log_entry);

        // Log detailed scoring if debug level is enabled
        if tracing::enabled!(tracing::Level::DEBUG) {
            for score in selected_obligations {
                tracing::debug!(
                    "  └─ {} (total:{:.3}) urgency:{:.3} salience:{:.3} fresh:{:.3} tension:{:.3} dep:{:.3} context:{:.3} | {}",
                    score.obligation_id,
                    score.total_score,
                    score.urgency_score,
                    score.salience_score,
                    score.freshness_score,
                    score.tension_balance_score,
                    score.dependency_score,
                    score.context_relevance_score,
                    score.justification
                );
            }
        }

        // Write to JSON log for structured analysis
        let json_entry = serde_json::json!({
            "timestamp": Utc::now(),
            "event_type": "obligation_selection",
            "chapter": chapter,
            "context": context,
            "selection_count": selected_obligations.len(),
            "performance_ms": selection_performance_ms,
            "selected_obligations": selected_obligations.iter().map(|score| {
                serde_json::json!({
                    "obligation_id": score.obligation_id,
                    "total_score": score.total_score,
                    "component_scores": {
                        "urgency": score.urgency_score,
                        "salience": score.salience_score,
                        "freshness": score.freshness_score,
                        "tension_balance": score.tension_balance_score,
                        "dependency": score.dependency_score,
                        "context_relevance": score.context_relevance_score
                    },
                    "justification": score.justification
                })
            }).collect::<Vec<_>>()
        });

        if let Err(e) = write_json_log_entry("logs/obli_select.json", &json_entry) {
            tracing::warn!("Failed to write ObliSelect JSON log: {}", e);
        }
    }

    /// Logs obligation management metrics and health status
    pub fn log_obligation_metrics(
        metrics: &ObligationMetrics,
        settings: &ObliSelectSettings,
        chapter: u32,
    ) {
        let log_entry = format!(
            "[{}] Ch.{} ObliSelect Metrics: {} total, {} stale, {} overused | Avg injection: {:.1}, Fulfillment: {:.1}%",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            chapter,
            metrics.total_obligations,
            metrics.stale_obligations,
            metrics.overused_obligations,
            metrics.average_injection_count,
            metrics.fulfillment_progress_average * 100.0
        );

        tracing::info!("{}", log_entry);

        // Log tension distribution
        tracing::debug!(
            "  Tension Distribution: {:.1}% negative, {:.1}% neutral, {:.1}% positive",
            metrics.tension_distribution.0 * 100.0,
            metrics.tension_distribution.1 * 100.0,
            metrics.tension_distribution.2 * 100.0
        );

        // Log category and urgency distributions
        if tracing::enabled!(tracing::Level::DEBUG) {
            tracing::debug!("  Category Distribution:");
            for (category, count) in &metrics.obligations_by_category {
                tracing::debug!("    {:?}: {}", category, count);
            }

            tracing::debug!("  Urgency Distribution:");
            for (urgency, count) in &metrics.obligations_by_urgency {
                tracing::debug!("    {:?}: {}", urgency, count);
            }
        }

        // Write comprehensive metrics to JSON
        let json_entry = serde_json::json!({
            "timestamp": Utc::now(),
            "event_type": "obligation_metrics",
            "chapter": chapter,
            "metrics": {
                "total_obligations": metrics.total_obligations,
                "stale_obligations": metrics.stale_obligations,
                "overused_obligations": metrics.overused_obligations,
                "average_injection_count": metrics.average_injection_count,
                "fulfillment_progress_average": metrics.fulfillment_progress_average,
                "tension_distribution": {
                    "negative_percent": metrics.tension_distribution.0,
                    "neutral_percent": metrics.tension_distribution.1,
                    "positive_percent": metrics.tension_distribution.2
                },
                "dependency_chain_length_max": metrics.dependency_chain_length_max,
                "last_selection_performance_ms": metrics.last_selection_performance_ms,
                "obligations_by_category": metrics.obligations_by_category,
                "obligations_by_urgency": metrics.obligations_by_urgency
            },
            "settings": {
                "max_obligations_per_selection": settings.max_obligations_per_selection,
                "weights": {
                    "urgency": settings.urgency_weight,
                    "salience": settings.salience_weight,
                    "freshness": settings.freshness_weight,
                    "tension_balance": settings.tension_balance_weight,
                    "dependency": settings.dependency_weight,
                    "context_relevance": settings.context_relevance_weight
                },
                "thresholds": {
                    "staleness_penalty": settings.staleness_penalty_threshold,
                    "overuse_penalty": settings.overuse_penalty_threshold,
                    "tension_balance_target": settings.tension_balance_target
                },
                "features": {
                    "adaptive_weighting": settings.enable_adaptive_weighting,
                    "dependency_resolution": settings.enable_dependency_resolution,
                    "contextual_filtering": settings.enable_contextual_filtering
                }
            }
        });

        if let Err(e) = write_json_log_entry("logs/obli_select_metrics.json", &json_entry) {
            tracing::warn!("Failed to write ObliSelect metrics JSON log: {}", e);
        }
    }

    /// Logs obligation lifecycle events (add, remove, fulfill)
    pub fn log_obligation_lifecycle_event(
        event_type: &str,
        obligation_id: &str,
        obligation_content: Option<&str>,
        category: Option<ObligationCategory>,
        urgency: Option<ObligationUrgency>,
        fulfillment_progress: Option<f32>,
        chapter: u32,
    ) {
        let log_entry = format!(
            "[{}] Ch.{} ObliSelect {}: {} | Progress: {:?} | Category: {:?} | Urgency: {:?}",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            chapter,
            event_type,
            obligation_id,
            fulfillment_progress.map(|p| format!("{:.1}%", p * 100.0)),
            category,
            urgency
        );

        match event_type {
            "ADD" => tracing::info!("{}", log_entry),
            "REMOVE" | "FULFILL" => tracing::info!("{}", log_entry),
            "UPDATE" => tracing::debug!("{}", log_entry),
            _ => tracing::debug!("{}", log_entry),
        }

        // Write lifecycle event to JSON log
        let json_entry = serde_json::json!({
            "timestamp": Utc::now(),
            "event_type": "obligation_lifecycle",
            "lifecycle_event": event_type,
            "chapter": chapter,
            "obligation_id": obligation_id,
            "obligation_content": obligation_content,
            "category": category,
            "urgency": urgency,
            "fulfillment_progress": fulfillment_progress
        });

        if let Err(e) = write_json_log_entry("logs/obli_select_lifecycle.json", &json_entry) {
            tracing::warn!("Failed to write ObliSelect lifecycle JSON log: {}", e);
        }
    }

    /// Logs performance warnings for ObliSelect operations
    pub fn log_performance_warning(
        operation: &str,
        actual_time_ms: u64,
        threshold_ms: u64,
        chapter: u32,
        obligation_count: usize,
    ) {
        let log_entry = format!(
            "[{}] Ch.{} ⚠️ ObliSelect Performance Warning: {} took {}ms (threshold: {}ms) | {} obligations",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            chapter,
            operation,
            actual_time_ms,
            threshold_ms,
            obligation_count
        );

        tracing::warn!("{}", log_entry);

        // Write performance warning to JSON log
        let json_entry = serde_json::json!({
            "timestamp": Utc::now(),
            "event_type": "obligation_performance_warning",
            "chapter": chapter,
            "operation": operation,
            "actual_time_ms": actual_time_ms,
            "threshold_ms": threshold_ms,
            "obligation_count": obligation_count,
            "performance_ratio": actual_time_ms as f64 / threshold_ms as f64
        });

        if let Err(e) = write_json_log_entry("logs/obli_select_performance.json", &json_entry) {
            tracing::warn!("Failed to write ObliSelect performance JSON log: {}", e);
        }
    }

    /// Helper function to write JSON log entries
    fn write_json_log_entry(log_path: &str, entry: &serde_json::Value) -> std::io::Result<()> {
        // Ensure logs directory exists
        if let Some(parent) = Path::new(log_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        writeln!(file, "{}", entry)?;
        Ok(())
    }

    /// Gets summary statistics for obligation management logging
    pub fn get_logging_summary(chapter: u32) -> serde_json::Value {
        serde_json::json!({
            "timestamp": Utc::now(),
            "chapter": chapter,
            "logging_status": {
                "obli_select_enabled": true,
                "log_files": [
                    "logs/obli_select.json",
                    "logs/obli_select_metrics.json",
                    "logs/obli_select_lifecycle.json",
                    "logs/obli_select_performance.json"
                ],
                "supported_events": [
                    "obligation_selection",
                    "obligation_metrics",
                    "obligation_lifecycle",
                    "obligation_performance_warning"
                ]
            }
        })
    }
}