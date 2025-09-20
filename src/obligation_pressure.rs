/// 📊 PHASE 3: Obligation Saturation Index
///
/// Measures narrative pressure from unresolved obligations.
/// This module provides functionality to calculate and monitor the pressure
/// exerted by accumulated narrative obligations over time.

use serde::{Deserialize, Serialize};

/// Represents a narrative obligation with associated metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obligation {
    /// The type or category of obligation (e.g., "character_location", "mystery", "promise")
    pub kind: String,
    /// Urgency level from 0.0 (low) to 1.0 (critical)
    pub urgency: f32,
    /// Number of chapters/scenes since this obligation was introduced
    pub age: usize,
}

impl Obligation {
    /// Creates a new obligation with specified parameters
    pub fn new(kind: impl Into<String>, urgency: f32, age: usize) -> Self {
        Self {
            kind: kind.into(),
            urgency: urgency.clamp(0.0, 1.0), // Ensure urgency stays within valid range
            age,
        }
    }

    /// Creates a new obligation with default urgency (0.5) and age (0)
    pub fn from_kind(kind: impl Into<String>) -> Self {
        Self::new(kind, 0.5, 0)
    }

    /// Calculates the pressure contribution of this individual obligation
    pub fn pressure_contribution(&self) -> f32 {
        self.urgency * self.age as f32
    }

    /// Ages the obligation by incrementing its age counter
    pub fn age_obligation(&mut self) {
        self.age += 1;
    }

    /// Increases urgency by a specified amount (clamped to 1.0)
    pub fn increase_urgency(&mut self, amount: f32) {
        self.urgency = (self.urgency + amount).min(1.0);
    }

    /// Decreases urgency by a specified amount (clamped to 0.0)
    pub fn decrease_urgency(&mut self, amount: f32) {
        self.urgency = (self.urgency - amount).max(0.0);
    }
}

/// Computes the overall narrative pressure (saturation index) from a collection of obligations
///
/// The saturation index is calculated as the average pressure contribution across all obligations.
/// Higher values indicate greater narrative pressure requiring resolution.
///
/// # Arguments
/// * `obligs` - A slice of obligations to analyze
///
/// # Returns
/// A float representing the saturation index. Values above 1.5 typically indicate high pressure.
///
/// # Example
/// ```
/// use shimmy_ds::obligation_pressure::{Obligation, compute_saturation};
///
/// let obligations = vec![
///     Obligation::new("mystery", 0.8, 3),
///     Obligation::new("promise", 0.6, 2),
/// ];
/// let pressure = compute_saturation(&obligations);
/// // pressure = (0.8*3 + 0.6*2) / 3 = 3.6 / 3 = 1.2
/// ```
pub fn compute_saturation(obligs: &[Obligation]) -> f32 {
    if obligs.is_empty() {
        return 0.0;
    }

    let total_pressure: f32 = obligs.iter()
        .map(|o| o.pressure_contribution())
        .sum();

    total_pressure / (obligs.len() as f32 + 1.0)
}

