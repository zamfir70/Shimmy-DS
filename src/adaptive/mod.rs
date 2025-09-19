/**
 * Adaptive Intelligence Module
 * ============================
 *
 * This module contains the AdaptIQ system for runtime narrative intelligence modulation.
 * It dynamically adjusts analysis depth, pathogen sensitivity, and other parameters
 * based on content entropy, user preferences, and performance constraints.
 */

pub mod adapt_iq;
pub mod entropy_helpers;
pub mod qualitier;

// Re-export main components for easier access
pub use adapt_iq::{AdaptIQEngine, AdaptIQSettings, TasteLUT};
pub use entropy_helpers::*;
pub use qualitier::{Qualitier, QualityLevel, QualityFeature, PerformanceConfig, QualitierStatusReport};