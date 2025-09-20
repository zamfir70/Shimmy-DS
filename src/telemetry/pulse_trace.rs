/**
 * PulseTrace: Lightweight Narrative System Telemetry Logger
 * =========================================================
 *
 * Implements Claude Code Card #1: pulse_trace.rs (Telemetry Ring Logger)
 *
 * Purpose: Track recursion pressure, drift detections, pathogen alerts,
 *          memory usage, and ADI in a lightweight, serializable format.
 *
 * Features:
 * - Ring buffer design for memory efficiency
 * - JSON serialization for export
 * - Sliding window averages
 * - Standalone operation (no external deps)
 * - Light memory footprint (8KB per 100 pulses)
 */

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Single telemetry pulse capturing narrative system state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pulse {
    /// When this pulse was recorded
    #[serde(skip, default = "Instant::now")]  // Skip Instant for JSON serialization, use current time as default
    pub timestamp: Instant,

    /// Timestamp as milliseconds since recording start (for JSON export)
    pub timestamp_ms: u64,

    /// Zero-Continuation gate tick counter
    pub zc_tick: usize,

    /// Number of narrative pathogens detected in this cycle
    pub pathogens_detected: usize,

    /// Number of drift detection hits
    pub drift_hits: usize,

    /// Adaptive Depth Intelligence score (0.0-1.0)
    pub adi_score: f32,

    /// Current memory usage in megabytes
    pub memory_usage_mb: f32,

    /// Affective pleasure signal (-1.0 to 1.0)
    pub affect_pleasure: f32,

    /// Affective coherence signal (0.0 to 1.0)
    pub affect_coherence: f32,

    /// Optional notes for this pulse
    pub notes: Option<String>,
}

impl Pulse {
    /// Create a new pulse with current timestamp
    pub fn new() -> Self {
        Self {
            timestamp: Instant::now(),
            timestamp_ms: 0, // Will be set when added to PulseTrace
            zc_tick: 0,
            pathogens_detected: 0,
            drift_hits: 0,
            adi_score: 0.0,
            memory_usage_mb: 0.0,
            affect_pleasure: 0.0,
            affect_coherence: 0.0,
            notes: None,
        }
    }

    /// Create a pulse with all fields specified
    pub fn with_data(
        zc_tick: usize,
        pathogens_detected: usize,
        drift_hits: usize,
        adi_score: f32,
        memory_usage_mb: f32,
        affect_pleasure: f32,
        affect_coherence: f32,
        notes: Option<String>,
    ) -> Self {
        Self {
            timestamp: Instant::now(),
            timestamp_ms: 0, // Will be set when added to PulseTrace
            zc_tick,
            pathogens_detected,
            drift_hits,
            adi_score,
            memory_usage_mb,
            affect_pleasure,
            affect_coherence,
            notes,
        }
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::new()
    }
}

/// Ring buffer telemetry logger for narrative system pulses
#[derive(Debug, Clone)]
pub struct PulseTrace {
    /// Ring buffer of pulses
    pub buffer: VecDeque<Pulse>,

    /// Maximum capacity of the ring buffer
    pub capacity: usize,

    /// When the trace was started (for relative timestamps)
    start_time: Instant,
}

impl Default for PulseTrace {
    fn default() -> Self {
        Self::new(512) // Default capacity
    }
}

