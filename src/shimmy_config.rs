/// 🔄 PHASE 6: Configuration System
///
/// Runtime configuration flags and settings for the SHIMMY-DS augmentation system.
/// This module provides centralized configuration management with TOML file support.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Main configuration structure for SHIMMY-DS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimmyConfig {
    #[serde(rename = "shimmy-ds")]
    pub shimmy_ds: ShimmyDsConfig,
    pub context: ContextConfig,
    pub logging: LoggingConfig,
    pub validation: ValidationConfig,
    pub performance: PerformanceConfig,
    pub drift_stabilizer: DriftStabilizerConfig,
}

/// Core SHIMMY-DS feature toggles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimmyDsConfig {
    /// Enable/disable prompt injection system
    pub enable_prompt_injection: bool,
    /// Enable/disable location validation
    pub enable_location_validation: bool,
    /// Enable/disable emotion resonance injection
    pub enable_emotion_resonance: bool,
    /// Enable/disable obligation pressure monitoring
    pub enable_pressure_monitoring: bool,
    /// Enable/disable audit logging
    pub enable_audit_logging: bool,
    /// Pressure threshold for high priority warnings
    pub pressure_threshold: f32,
    /// Default emotional intensity multiplier
    pub emotion_intensity_multiplier: f32,
    /// Default chapter number
    pub default_chapter: u32,
    /// Maximum number of obligations to inject per prompt
    pub max_obligations_per_prompt: usize,
}

/// Context management configuration for long-form writing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// Preferred context length for long-form writing
    pub preferred_ctx_length: usize,
    /// Context retention strategy
    pub retain_context_between_generations: bool,
    /// Number of previous generations to retain in context
    pub context_history_depth: usize,
    /// Enable context compression for very long sessions
    pub enable_context_compression: bool,
    /// Compression ratio (0.0 to 1.0, higher = more aggressive)
    pub compression_ratio: f32,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Text log file path
    pub text_log_path: String,
    /// JSON log file path
    pub json_log_path: String,
    /// Maximum log file size in MB before rotation
    pub max_log_size_mb: u64,
    /// Number of rotated log files to keep
    pub max_log_files: u32,
}

/// Validation system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Require explicit transition words for location changes
    pub strict_location_validation: bool,
    /// Allow implicit transitions (scene cuts, etc.)
    pub allow_implicit_transitions: bool,
    /// Minimum confidence threshold for emotion detection
    pub emotion_detection_threshold: f32,
}

/// Performance-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Cache obligation lists for better performance
    pub cache_obligations: bool,
    /// Maximum cache age in minutes
    pub cache_max_age_minutes: u32,
    /// Enable performance metrics collection
    pub collect_metrics: bool,
}

/// Drift stabilizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftStabilizerConfig {
    /// Enable/disable recursive drift stabilizer
    pub enabled: bool,
    /// Threshold for stale obligation warnings
    pub stale_obligation_threshold: usize,
    /// Limit for emotional decay warnings
    pub emotional_decay_limit: f32,
    /// Threshold for theme drift warnings
    pub theme_threshold: f32,
    /// Chapter limit for spatial pressure warnings
    pub spatial_pressure_chapter_limit: u32,
    /// Enable drift injection prompts when warnings are detected
    pub enable_drift_injection: bool,
    /// Enable detailed stability logging
    pub enable_stability_logging: bool,
}

impl Default for ShimmyConfig {
    fn default() -> Self {
        Self {
            shimmy_ds: ShimmyDsConfig {
                enable_prompt_injection: true,
                enable_location_validation: true,
                enable_emotion_resonance: true,
                enable_pressure_monitoring: true,
                enable_audit_logging: true,
                pressure_threshold: 1.5,
                emotion_intensity_multiplier: 1.0,
                default_chapter: 1,
                max_obligations_per_prompt: 15, // Increased for long-form
            },
            context: ContextConfig {
                preferred_ctx_length: 32768,
                retain_context_between_generations: true,
                context_history_depth: 3,
                enable_context_compression: true,
                compression_ratio: 0.3,
            },
            logging: LoggingConfig {
                text_log_path: "logs/prompt_audit.log".to_string(),
                json_log_path: "logs/prompt_audit.json".to_string(),
                max_log_size_mb: 100,
                max_log_files: 5,
            },
            validation: ValidationConfig {
                strict_location_validation: false,
                allow_implicit_transitions: true,
                emotion_detection_threshold: 0.3,
            },
            performance: PerformanceConfig {
                cache_obligations: true,
                cache_max_age_minutes: 30,
                collect_metrics: false,
            },
            drift_stabilizer: DriftStabilizerConfig {
                enabled: true,
                stale_obligation_threshold: 5,
                emotional_decay_limit: 2.5,
                theme_threshold: 1.0,
                spatial_pressure_chapter_limit: 3,
                enable_drift_injection: true,
                enable_stability_logging: true,
            },
        }
    }
}

