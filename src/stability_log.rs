/// 📊 Stability Log - Advanced Reporting for Recursive Drift Stabilizer
///
/// This module provides specialized logging and reporting capabilities
/// for the Recursive Drift Stabilizer system.

use crate::recursive_drift_stabilizer::{DriftStabilityState, DriftStabilizerConfig};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

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

/// Manages stability logging for the drift stabilizer
pub struct StabilityLogger {
    /// Path to the stability log file
    log_path: PathBuf,
    /// Path to the JSON stability log
    json_log_path: PathBuf,
    /// Whether logging is enabled
    enabled: bool,
    /// Maximum number of entries to keep in memory
    max_memory_entries: usize,
    /// In-memory cache of recent entries
    recent_entries: Vec<StabilityLogEntry>,
}

impl StabilityLogger {
    /// Creates a new stability logger with default paths
    pub fn new() -> Self {
        Self {
            log_path: PathBuf::from("logs/stability.log"),
            json_log_path: PathBuf::from("logs/stability.json"),
            enabled: true,
            max_memory_entries: 100,
            recent_entries: Vec::new(),
        }
    }

    /// Creates a new stability logger with custom paths
    pub fn with_paths(text_path: impl AsRef<Path>, json_path: impl AsRef<Path>) -> Self {
        Self {
            log_path: text_path.as_ref().to_path_buf(),
            json_log_path: json_path.as_ref().to_path_buf(),
            enabled: true,
            max_memory_entries: 100,
            recent_entries: Vec::new(),
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
            let entries = chapter_groups.get(chapter).unwrap();
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