impl PulseTrace {
    /// Create a new PulseTrace with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            start_time: Instant::now(),
        }
    }

    /// Record a new pulse, evicting oldest if at capacity
    pub fn record(&mut self, mut pulse: Pulse) {
        // Set relative timestamp for JSON export
        pulse.timestamp_ms = pulse.timestamp.duration_since(self.start_time).as_millis() as u64;

        // Maintain ring buffer capacity
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(pulse);
    }

    /// Get recent average over specified window size
    pub fn recent_average(&self, window: usize) -> Option<Pulse> {
        if self.buffer.is_empty() {
            return None;
        }

        let window_size = window.min(self.buffer.len());
        let slice: Vec<&Pulse> = self.buffer.iter().rev().take(window_size).collect();

        if slice.is_empty() {
            return None;
        }

        let count = slice.len() as f32;

        Some(Pulse {
            timestamp: Instant::now(),
            timestamp_ms: self.get_current_timestamp_ms(),
            zc_tick: slice.first().unwrap().zc_tick, // Use most recent zc_tick
            pathogens_detected: (slice.iter().map(|p| p.pathogens_detected).sum::<usize>() as f32 / count) as usize,
            drift_hits: (slice.iter().map(|p| p.drift_hits).sum::<usize>() as f32 / count) as usize,
            adi_score: slice.iter().map(|p| p.adi_score).sum::<f32>() / count,
            memory_usage_mb: slice.iter().map(|p| p.memory_usage_mb).sum::<f32>() / count,
            affect_pleasure: slice.iter().map(|p| p.affect_pleasure).sum::<f32>() / count,
            affect_coherence: slice.iter().map(|p| p.affect_coherence).sum::<f32>() / count,
            notes: Some(format!("Average over {} pulses", window_size)),
        })
    }

    /// Get the latest pulse
    pub fn latest(&self) -> Option<&Pulse> {
        self.buffer.back()
    }

    /// Get current timestamp relative to start
    fn get_current_timestamp_ms(&self) -> u64 {
        Instant::now().duration_since(self.start_time).as_millis() as u64
    }

    /// Get all pulses in chronological order
    pub fn get_all_pulses(&self) -> Vec<&Pulse> {
        self.buffer.iter().collect()
    }

    /// Get pulses from the last N milliseconds
    pub fn get_recent_pulses(&self, duration_ms: u64) -> Vec<&Pulse> {
        let current_time_ms = self.get_current_timestamp_ms();
        let cutoff_time_ms = current_time_ms.saturating_sub(duration_ms);

        self.buffer
            .iter()
            .filter(|pulse| pulse.timestamp_ms >= cutoff_time_ms)
            .collect()
    }

    /// Export to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.buffer).unwrap_or_else(|_| "[]".to_string())
    }

    /// Export summary statistics to JSON
    pub fn to_summary_json(&self) -> String {
        if self.buffer.is_empty() {
            return r#"{"status":"empty","pulse_count":0}"#.to_string();
        }

        let pulse_count = self.buffer.len();
        let latest = self.buffer.back().unwrap();
        let average_5 = self.recent_average(5);
        let average_20 = self.recent_average(20);

        let summary = serde_json::json!({
            "status": "active",
            "pulse_count": pulse_count,
            "capacity": self.capacity,
            "uptime_ms": self.get_current_timestamp_ms(),
            "latest": {
                "zc_tick": latest.zc_tick,
                "pathogens_detected": latest.pathogens_detected,
                "drift_hits": latest.drift_hits,
                "adi_score": latest.adi_score,
                "memory_usage_mb": latest.memory_usage_mb,
                "affect_pleasure": latest.affect_pleasure,
                "affect_coherence": latest.affect_coherence,
                "timestamp_ms": latest.timestamp_ms
            },
            "average_5_pulses": average_5.as_ref().map(|avg| serde_json::json!({
                "pathogens_detected": avg.pathogens_detected,
                "drift_hits": avg.drift_hits,
                "adi_score": avg.adi_score,
                "memory_usage_mb": avg.memory_usage_mb,
                "affect_pleasure": avg.affect_pleasure,
                "affect_coherence": avg.affect_coherence
            })),
            "average_20_pulses": average_20.as_ref().map(|avg| serde_json::json!({
                "pathogens_detected": avg.pathogens_detected,
                "drift_hits": avg.drift_hits,
                "adi_score": avg.adi_score,
                "memory_usage_mb": avg.memory_usage_mb,
                "affect_pleasure": avg.affect_pleasure,
                "affect_coherence": avg.affect_coherence
            }))
        });

        serde_json::to_string(&summary).unwrap_or_else(|_| r#"{"status":"error"}"#.to_string())
    }

    /// Clear all pulses
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.start_time = Instant::now();
    }

    /// Get telemetry health statistics
    pub fn get_health_stats(&self) -> PulseTraceHealthStats {
        if self.buffer.is_empty() {
            return PulseTraceHealthStats::default();
        }

        let pulse_count = self.buffer.len();
        let latest = self.buffer.back().unwrap();

        // Calculate trends over last 10 pulses
        let recent_window = 10.min(pulse_count);
        let recent_pulses: Vec<&Pulse> = self.buffer.iter().rev().take(recent_window).collect();

        let avg_pathogens = recent_pulses.iter().map(|p| p.pathogens_detected).sum::<usize>() as f32 / recent_window as f32;
        let avg_drift_hits = recent_pulses.iter().map(|p| p.drift_hits).sum::<usize>() as f32 / recent_window as f32;
        let avg_adi_score = recent_pulses.iter().map(|p| p.adi_score).sum::<f32>() / recent_window as f32;
        let avg_memory = recent_pulses.iter().map(|p| p.memory_usage_mb).sum::<f32>() / recent_window as f32;

        // Determine health status
        let health_status = if avg_pathogens > 5.0 || avg_drift_hits > 3.0 || avg_adi_score < 0.3 {
            "degraded".to_string()
        } else if avg_pathogens > 2.0 || avg_drift_hits > 1.0 || avg_adi_score < 0.6 {
            "warning".to_string()
        } else {
            "healthy".to_string()
        };

        PulseTraceHealthStats {
            status: health_status,
            pulse_count,
            uptime_ms: self.get_current_timestamp_ms(),
            avg_pathogens_detected: avg_pathogens,
            avg_drift_hits: avg_drift_hits,
            avg_adi_score: avg_adi_score,
            avg_memory_usage_mb: avg_memory,
            latest_zc_tick: latest.zc_tick,
            latest_affect_pleasure: latest.affect_pleasure,
            latest_affect_coherence: latest.affect_coherence,
        }
    }
}

