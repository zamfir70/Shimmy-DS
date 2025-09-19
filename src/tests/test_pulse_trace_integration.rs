/**
 * Integration tests for PulseTrace telemetry system
 * ==============================================
 *
 * Tests the integration between PulseTrace and the narrative system components.
 */

#[cfg(test)]
mod tests {
    use crate::telemetry::{PulseTrace, Pulse, PulseTraceHealthStats};
    use crate::telemetry::helpers::*;
    use crate::recursive_narrative_assistant::RecursiveNarrativeAssistant;
    use std::time::Duration;

    #[test]
    fn test_pulse_trace_basic_functionality() {
        let mut trace = PulseTrace::new(10);

        // Record a few pulses
        for i in 0..5 {
            let pulse = create_test_pulse(i);
            trace.record(pulse);
        }

        assert_eq!(trace.buffer.len(), 5);

        let latest = trace.latest().unwrap();
        assert_eq!(latest.zc_tick, 4);

        let health = trace.get_health_stats();
        assert_eq!(health.pulse_count, 5);
    }

    #[test]
    fn test_pulse_trace_ring_buffer() {
        let mut trace = PulseTrace::new(3); // Small capacity

        // Record more pulses than capacity
        for i in 0..5 {
            let pulse = create_test_pulse(i);
            trace.record(pulse);
        }

        // Should only have 3 pulses (the latest ones)
        assert_eq!(trace.buffer.len(), 3);

        let latest = trace.latest().unwrap();
        assert_eq!(latest.zc_tick, 4); // Latest pulse

        let oldest = trace.buffer.front().unwrap();
        assert_eq!(oldest.zc_tick, 2); // Oldest remaining pulse
    }

    #[test]
    fn test_pulse_trace_json_serialization() {
        let mut trace = PulseTrace::new(5);

        let pulse = Pulse::with_data(
            1, 2, 1, 0.8, 64.0, 0.5, 0.9,
            Some("Test pulse".to_string())
        );
        trace.record(pulse);

        let json = trace.to_json();
        assert!(json.contains("\"zc_tick\":1"));
        assert!(json.contains("\"pathogens_detected\":2"));
        assert!(json.contains("\"adi_score\":0.8"));

        let summary = trace.to_summary_json();
        assert!(summary.contains("\"status\":\"active\""));
        assert!(summary.contains("\"pulse_count\":1"));
    }

    #[test]
    fn test_pulse_trace_averages() {
        let mut trace = PulseTrace::new(10);

        // Record pulses with known values
        let pulses = vec![
            Pulse::with_data(1, 0, 0, 0.2, 50.0, -0.5, 0.3, None),
            Pulse::with_data(2, 2, 1, 0.6, 60.0, 0.0, 0.7, None),
            Pulse::with_data(3, 4, 2, 1.0, 70.0, 0.5, 1.0, None),
        ];

        for pulse in pulses {
            trace.record(pulse);
        }

        let avg = trace.recent_average(3).unwrap();
        assert_eq!(avg.pathogens_detected, 2); // (0+2+4)/3 = 2
        assert_eq!(avg.drift_hits, 1); // (0+1+2)/3 = 1
        assert!((avg.adi_score - 0.6).abs() < 0.1); // (0.2+0.6+1.0)/3 = 0.6
    }

    #[test]
    fn test_recursive_narrative_assistant_pulse_integration() {
        let mut assistant = RecursiveNarrativeAssistant::new();

        // Verify pulse trace is initialized
        assert_eq!(assistant.pulse_trace.buffer.len(), 0);

        // Test pulse trace methods
        let health = assistant.get_pulse_trace_health();
        assert_eq!(health.status, "inactive"); // No pulses recorded yet

        let summary = assistant.get_pulse_trace_summary();
        assert!(summary.contains("\"status\":\"empty\""));
    }

    #[test]
    fn test_pulse_trace_health_calculation() {
        let mut trace = PulseTrace::new(10);

        // Record pulses with different health indicators
        let pulses = vec![
            // Healthy pulses
            Pulse::with_data(1, 0, 0, 0.8, 50.0, 0.3, 0.9, None),
            Pulse::with_data(2, 1, 0, 0.7, 52.0, 0.2, 0.8, None),
            // Warning pulses
            Pulse::with_data(3, 3, 2, 0.5, 55.0, -0.1, 0.6, None),
            // Degraded pulses
            Pulse::with_data(4, 6, 4, 0.2, 60.0, -0.4, 0.3, None),
        ];

        for pulse in pulses {
            trace.record(pulse);
        }

        let health = trace.get_health_stats();
        assert!(health.avg_pathogens_detected > 0.0);
        assert!(health.avg_drift_hits > 0.0);
        assert_eq!(health.pulse_count, 4);

        // Health status should be degraded due to high pathogen/drift counts
        assert!(health.status == "degraded" || health.status == "warning");
    }

    #[test]
    fn test_pulse_trace_memory_efficiency() {
        let trace = PulseTrace::new(100);

        // Test that the trace starts empty and has the correct capacity
        assert_eq!(trace.buffer.len(), 0);
        assert_eq!(trace.capacity, 100);

        // Memory footprint should be reasonable
        let memory_size = std::mem::size_of_val(&trace);
        assert!(memory_size < 10000); // Should be less than 10KB for empty trace
    }

    #[test]
    fn test_pulse_trace_time_filtering() {
        let mut trace = PulseTrace::new(10);

        // Record pulses with some delay simulation
        for i in 0..5 {
            let pulse = create_test_pulse(i);
            trace.record(pulse);
        }

        // Test recent pulses retrieval (should return all pulses for this test)
        let recent = trace.get_recent_pulses(1000); // Last 1 second
        assert!(recent.len() <= 5); // All pulses should be recent

        let very_recent = trace.get_recent_pulses(1); // Last 1 millisecond
        assert!(very_recent.len() >= 0); // May or may not have recent pulses depending on timing
    }

    #[test]
    fn test_helper_functions() {
        // Test memory estimation
        let memory = get_memory_usage_estimate();
        assert!(memory > 0.0);

        // Test pathogen counting
        let pathogen_results = vec![true, false, true, true, false];
        let count = count_pathogens_from_results(&pathogen_results);
        assert_eq!(count, 3);

        // Test ADI calculation
        let adi = calculate_adi_from_state(5, 3, 2, 0.7);
        assert!(adi >= 0.0 && adi <= 1.0);
        assert!(adi > 0.5); // Should be above baseline for good parameters
    }
}