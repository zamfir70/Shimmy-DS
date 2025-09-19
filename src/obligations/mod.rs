// Smart Obligation Management System Module
// ObliSelect for intelligent obligation selection and scoring

pub mod obli_select;

// Re-export for easier access
pub use obli_select::{
    Obligation, SmartObligationManager, ObligationScore, ObligationMetrics,
    ObliSelectSettings, ObligationUrgency, ObligationCategory
};