/// Health statistics for PulseTrace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseTraceHealthStats {
    pub status: String,
    pub pulse_count: usize,
    pub uptime_ms: u64,
    pub avg_pathogens_detected: f32,
    pub avg_drift_hits: f32,
    pub avg_adi_score: f32,
    pub avg_memory_usage_mb: f32,
    pub latest_zc_tick: usize,
    pub latest_affect_pleasure: f32,
    pub latest_affect_coherence: f32,
}

impl Default for PulseTraceHealthStats {
    fn default() -> Self {
        Self {
            status: "inactive".to_string(),
            pulse_count: 0,
            uptime_ms: 0,
            avg_pathogens_detected: 0.0,
            avg_drift_hits: 0.0,
            avg_adi_score: 0.0,
            avg_memory_usage_mb: 0.0,
            latest_zc_tick: 0,
            latest_affect_pleasure: 0.0,
            latest_affect_coherence: 0.0,
        }
    }
}

/// Helper functions for common telemetry operations
pub mod helpers {
    use super::*;

    /// Estimate memory usage (placeholder - would integrate with actual memory monitoring)
    pub fn get_memory_usage_estimate() -> f32 {
        // Placeholder implementation - in real usage, this would query actual memory usage
        // Could integrate with system metrics or Rust memory profilers
        50.0 // Default estimate in MB
    }

    /// Calculate ADI score based on current narrative state
    pub fn calculate_adi_from_state(
        constraint_count: usize,
        character_count: usize,
        recursion_depth: usize,
        narrative_complexity: f32
    ) -> f32 {
        // Simplified ADI calculation
        let base_score = 0.5;
        let constraint_factor = (constraint_count as f32 * 0.1).min(0.3);
        let character_factor = (character_count as f32 * 0.05).min(0.2);
        let depth_factor = (recursion_depth as f32 * 0.02).min(0.1);
        let complexity_factor = narrative_complexity * 0.3;

        (base_score + constraint_factor + character_factor + depth_factor + complexity_factor).min(1.0)
    }

    /// Count active pathogens from pathogen detection results
    pub fn count_pathogens_from_results(pathogen_results: &[bool]) -> usize {
        pathogen_results.iter().filter(|&&detected| detected).count()
    }

