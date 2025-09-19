/// 🔬 PHASE 5: Prompt Audit Logging (Explainable Shimmy)
///
/// Writes out what constraints were injected and why.
/// This module provides comprehensive logging and auditing capabilities
/// for all prompt modifications and constraint injections in the SHIMMY-DS system.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Timestamp of the audit entry
    pub timestamp: DateTime<Utc>,
    /// Type of injection performed
    pub injection_type: String,
    /// The specific constraint or obligation that was injected
    pub injected_content: String,
    /// Reason for the injection
    pub reason: String,
    /// Optional chapter or scene number
    pub chapter: Option<u32>,
    /// Original prompt before modification
    pub original_prompt: String,
    /// Final prompt after all modifications
    pub modified_prompt: String,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

impl AuditEntry {
    /// Creates a new audit entry with the specified parameters
    pub fn new(
        injection_type: impl Into<String>,
        injected_content: impl Into<String>,
        reason: impl Into<String>,
        original_prompt: impl Into<String>,
        modified_prompt: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            injection_type: injection_type.into(),
            injected_content: injected_content.into(),
            reason: reason.into(),
            chapter: None,
            original_prompt: original_prompt.into(),
            modified_prompt: modified_prompt.into(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Sets the chapter number for this audit entry
    pub fn with_chapter(mut self, chapter: u32) -> Self {
        self.chapter = Some(chapter);
        self
    }

    /// Adds metadata to the audit entry
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    /// Formats the entry as a human-readable log line
    pub fn format_text(&self) -> String {
        let chapter_info = self.chapter
            .map(|c| format!(" (Chapter {})", c))
            .unwrap_or_default();

        format!(
            "[{}]{}\nInjected: \"{}\"\nReason: {}\nOriginal: \"{}\"\nModified: \"{}\"\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            chapter_info,
            self.injected_content,
            self.reason,
            self.original_prompt.chars().take(100).collect::<String>(),
            self.modified_prompt.chars().take(100).collect::<String>()
        )
    }
}

/// Main audit logger for SHIMMY-DS prompt modifications
pub struct PromptAuditor {
    /// Path to the text log file
    text_log_path: PathBuf,
    /// Path to the JSON log file
    json_log_path: PathBuf,
    /// Whether logging is enabled
    enabled: bool,
}

impl PromptAuditor {
    /// Creates a new prompt auditor with default log paths
    pub fn new() -> Self {
        Self {
            text_log_path: PathBuf::from("logs/prompt_audit.log"),
            json_log_path: PathBuf::from("logs/prompt_audit.json"),
            enabled: true,
        }
    }

    /// Creates a new prompt auditor with custom log paths
    pub fn with_paths(text_path: impl AsRef<Path>, json_path: impl AsRef<Path>) -> Self {
        Self {
            text_log_path: text_path.as_ref().to_path_buf(),
            json_log_path: json_path.as_ref().to_path_buf(),
            enabled: true,
        }
    }

    /// Enables or disables logging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Logs an obligation injection
    pub fn log_obligation_injection(
        &self,
        obligations: &[String],
        original_prompt: &str,
        modified_prompt: &str,
        chapter: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        for obligation in obligations {
            let entry = AuditEntry::new(
                "obligation_injection",
                obligation,
                "Spatial continuity from persistent state",
                original_prompt,
                modified_prompt,
            )
            .with_chapter(chapter.unwrap_or(0));

            self.write_entry(&entry)?;
        }

        Ok(())
    }

    /// Logs an emotional field injection
    pub fn log_emotion_injection(
        &self,
        emotion: &str,
        intensity: f32,
        original_prompt: &str,
        modified_prompt: &str,
        reason: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let injected_content = format!("{} (intensity: {:.2})", emotion, intensity);
        let entry = AuditEntry::new(
            "emotion_injection",
            injected_content,
            reason,
            original_prompt,
            modified_prompt,
        );

        self.write_entry(&entry)
    }

    /// Logs a spatial continuity validation
    pub fn log_spatial_validation(
        &self,
        last_location: &str,
        generated_text: &str,
        is_valid: bool,
        validation_details: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let validation_status = if is_valid { "VALID" } else { "INVALID" };
        let entry = AuditEntry::new(
            "spatial_validation",
            format!("Location: {} -> {}", last_location, validation_status),
            validation_details,
            "", // No original prompt for validation
            generated_text,
        );

        self.write_entry(&entry)
    }

    /// Logs obligation pressure analysis
    pub fn log_pressure_analysis(
        &self,
        pressure_level: f32,
        recommendation: &str,
        obligations_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let metadata = serde_json::json!({
            "pressure_level": pressure_level,
            "obligations_count": obligations_count,
            "threshold_status": if pressure_level > 1.5 { "HIGH" } else { "NORMAL" }
        });

        let entry = AuditEntry::new(
            "pressure_analysis",
            format!("Pressure: {:.2}", pressure_level),
            recommendation,
            "",
            "",
        )
        .with_metadata(metadata);

        self.write_entry(&entry)
    }

    /// Logs a general audit event
    pub fn log_event(
        &self,
        event_type: &str,
        content: &str,
        reason: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let mut entry = AuditEntry::new(event_type, content, reason, "", "");
        if let Some(meta) = metadata {
            entry = entry.with_metadata(meta);
        }

        self.write_entry(&entry)
    }

    /// Writes an audit entry to both text and JSON logs
    fn write_entry(&self, entry: &AuditEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directories exist
        if let Some(parent) = self.text_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if let Some(parent) = self.json_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write to text log
        let mut text_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.text_log_path)?;
        writeln!(text_file, "{}", entry.format_text())?;

        // Write to JSON log
        let mut json_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.json_log_path)?;
        writeln!(json_file, "{}", serde_json::to_string(entry)?)?;

        Ok(())
    }

