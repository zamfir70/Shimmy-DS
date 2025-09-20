pub mod api;
pub mod api_errors;
pub mod auto_discovery;
pub mod cache;
pub mod cli;
pub mod discovery;
pub mod engine;
pub mod error;
pub mod main_integration;
pub mod metrics;
pub mod model_manager;
pub mod model_registry;
pub mod openai_compat;
pub mod port_manager;
pub mod rustchain_compat;
pub mod safetensors_adapter;
pub mod server;
pub mod templates;
pub mod tools;
pub mod util {
    pub mod diag;
}
pub mod invariant_ppt;
pub mod workflow;

// SHIMMY-DS Augmentation System
pub mod prompt_injector;
pub mod waymark_validator;
pub mod obligation_pressure;
pub mod emotion_resonance;
pub mod prompt_audit;
pub mod shimmy_config;
pub mod recursive_drift_stabilizer;
pub mod stability_log;
pub mod stability_tracing;

// SHIMMY-DS Recursive Narrative System
pub mod narrative_dna;
pub mod constraint_space;
pub mod multi_level_recursion;
pub mod character_consistency;
pub mod reader_engagement_loops;
pub mod recursive_narrative_assistant;

// SHIMMY-DS Recursive Integrity Core (RIC v1.0)
pub mod recursive_integrity_core;

// SHIMMY-DS Telemetry System
pub mod telemetry;

// SHIMMY-DS Adaptive Intelligence System
pub mod adaptive;

// SHIMMY-DS Profile Persistence System
pub mod profile;

// SHIMMY-DS Obligation Management System
pub mod obligations;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod test_utils;

// Note: Mock infrastructure removed - use real testing with local models
// PPT + Invariant Testing System ensures semantic integrity under high-visibility development

pub struct AppState {
    pub engine: Box<dyn engine::InferenceEngine>,
    pub registry: model_registry::Registry,
}