    /// Create a quick pulse for testing
    pub fn create_test_pulse(tick: usize) -> Pulse {
        Pulse::with_data(
            tick,
            (tick % 3) as usize, // Simulate pathogens
            (tick % 5) as usize, // Simulate drift hits
            0.5 + (tick as f32 * 0.1) % 0.5, // ADI score between 0.5-1.0
            45.0 + (tick as f32 * 2.0) % 20.0, // Memory usage 45-65 MB
            -0.5 + (tick as f32 * 0.1) % 1.0, // Affect pleasure -0.5 to 0.5
            0.3 + (tick as f32 * 0.05) % 0.7, // Affect coherence 0.3-1.0
            Some(format!("Test pulse {}", tick))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_pulse_creation() {
        let pulse = Pulse::new();
        assert_eq!(pulse.zc_tick, 0);
        assert_eq!(pulse.pathogens_detected, 0);
        assert_eq!(pulse.drift_hits, 0);
        assert_eq!(pulse.adi_score, 0.0);
    }

    #[test]
    fn test_pulse_with_data() {
        let pulse = Pulse::with_data(10, 2, 1, 0.8, 64.5, 0.3, 0.9, Some("test".to_string()));
        assert_eq!(pulse.zc_tick, 10);
        assert_eq!(pulse.pathogens_detected, 2);
        assert_eq!(pulse.drift_hits, 1);
        assert_eq!(pulse.adi_score, 0.8);
        assert_eq!(pulse.memory_usage_mb, 64.5);
        assert_eq!(pulse.affect_pleasure, 0.3);
        assert_eq!(pulse.affect_coherence, 0.9);
        assert_eq!(pulse.notes, Some("test".to_string()));
    }

    #[test]
    fn test_pulse_trace_creation() {
        let trace = PulseTrace::new(100);
        assert_eq!(trace.capacity, 100);
        assert_eq!(trace.buffer.len(), 0);
    }

    #[test]
    fn test_pulse_recording() {
        let mut trace = PulseTrace::new(3);

        // Record pulses
        for i in 0..5 {
            let pulse = Pulse::with_data(i, i, i, i as f32, i as f32, 0.0, 1.0, None);
            trace.record(pulse);
        }

        // Should only have 3 pulses (ring buffer)
        assert_eq!(trace.buffer.len(), 3);

        // Should have the last 3 pulses (2, 3, 4)
        let latest = trace.latest().unwrap();
        assert_eq!(latest.zc_tick, 4);
    }

    #[test]
    fn test_recent_average() {
        let mut trace = PulseTrace::new(10);

        // Record some test pulses
        for i in 0..5 {
            let pulse = Pulse::with_data(i, i * 2, i, i as f32 * 0.1, 50.0, 0.0, 1.0, None);
            trace.record(pulse);
        }

        let avg = trace.recent_average(3).unwrap();

        // Should average the last 3 pulses (2, 3, 4)
        assert_eq!(avg.pathogens_detected, (6 + 8 + 4) / 3); // (2*2 + 3*2 + 4*2) / 3 = 6
        assert_eq!(avg.drift_hits, (2 + 3 + 4) / 3); // (2 + 3 + 4) / 3 = 3
    }

    #[test]
    fn test_json_serialization() {
        let mut trace = PulseTrace::new(5);

        let pulse = Pulse::with_data(1, 2, 1, 0.8, 64.0, 0.5, 0.9, Some("test".to_string()));
        trace.record(pulse);

        let json = trace.to_json();
        assert!(json.contains("\"zc_tick\":1"));
        assert!(json.contains("\"pathogens_detected\":2"));
        assert!(json.contains("\"adi_score\":0.8"));
    }

    #[test]
    fn test_summary_json() {
        let mut trace = PulseTrace::new(10);

        // Add some test data
        for i in 0..3 {
            let pulse = helpers::create_test_pulse(i);
            trace.record(pulse);
        }

        let summary = trace.to_summary_json();
        assert!(summary.contains("\"status\":\"active\""));
        assert!(summary.contains("\"pulse_count\":3"));
    }

    #[test]
    fn test_health_stats() {
        let mut trace = PulseTrace::new(10);

        // Add some pulses with different health indicators
        for i in 0..5 {
            let pulse = Pulse::with_data(i, i % 2, i % 3, 0.7, 50.0, 0.0, 0.8, None);
            trace.record(pulse);
        }

        let health = trace.get_health_stats();
        assert_eq!(health.pulse_count, 5);
        assert!(health.avg_adi_score > 0.0);
        assert_eq!(health.status, "healthy");
    }

    #[test]
    fn test_helper_functions() {
        let memory = helpers::get_memory_usage_estimate();
        assert!(memory > 0.0);

        let adi = helpers::calculate_adi_from_state(5, 3, 2, 0.4);
        assert!(adi >= 0.0 && adi <= 1.0);

        let pathogen_results = vec![true, false, true, false];
        let count = helpers::count_pathogens_from_results(&pathogen_results);
        assert_eq!(count, 2);

        let test_pulse = helpers::create_test_pulse(42);
        assert_eq!(test_pulse.zc_tick, 42);
    }
}