    /// Reads all audit entries from the JSON log
    pub fn read_audit_history(&self) -> Result<Vec<AuditEntry>, Box<dyn std::error::Error>> {
        if !self.json_log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.json_log_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if !line.trim().is_empty() {
                let entry: AuditEntry = serde_json::from_str(line)?;
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Generates a summary report of audit activities
    pub fn generate_summary_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let entries = self.read_audit_history()?;

        if entries.is_empty() {
            return Ok("No audit entries found.".to_string());
        }

        let mut report = String::new();
        report.push_str("🔬 SHIMMY-DS AUDIT SUMMARY REPORT\n");
        report.push_str("==================================\n\n");

        // Count by injection type
        let mut type_counts = std::collections::HashMap::new();
        let mut recent_entries = Vec::new();

        for entry in &entries {
            *type_counts.entry(entry.injection_type.clone()).or_insert(0) += 1;

            // Keep last 10 entries
            if recent_entries.len() < 10 {
                recent_entries.push(entry);
            }
        }

        report.push_str(&format!("Total Audit Entries: {}\n", entries.len()));
        report.push_str(&format!("First Entry: {}\n", entries.first().unwrap().timestamp.format("%Y-%m-%d %H:%M:%S")));
        report.push_str(&format!("Last Entry: {}\n\n", entries.last().unwrap().timestamp.format("%Y-%m-%d %H:%M:%S")));

        report.push_str("Injection Types:\n");
        for (injection_type, count) in type_counts {
            report.push_str(&format!("  • {}: {} times\n", injection_type, count));
        }

        report.push_str("\nRecent Entries:\n");
        for entry in recent_entries.iter().rev().take(5) {
            report.push_str(&format!("  • [{}] {}: {}\n",
                entry.timestamp.format("%m-%d %H:%M"),
                entry.injection_type,
                entry.injected_content.chars().take(50).collect::<String>()
            ));
        }

        Ok(report)
    }

    /// Clears all audit logs (use with caution)
    pub fn clear_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.text_log_path.exists() {
            std::fs::remove_file(&self.text_log_path)?;
        }
        if self.json_log_path.exists() {
            std::fs::remove_file(&self.json_log_path)?;
        }
        Ok(())
    }
}

impl Default for PromptAuditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global auditor instance (use carefully in multi-threaded contexts)
static mut GLOBAL_AUDITOR: Option<PromptAuditor> = None;
static AUDITOR_INIT: std::sync::Once = std::sync::Once::new();

/// Gets the global auditor instance (initializes if needed)
pub fn global_auditor() -> &'static PromptAuditor {
    unsafe {
        AUDITOR_INIT.call_once(|| {
            GLOBAL_AUDITOR = Some(PromptAuditor::new());
        });
        GLOBAL_AUDITOR.as_ref().unwrap()
    }
}

/// Convenience function to log obligation injection using global auditor
pub fn log_obligation_injection(
    obligations: &[String],
    original_prompt: &str,
    modified_prompt: &str,
    chapter: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    global_auditor().log_obligation_injection(obligations, original_prompt, modified_prompt, chapter)
}

/// Convenience function to log emotion injection using global auditor
pub fn log_emotion_injection(
    emotion: &str,
    intensity: f32,
    original_prompt: &str,
    modified_prompt: &str,
    reason: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    global_auditor().log_emotion_injection(emotion, intensity, original_prompt, modified_prompt, reason)
}