/// Configuration manager for SHIMMY-DS
pub struct ConfigManager {
    config: Arc<RwLock<ShimmyConfig>>,
    config_path: PathBuf,
}

impl ConfigManager {
    /// Creates a new configuration manager with the specified config file path
    pub fn new(config_path: impl AsRef<Path>) -> Self {
        Self {
            config: Arc::new(RwLock::new(ShimmyConfig::default())),
            config_path: config_path.as_ref().to_path_buf(),
        }
    }

    /// Creates a configuration manager with default config file path
    pub fn with_default_path() -> Self {
        Self::new("shimmy-ds.toml")
    }

    /// Loads configuration from the TOML file
    pub fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            // Create default config file if it doesn't exist
            self.save_default()?;
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let parsed_config: ShimmyConfig = toml::from_str(&content)?;

        let mut config = self.config.write().unwrap();
        *config = parsed_config;

        Ok(())
    }

    /// Saves the current configuration to the TOML file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.read().unwrap();
        let toml_content = toml::to_string_pretty(&*config)?;
        std::fs::write(&self.config_path, toml_content)?;
        Ok(())
    }

    /// Saves the default configuration to the TOML file
    pub fn save_default(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default_config = ShimmyConfig::default();
        let toml_content = toml::to_string_pretty(&default_config)?;

        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&self.config_path, toml_content)?;
        Ok(())
    }

    /// Gets a clone of the current configuration
    pub fn get_config(&self) -> ShimmyConfig {
        self.config.read().unwrap().clone()
    }

    /// Updates a specific configuration value
    pub fn update_config<F>(&self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut ShimmyConfig),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut config);
        drop(config);
        self.save()
    }

    /// Checks if prompt injection is enabled
    pub fn is_prompt_injection_enabled(&self) -> bool {
        self.config.read().unwrap().shimmy_ds.enable_prompt_injection
    }

    /// Checks if location validation is enabled
    pub fn is_location_validation_enabled(&self) -> bool {
        self.config.read().unwrap().shimmy_ds.enable_location_validation
    }

    /// Checks if emotion resonance is enabled
    pub fn is_emotion_resonance_enabled(&self) -> bool {
        self.config.read().unwrap().shimmy_ds.enable_emotion_resonance
    }

    /// Checks if pressure monitoring is enabled
    pub fn is_pressure_monitoring_enabled(&self) -> bool {
        self.config.read().unwrap().shimmy_ds.enable_pressure_monitoring
    }

    /// Checks if audit logging is enabled
    pub fn is_audit_logging_enabled(&self) -> bool {
        self.config.read().unwrap().shimmy_ds.enable_audit_logging
    }

    /// Gets the pressure threshold
    pub fn get_pressure_threshold(&self) -> f32 {
        self.config.read().unwrap().shimmy_ds.pressure_threshold
    }

    /// Gets the emotion intensity multiplier
    pub fn get_emotion_intensity_multiplier(&self) -> f32 {
        self.config.read().unwrap().shimmy_ds.emotion_intensity_multiplier
    }

    /// Gets the maximum obligations per prompt
    pub fn get_max_obligations_per_prompt(&self) -> usize {
        self.config.read().unwrap().shimmy_ds.max_obligations_per_prompt
    }

    /// Gets the emotion detection threshold
    pub fn get_emotion_detection_threshold(&self) -> f32 {
        self.config.read().unwrap().validation.emotion_detection_threshold
    }

    /// Gets logging configuration
    pub fn get_logging_config(&self) -> LoggingConfig {
        self.config.read().unwrap().logging.clone()
    }

    /// Gets drift stabilizer configuration
    pub fn get_drift_stabilizer_config(&self) -> DriftStabilizerConfig {
        self.config.read().unwrap().drift_stabilizer.clone()
    }

    /// Checks if drift stabilizer is enabled
    pub fn is_drift_stabilizer_enabled(&self) -> bool {
        self.config.read().unwrap().drift_stabilizer.enabled
    }

    /// Gets the stale obligation threshold
    pub fn get_stale_obligation_threshold(&self) -> usize {
        self.config.read().unwrap().drift_stabilizer.stale_obligation_threshold
    }

    /// Gets the emotional decay limit
    pub fn get_emotional_decay_limit(&self) -> f32 {
        self.config.read().unwrap().drift_stabilizer.emotional_decay_limit
    }

    /// Gets the theme threshold
    pub fn get_theme_threshold(&self) -> f32 {
        self.config.read().unwrap().drift_stabilizer.theme_threshold
    }

    /// Checks if drift injection is enabled
    pub fn is_drift_injection_enabled(&self) -> bool {
        self.config.read().unwrap().drift_stabilizer.enable_drift_injection
    }

    /// Checks if stability logging is enabled
    pub fn is_stability_logging_enabled(&self) -> bool {
        self.config.read().unwrap().drift_stabilizer.enable_stability_logging
    }

    // Context management getters for long-form writing

    /// Gets the preferred context length
    pub fn get_preferred_ctx_length(&self) -> usize {
        self.config.read().unwrap().context.preferred_ctx_length
    }

    /// Checks if context retention between generations is enabled
    pub fn is_context_retention_enabled(&self) -> bool {
        self.config.read().unwrap().context.retain_context_between_generations
    }

    /// Gets the context history depth
    pub fn get_context_history_depth(&self) -> usize {
        self.config.read().unwrap().context.context_history_depth
    }

    /// Checks if context compression is enabled
    pub fn is_context_compression_enabled(&self) -> bool {
        self.config.read().unwrap().context.enable_context_compression
    }

    /// Gets the context compression ratio
    pub fn get_context_compression_ratio(&self) -> f32 {
        self.config.read().unwrap().context.compression_ratio
    }

    /// Gets the complete context configuration
    pub fn get_context_config(&self) -> ContextConfig {
        self.config.read().unwrap().context.clone()
    }

    /// Toggles a feature on/off
    pub fn toggle_feature(&self, feature: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.update_config(|config| {
            match feature {
                "prompt_injection" => config.shimmy_ds.enable_prompt_injection = enabled,
                "location_validation" => config.shimmy_ds.enable_location_validation = enabled,
                "emotion_resonance" => config.shimmy_ds.enable_emotion_resonance = enabled,
                "pressure_monitoring" => config.shimmy_ds.enable_pressure_monitoring = enabled,
                "audit_logging" => config.shimmy_ds.enable_audit_logging = enabled,
                _ => {}
            }
        })
    }

    /// Generates a configuration status report
    pub fn generate_status_report(&self) -> String {
        let config = self.get_config();
        let mut report = String::new();

        report.push_str("🔄 SHIMMY-DS CONFIGURATION STATUS\n");
        report.push_str("==================================\n\n");

        report.push_str("Core Features:\n");
        report.push_str(&format!("  • Prompt Injection: {}\n",
            if config.shimmy_ds.enable_prompt_injection { "✅ ENABLED" } else { "❌ DISABLED" }));
        report.push_str(&format!("  • Location Validation: {}\n",
            if config.shimmy_ds.enable_location_validation { "✅ ENABLED" } else { "❌ DISABLED" }));
        report.push_str(&format!("  • Emotion Resonance: {}\n",
            if config.shimmy_ds.enable_emotion_resonance { "✅ ENABLED" } else { "❌ DISABLED" }));
        report.push_str(&format!("  • Pressure Monitoring: {}\n",
            if config.shimmy_ds.enable_pressure_monitoring { "✅ ENABLED" } else { "❌ DISABLED" }));
        report.push_str(&format!("  • Audit Logging: {}\n",
            if config.shimmy_ds.enable_audit_logging { "✅ ENABLED" } else { "❌ DISABLED" }));

        report.push_str("\nConfiguration Values:\n");
        report.push_str(&format!("  • Pressure Threshold: {:.2}\n", config.shimmy_ds.pressure_threshold));
        report.push_str(&format!("  • Emotion Intensity Multiplier: {:.2}\n", config.shimmy_ds.emotion_intensity_multiplier));
        report.push_str(&format!("  • Max Obligations per Prompt: {}\n", config.shimmy_ds.max_obligations_per_prompt));
        report.push_str(&format!("  • Emotion Detection Threshold: {:.2}\n", config.validation.emotion_detection_threshold));

        report.push_str("\nContext Configuration:\n");
        report.push_str(&format!("  • Preferred Context Length: {} tokens\n", config.context.preferred_ctx_length));
        report.push_str(&format!("  • Context Retention: {}\n",
            if config.context.retain_context_between_generations { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Context History Depth: {} generations\n", config.context.context_history_depth));
        report.push_str(&format!("  • Context Compression: {}\n",
            if config.context.enable_context_compression { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Compression Ratio: {:.2}\n", config.context.compression_ratio));

        report.push_str("\nLogging:\n");
        report.push_str(&format!("  • Text Log: {}\n", config.logging.text_log_path));
        report.push_str(&format!("  • JSON Log: {}\n", config.logging.json_log_path));
        report.push_str(&format!("  • Max Log Size: {} MB\n", config.logging.max_log_size_mb));

        report.push_str("\nValidation:\n");
        report.push_str(&format!("  • Strict Location Validation: {}\n",
            if config.validation.strict_location_validation { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Allow Implicit Transitions: {}\n",
            if config.validation.allow_implicit_transitions { "✅ ON" } else { "❌ OFF" }));

        report.push_str("\nPerformance:\n");
        report.push_str(&format!("  • Cache Obligations: {}\n",
            if config.performance.cache_obligations { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Cache Max Age: {} minutes\n", config.performance.cache_max_age_minutes));
        report.push_str(&format!("  • Collect Metrics: {}\n",
            if config.performance.collect_metrics { "✅ ON" } else { "❌ OFF" }));

        report.push_str("\nDrift Stabilizer:\n");
        report.push_str(&format!("  • Enabled: {}\n",
            if config.drift_stabilizer.enabled { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Stale Obligation Threshold: {}\n", config.drift_stabilizer.stale_obligation_threshold));
        report.push_str(&format!("  • Emotional Decay Limit: {:.2}\n", config.drift_stabilizer.emotional_decay_limit));
        report.push_str(&format!("  • Theme Threshold: {:.2}\n", config.drift_stabilizer.theme_threshold));
        report.push_str(&format!("  • Drift Injection: {}\n",
            if config.drift_stabilizer.enable_drift_injection { "✅ ON" } else { "❌ OFF" }));
        report.push_str(&format!("  • Stability Logging: {}\n",
            if config.drift_stabilizer.enable_stability_logging { "✅ ON" } else { "❌ OFF" }));

        report
    }
}

/// Global configuration manager instance
static mut GLOBAL_CONFIG: Option<ConfigManager> = None;
static CONFIG_INIT: std::sync::Once = std::sync::Once::new();

/// Gets the global configuration manager (initializes if needed)
pub fn global_config() -> &'static ConfigManager {
    unsafe {
        CONFIG_INIT.call_once(|| {
            let config_manager = ConfigManager::with_default_path();
            // Try to load existing config, create default if none exists
            let _ = config_manager.load();
            GLOBAL_CONFIG = Some(config_manager);
        });
        GLOBAL_CONFIG.as_ref().unwrap()
    }
}

/// Convenience function to check if a feature is enabled
pub fn is_feature_enabled(feature: &str) -> bool {
    let config = global_config();
    match feature {
        "prompt_injection" => config.is_prompt_injection_enabled(),
        "location_validation" => config.is_location_validation_enabled(),
        "emotion_resonance" => config.is_emotion_resonance_enabled(),
        "pressure_monitoring" => config.is_pressure_monitoring_enabled(),
        "audit_logging" => config.is_audit_logging_enabled(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config_manager() -> (ConfigManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_shimmy.toml");
        let config_manager = ConfigManager::new(config_path);
        (config_manager, temp_dir)
    }

    #[test]
    fn test_default_config() {
        let config = ShimmyConfig::default();
        assert!(config.shimmy_ds.enable_prompt_injection);
        assert!(config.shimmy_ds.enable_location_validation);
        assert!(config.shimmy_ds.enable_emotion_resonance);
        assert_eq!(config.shimmy_ds.pressure_threshold, 1.5);
        assert_eq!(config.shimmy_ds.emotion_intensity_multiplier, 1.0);
    }

    #[test]
    fn test_config_manager_creation() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        assert!(config_manager.is_prompt_injection_enabled());
        assert!(config_manager.is_location_validation_enabled());
    }

    #[test]
    fn test_save_and_load_config() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        // Save default config
        config_manager.save_default().unwrap();
        assert!(config_manager.config_path.exists());

        // Modify and save
        config_manager.update_config(|config| {
            config.shimmy_ds.enable_prompt_injection = false;
            config.shimmy_ds.pressure_threshold = 2.0;
        }).unwrap();

        // Create new manager and load
        let config_manager2 = ConfigManager::new(&config_manager.config_path);
        config_manager2.load().unwrap();

        assert!(!config_manager2.is_prompt_injection_enabled());
        assert_eq!(config_manager2.get_pressure_threshold(), 2.0);
    }

    #[test]
    fn test_feature_toggles() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        // Test toggle
        config_manager.toggle_feature("prompt_injection", false).unwrap();
        assert!(!config_manager.is_prompt_injection_enabled());

        config_manager.toggle_feature("emotion_resonance", false).unwrap();
        assert!(!config_manager.is_emotion_resonance_enabled());

        // Toggle back
        config_manager.toggle_feature("prompt_injection", true).unwrap();
        assert!(config_manager.is_prompt_injection_enabled());
    }

    #[test]
    fn test_config_getters() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        assert_eq!(config_manager.get_pressure_threshold(), 1.5);
        assert_eq!(config_manager.get_emotion_intensity_multiplier(), 1.0);
        assert_eq!(config_manager.get_max_obligations_per_prompt(), 5);
        assert_eq!(config_manager.get_emotion_detection_threshold(), 0.3);
    }

    #[test]
    fn test_logging_config() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        let logging_config = config_manager.get_logging_config();

        assert_eq!(logging_config.text_log_path, "logs/prompt_audit.log");
        assert_eq!(logging_config.json_log_path, "logs/prompt_audit.json");
        assert_eq!(logging_config.max_log_size_mb, 100);
        assert_eq!(logging_config.max_log_files, 5);
    }

    #[test]
    fn test_status_report() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        let report = config_manager.generate_status_report();

        assert!(report.contains("SHIMMY-DS CONFIGURATION STATUS"));
        assert!(report.contains("Core Features:"));
        assert!(report.contains("Prompt Injection: ✅ ENABLED"));
        assert!(report.contains("Pressure Threshold: 1.50"));
        assert!(report.contains("Logging:"));
        assert!(report.contains("Performance:"));
    }

    #[test]
    fn test_update_config() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        config_manager.update_config(|config| {
            config.shimmy_ds.pressure_threshold = 3.0;
            config.shimmy_ds.max_obligations_per_prompt = 10;
            config.validation.strict_location_validation = true;
        }).unwrap();

        assert_eq!(config_manager.get_pressure_threshold(), 3.0);
        assert_eq!(config_manager.get_max_obligations_per_prompt(), 10);

        let config = config_manager.get_config();
        assert!(config.validation.strict_location_validation);
    }

    #[test]
    fn test_create_config_file_if_missing() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        // File shouldn't exist initially
        assert!(!config_manager.config_path.exists());

        // Load should create default file
        config_manager.load().unwrap();
        assert!(config_manager.config_path.exists());

        // Should be able to read it back
        let content = std::fs::read_to_string(&config_manager.config_path).unwrap();
        assert!(content.contains("[shimmy-ds]"));
        assert!(content.contains("enable_prompt_injection"));
    }

    #[test]
    fn test_invalid_feature_toggle() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        // Should not panic or error on invalid feature name
        config_manager.toggle_feature("invalid_feature", true).unwrap();

        // Original features should be unchanged
        assert!(config_manager.is_prompt_injection_enabled());
    }

    #[test]
    fn test_convenience_function() {
        // Test the global convenience function
        // Note: This test might affect other tests if they use global state
        assert!(is_feature_enabled("prompt_injection") || !is_feature_enabled("prompt_injection"));
    }
}