/// 🧭 PHASE 2: Spatial Continuity Validator (WAYMARK Integration)
///
/// Checks if current prompt violates last known location.
/// This module provides functionality to validate location transitions
/// in narrative generation to ensure spatial continuity.

/// Validates whether generated output contains appropriate location transition indicators
/// when transitioning from a known last location.
///
/// This function checks if the generated text contains movement indicators that properly
/// acknowledge the character's departure from their last known location.
///
/// # Arguments
/// * `output` - The generated text to validate
/// * `last_location` - The character's last known location
///
/// # Returns
/// `true` if the output contains valid location transition indicators, `false` otherwise
///
/// # Example
/// ```
/// use shimmy_ds::waymark_validator::validate_location_transition;
///
/// let output = "Harper left the attic and walked downstairs";
/// let last_location = "attic";
/// let is_valid = validate_location_transition(output, last_location);
/// assert!(is_valid);
///
/// let invalid_output = "Harper was suddenly in the kitchen";
/// let is_invalid = validate_location_transition(invalid_output, last_location);
/// assert!(!is_invalid);
/// ```
pub fn validate_location_transition(output: &str, last_location: &str) -> bool {
    let indicators = vec![
        "left", "walked out", "stepped outside", "exited", "fled", "ran from",
        "departed", "moved from", "went out", "came out", "emerged from",
        "descended from", "climbed down from", "escaped from", "vacated",
    ];

    // Check if any transition indicator is present AND mentions the last location
    indicators.iter().any(|phrase| {
        output.to_lowercase().contains(phrase) &&
        output.to_lowercase().contains(&last_location.to_lowercase())
    })
}

/// Checks for implicit location transitions that might be valid without explicit movement
///
/// Some location changes might be narratively valid even without explicit movement verbs,
/// such as when showing a flashback, dream sequence, or scene cut.
///
/// # Arguments
/// * `output` - The generated text to check
///
/// # Returns
/// `true` if the output contains indicators of valid implicit transitions
pub fn has_implicit_transition_markers(output: &str) -> bool {
    let implicit_markers = vec![
        "meanwhile", "elsewhere", "suddenly", "then", "later", "after",
        "flashback", "dream", "vision", "memory", "scene", "cut to",
        "fade to", "dissolve to", "shift to",
    ];

    implicit_markers.iter().any(|marker| {
        output.to_lowercase().contains(marker)
    })
}

/// Comprehensive location transition validation that considers both explicit and implicit transitions
///
/// # Arguments
/// * `output` - The generated text to validate
/// * `last_location` - The character's last known location
///
/// # Returns
/// `true` if the location transition is valid (either explicit or implicitly acceptable)
pub fn is_valid_location_transition(output: &str, last_location: &str) -> bool {
    validate_location_transition(output, last_location) ||
    has_implicit_transition_markers(output)
}

/// Extracts potential new locations mentioned in the output
///
/// # Arguments
/// * `output` - The generated text to analyze
///
/// # Returns
/// A vector of potential location names found in the text
pub fn extract_potential_locations(output: &str) -> Vec<String> {
    let location_patterns = vec![
        "in the", "at the", "inside the", "outside the", "near the",
        "by the", "under the", "over the", "through the", "across the",
    ];

    let mut locations = Vec::new();
    let output_lower = output.to_lowercase();

    for pattern in location_patterns {
        if let Some(start) = output_lower.find(pattern) {
            let after_pattern = start + pattern.len();
            if let Some(location_end) = output_lower[after_pattern..].find(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
                let location = output_lower[after_pattern..after_pattern + location_end].trim().to_string();
                if !location.is_empty() && location.len() > 2 {
                    locations.push(location);
                }
            }
        }
    }

    locations.sort();
    locations.dedup();
    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_location_transition() {
        let output = "Harper left the attic and walked downstairs";
        let last_location = "attic";
        assert!(validate_location_transition(output, last_location));
    }

    #[test]
    fn test_invalid_location_transition() {
        let output = "Harper was suddenly in the kitchen";
        let last_location = "attic";
        assert!(!validate_location_transition(output, last_location));
    }

    #[test]
    fn test_case_insensitive_validation() {
        let output = "Harper EXITED THE ATTIC quickly";
        let last_location = "Attic";
        assert!(validate_location_transition(output, last_location));
    }

    #[test]
    fn test_multiple_indicators() {
        let test_cases = vec![
            ("Harper walked out of the attic", "attic", true),
            ("She stepped outside the attic door", "attic", true),
            ("He fled from the attic in terror", "attic", true),
            ("Harper ran from the dark attic", "attic", true),
            ("She emerged from the attic hatch", "attic", true),
        ];

        for (output, location, expected) in test_cases {
            assert_eq!(
                validate_location_transition(output, location),
                expected,
                "Failed for: '{}'",
                output
            );
        }
    }

    #[test]
    fn test_no_location_mention() {
        let output = "Harper left the building";
        let last_location = "attic";
        assert!(!validate_location_transition(output, last_location));
    }

    #[test]
    fn test_no_transition_indicator() {
        let output = "Harper was in the attic thinking";
        let last_location = "attic";
        assert!(!validate_location_transition(output, last_location));
    }

    #[test]
    fn test_implicit_transition_markers() {
        let test_cases = vec![
            ("Meanwhile, Harper was in the kitchen", true),
            ("Suddenly, the scene shifted to the garden", true),
            ("In a flashback, Harper remembered the attic", true),
            ("Later that day, Harper found herself downstairs", true),
            ("Harper continued thinking", false),
            ("The attic was dark and scary", false),
        ];

        for (output, expected) in test_cases {
            assert_eq!(
                has_implicit_transition_markers(output),
                expected,
                "Failed for: '{}'",
                output
            );
        }
    }

    #[test]
    fn test_comprehensive_validation() {
        // Should pass with explicit transition
        let output1 = "Harper left the attic";
        assert!(is_valid_location_transition(output1, "attic"));

        // Should pass with implicit transition
        let output2 = "Meanwhile, Harper was in the kitchen";
        assert!(is_valid_location_transition(output2, "attic"));

        // Should fail without either
        let output3 = "Harper was in the kitchen";
        assert!(!is_valid_location_transition(output3, "attic"));
    }

    #[test]
    fn test_extract_potential_locations() {
        let output = "Harper was in the kitchen near the window, then went to the garden";
        let locations = extract_potential_locations(output);

        assert!(locations.contains(&"kitchen".to_string()));
        assert!(locations.contains(&"window".to_string()));
        assert!(locations.contains(&"garden".to_string()));
    }

    #[test]
    fn test_extract_locations_complex() {
        let output = "She moved through the dark hallway, past the old library, into the bright study";
        let locations = extract_potential_locations(output);

        assert!(locations.len() >= 2);
        // Should find "dark", "old", "bright" or similar location qualifiers
    }

    #[test]
    fn test_empty_input() {
        assert!(!validate_location_transition("", "attic"));
        assert!(!has_implicit_transition_markers(""));
        assert!(extract_potential_locations("").is_empty());
    }

    #[test]
    fn test_location_edge_cases() {
        // Very short location names
        assert!(!validate_location_transition("left a", "a"));

        // Punctuation in location names
        let output = "Harper left the attic, quickly";
        assert!(validate_location_transition(output, "attic"));

        // Multiple same locations in text
        let output = "Harper left the attic to clean the attic later";
        assert!(validate_location_transition(output, "attic"));
    }
}