/// Convenience function to log spatial validation using global auditor
pub fn log_spatial_validation(
    last_location: &str,
    generated_text: &str,
    is_valid: bool,
    validation_details: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    global_auditor().log_spatial_validation(last_location, generated_text, is_valid, validation_details)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_auditor() -> (PromptAuditor, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let text_path = temp_dir.path().join("test_audit.log");
        let json_path = temp_dir.path().join("test_audit.json");
        let auditor = PromptAuditor::with_paths(text_path, json_path);
        (auditor, temp_dir)
    }

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(
            "test_injection",
            "test content",
            "test reason",
            "original",
            "modified",
        );

        assert_eq!(entry.injection_type, "test_injection");
        assert_eq!(entry.injected_content, "test content");
        assert_eq!(entry.reason, "test reason");
        assert_eq!(entry.original_prompt, "original");
        assert_eq!(entry.modified_prompt, "modified");
    }

    #[test]
    fn test_audit_entry_with_chapter() {
        let entry = AuditEntry::new("test", "content", "reason", "orig", "mod")
            .with_chapter(5);

        assert_eq!(entry.chapter, Some(5));
    }

    #[test]
    fn test_audit_entry_format_text() {
        let entry = AuditEntry::new("test", "content", "reason", "original", "modified")
            .with_chapter(3);

        let formatted = entry.format_text();
        assert!(formatted.contains("(Chapter 3)"));
        assert!(formatted.contains("Injected: \"content\""));
        assert!(formatted.contains("Reason: reason"));
    }

    #[test]
    fn test_auditor_creation() {
        let (auditor, _temp_dir) = create_test_auditor();
        assert!(auditor.enabled);
    }

    #[test]
    fn test_auditor_enable_disable() {
        let (mut auditor, _temp_dir) = create_test_auditor();

        auditor.set_enabled(false);
        assert!(!auditor.enabled);

        auditor.set_enabled(true);
        assert!(auditor.enabled);
    }

    #[test]
    fn test_log_obligation_injection() {
        let (auditor, _temp_dir) = create_test_auditor();

        let obligations = vec!["Test obligation".to_string()];
        let result = auditor.log_obligation_injection(
            &obligations,
            "original prompt",
            "modified prompt",
            Some(1),
        );

        assert!(result.is_ok());

        // Verify logs were written
        assert!(auditor.text_log_path.exists());
        assert!(auditor.json_log_path.exists());
    }

    #[test]
    fn test_log_emotion_injection() {
        let (auditor, _temp_dir) = create_test_auditor();

        let result = auditor.log_emotion_injection(
            "guilt",
            0.8,
            "original",
            "modified",
            "narrative continuity",
        );

        assert!(result.is_ok());
        assert!(auditor.text_log_path.exists());
        assert!(auditor.json_log_path.exists());
    }

    #[test]
    fn test_log_spatial_validation() {
        let (auditor, _temp_dir) = create_test_auditor();

        let result = auditor.log_spatial_validation(
            "attic",
            "Harper left the attic",
            true,
            "Valid transition detected",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_log_pressure_analysis() {
        let (auditor, _temp_dir) = create_test_auditor();

        let result = auditor.log_pressure_analysis(
            2.1,
            "High pressure - recommend resolution",
            3,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_read_audit_history() {
        let (auditor, _temp_dir) = create_test_auditor();

        // Write some entries
        let obligations = vec!["Test".to_string()];
        auditor.log_obligation_injection(&obligations, "orig", "mod", Some(1)).unwrap();
        auditor.log_emotion_injection("joy", 0.7, "orig", "mod", "test").unwrap();

        // Read back
        let history = auditor.read_audit_history().unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].injection_type, "obligation_injection");
        assert_eq!(history[1].injection_type, "emotion_injection");
    }

    #[test]
    fn test_generate_summary_report() {
        let (auditor, _temp_dir) = create_test_auditor();

        // Write some entries
        let obligations = vec!["Test".to_string()];
        auditor.log_obligation_injection(&obligations, "orig", "mod", Some(1)).unwrap();
        auditor.log_emotion_injection("joy", 0.7, "orig", "mod", "test").unwrap();

        let report = auditor.generate_summary_report().unwrap();
        assert!(report.contains("SHIMMY-DS AUDIT SUMMARY REPORT"));
        assert!(report.contains("Total Audit Entries: 2"));
        assert!(report.contains("obligation_injection"));
        assert!(report.contains("emotion_injection"));
    }

    #[test]
    fn test_disabled_auditor() {
        let (mut auditor, _temp_dir) = create_test_auditor();
        auditor.set_enabled(false);

        let obligations = vec!["Test".to_string()];
        let result = auditor.log_obligation_injection(&obligations, "orig", "mod", Some(1));

        assert!(result.is_ok());
        // Should not create log files when disabled
        assert!(!auditor.text_log_path.exists());
        assert!(!auditor.json_log_path.exists());
    }

    #[test]
    fn test_clear_logs() {
        let (auditor, _temp_dir) = create_test_auditor();

        // Create some logs
        let obligations = vec!["Test".to_string()];
        auditor.log_obligation_injection(&obligations, "orig", "mod", Some(1)).unwrap();

        assert!(auditor.text_log_path.exists());
        assert!(auditor.json_log_path.exists());

        // Clear logs
        auditor.clear_logs().unwrap();

        assert!(!auditor.text_log_path.exists());
        assert!(!auditor.json_log_path.exists());
    }

    #[test]
    fn test_empty_history() {
        let (auditor, _temp_dir) = create_test_auditor();
        let history = auditor.read_audit_history().unwrap();
        assert!(history.is_empty());

        let report = auditor.generate_summary_report().unwrap();
        assert_eq!(report, "No audit entries found.");
    }
}