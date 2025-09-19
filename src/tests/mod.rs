// PPT + Invariant Testing System Test Modules

pub mod ppt_contracts;

// Recursive Narrative System Integration Tests
pub mod recursive_integration_tests;

// Recursive Integrity Core (RIC v1.0) Integration Tests
pub mod ric_integration_tests;

// RIP+RIC Unified Protocol Stack Integration Tests
pub mod rip_ric_integration_tests;

// PulseTrace Telemetry Integration Tests
pub mod test_pulse_trace_integration;

// CacheMind Cross-System State Cache Integration Tests
pub mod test_cachemind_integration;

// AdaptIQ Narrative Intelligence Modulator Integration Tests
pub mod test_adaptiq_integration;

// Qualitier Adaptive Quality Control Integration Tests
pub mod test_qualitier_integration;

// ObliSelect Smart Obligation Management Integration Tests
pub mod test_obli_select_integration;

// Re-export for easier access
pub use crate::invariant_ppt::*;