/// Analyzes pressure distribution across different obligation types
///
/// # Arguments
/// * `obligs` - A slice of obligations to analyze
///
/// # Returns
/// A vector of tuples containing (obligation_type, average_pressure)
pub fn analyze_pressure_by_type(obligs: &[Obligation]) -> Vec<(String, f32)> {
    use std::collections::HashMap;

    let mut type_pressure: HashMap<String, Vec<f32>> = HashMap::new();

    for oblig in obligs {
        type_pressure
            .entry(oblig.kind.clone())
            .or_insert_with(Vec::new)
            .push(oblig.pressure_contribution());
    }

    let mut results: Vec<(String, f32)> = type_pressure
        .into_iter()
        .map(|(kind, pressures)| {
            let avg_pressure = pressures.iter().sum::<f32>() / pressures.len() as f32;
            (kind, avg_pressure)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results
}

/// Identifies obligations that contribute most significantly to overall pressure
///
/// # Arguments
/// * `obligs` - A slice of obligations to analyze
/// * `threshold` - Minimum pressure contribution to be considered "high pressure"
///
/// # Returns
/// A vector of references to high-pressure obligations
pub fn identify_high_pressure_obligations(obligs: &[Obligation], threshold: f32) -> Vec<&Obligation> {
    obligs.iter()
        .filter(|o| o.pressure_contribution() >= threshold)
        .collect()
}

/// Suggests actions based on current pressure levels
///
/// # Arguments
/// * `pressure` - Current saturation index
///
/// # Returns
/// A string with suggested actions
pub fn pressure_recommendations(pressure: f32) -> String {
    match pressure {
        p if p >= 3.0 => "🚨 CRITICAL: Immediate resolution required. Consider emergency resolution injection.".to_string(),
        p if p >= 2.0 => "⚠️ HIGH: Multiple obligations need attention. Plan resolution sequences.".to_string(),
        p if p >= 1.5 => "⚡ ELEVATED: Monitor closely. Recommend resolution injection.".to_string(),
        p if p >= 1.0 => "📊 MODERATE: Normal narrative tension. Continue monitoring.".to_string(),
        p if p >= 0.5 => "✅ LOW: Healthy narrative pace. Consider introducing new elements.".to_string(),
        _ => "🔄 MINIMAL: Very low pressure. Story may benefit from new obligations.".to_string(),
    }
}

/// Creates a comprehensive pressure report
///
/// # Arguments
/// * `obligs` - A slice of obligations to analyze
///
/// # Returns
/// A formatted string containing detailed pressure analysis
pub fn generate_pressure_report(obligs: &[Obligation]) -> String {
    let pressure = compute_saturation(obligs);
    let recommendations = pressure_recommendations(pressure);
    let type_analysis = analyze_pressure_by_type(obligs);
    let high_pressure = identify_high_pressure_obligations(obligs, 1.0);

    let mut report = String::new();
    report.push_str(&format!("📊 OBLIGATION PRESSURE REPORT\n"));
    report.push_str(&format!("================================\n"));
    report.push_str(&format!("Overall Saturation Index: {:.2}\n", pressure));
    report.push_str(&format!("Total Obligations: {}\n", obligs.len()));
    report.push_str(&format!("Recommendation: {}\n\n", recommendations));

    if !type_analysis.is_empty() {
        report.push_str("📋 Pressure by Type:\n");
        for (kind, avg_pressure) in type_analysis {
            report.push_str(&format!("  • {}: {:.2}\n", kind, avg_pressure));
        }
        report.push('\n');
    }

    if !high_pressure.is_empty() {
        report.push_str("⚡ High Pressure Obligations:\n");
        for oblig in high_pressure {
            report.push_str(&format!(
                "  • {} (urgency: {:.2}, age: {}, pressure: {:.2})\n",
                oblig.kind, oblig.urgency, oblig.age, oblig.pressure_contribution()
            ));
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligation_creation() {
        let oblig = Obligation::new("mystery", 0.8, 3);
        assert_eq!(oblig.kind, "mystery");
        assert_eq!(oblig.urgency, 0.8);
        assert_eq!(oblig.age, 3);
    }

    #[test]
    fn test_obligation_from_kind() {
        let oblig = Obligation::from_kind("test");
        assert_eq!(oblig.kind, "test");
        assert_eq!(oblig.urgency, 0.5);
        assert_eq!(oblig.age, 0);
    }

    #[test]
    fn test_urgency_clamping() {
        let oblig1 = Obligation::new("test", 1.5, 0); // Should clamp to 1.0
        assert_eq!(oblig1.urgency, 1.0);

        let oblig2 = Obligation::new("test", -0.5, 0); // Should clamp to 0.0
        assert_eq!(oblig2.urgency, 0.0);
    }

    #[test]
    fn test_pressure_contribution() {
        let oblig = Obligation::new("mystery", 0.8, 3);
        assert_eq!(oblig.pressure_contribution(), 2.4); // 0.8 * 3
    }

    #[test]
    fn test_age_obligation() {
        let mut oblig = Obligation::new("test", 0.5, 2);
        oblig.age_obligation();
        assert_eq!(oblig.age, 3);
    }

    #[test]
    fn test_urgency_modification() {
        let mut oblig = Obligation::new("test", 0.5, 0);

        oblig.increase_urgency(0.3);
        assert_eq!(oblig.urgency, 0.8);

        oblig.increase_urgency(0.5); // Should clamp to 1.0
        assert_eq!(oblig.urgency, 1.0);

        oblig.decrease_urgency(0.7);
        assert_eq!(oblig.urgency, 0.3);

        oblig.decrease_urgency(0.5); // Should clamp to 0.0
        assert_eq!(oblig.urgency, 0.0);
    }

    #[test]
    fn test_compute_saturation_empty() {
        let obligs = vec![];
        assert_eq!(compute_saturation(&obligs), 0.0);
    }

    #[test]
    fn test_compute_saturation_single() {
        let obligs = vec![Obligation::new("mystery", 0.8, 3)];
        let expected = 2.4 / 2.0; // (0.8 * 3) / (1 + 1)
        assert_eq!(compute_saturation(&obligs), expected);
    }

    #[test]
    fn test_compute_saturation_multiple() {
        let obligs = vec![
            Obligation::new("mystery", 0.8, 3),   // 2.4
            Obligation::new("promise", 0.6, 2),   // 1.2
        ];
        let expected = (2.4 + 1.2) / 3.0; // 3.6 / 3.0 = 1.2
        assert_eq!(compute_saturation(&obligs), expected);
    }

    #[test]
    fn test_analyze_pressure_by_type() {
        let obligs = vec![
            Obligation::new("mystery", 0.8, 3),   // 2.4
            Obligation::new("mystery", 0.6, 2),   // 1.2
            Obligation::new("promise", 0.9, 1),   // 0.9
        ];

        let analysis = analyze_pressure_by_type(&obligs);
        assert_eq!(analysis.len(), 2);

        // Should be sorted by pressure (highest first)
        assert_eq!(analysis[0].0, "mystery");
        assert_eq!(analysis[0].1, (2.4 + 1.2) / 2.0); // 1.8

        assert_eq!(analysis[1].0, "promise");
        assert_eq!(analysis[1].1, 0.9);
    }

    #[test]
    fn test_identify_high_pressure_obligations() {
        let obligs = vec![
            Obligation::new("mystery", 0.8, 3),   // 2.4 - high
            Obligation::new("promise", 0.6, 2),   // 1.2 - high
            Obligation::new("detail", 0.3, 1),    // 0.3 - low
        ];

        let high_pressure = identify_high_pressure_obligations(&obligs, 1.0);
        assert_eq!(high_pressure.len(), 2);
        assert_eq!(high_pressure[0].kind, "mystery");
        assert_eq!(high_pressure[1].kind, "promise");
    }

    #[test]
    fn test_pressure_recommendations() {
        assert!(pressure_recommendations(3.5).contains("CRITICAL"));
        assert!(pressure_recommendations(2.5).contains("HIGH"));
        assert!(pressure_recommendations(1.7).contains("ELEVATED"));
        assert!(pressure_recommendations(1.2).contains("MODERATE"));
        assert!(pressure_recommendations(0.7).contains("LOW"));
        assert!(pressure_recommendations(0.2).contains("MINIMAL"));
    }

    #[test]
    fn test_generate_pressure_report() {
        let obligs = vec![
            Obligation::new("mystery", 0.8, 3),
            Obligation::new("promise", 0.6, 2),
        ];

        let report = generate_pressure_report(&obligs);
        assert!(report.contains("OBLIGATION PRESSURE REPORT"));
        assert!(report.contains("Saturation Index"));
        assert!(report.contains("Total Obligations: 2"));
        assert!(report.contains("mystery"));
        assert!(report.contains("promise"));
    }

    #[test]
    fn test_empty_obligations_report() {
        let obligs = vec![];
        let report = generate_pressure_report(&obligs);
        assert!(report.contains("Total Obligations: 0"));
        assert!(report.contains("Saturation Index: 0.00"));
    }
}