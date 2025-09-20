/**
 * Telemetry Module - Narrative System Monitoring
 * ==============================================
 *
 * Provides lightweight telemetry and monitoring capabilities for the
 * narrative intelligence system.
 */

pub mod pulse_trace;

pub use pulse_trace::{Pulse, PulseTrace, PulseTraceHealthStats};

/// Re-export helper functions for convenience
pub mod helpers {
    pub use super::pulse_trace::helpers::*;
}