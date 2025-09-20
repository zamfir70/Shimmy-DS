/// 🔧 PHASE 1: Obligation Injection Hook
///
/// Allows shimmy-DS to prepend current unresolved obligations to the prompt
/// before sending to the model.
///
/// This module provides the core functionality for injecting narrative obligations
/// into user prompts to ensure story continuity and narrative coherence.
/// Now enhanced with ObliSelect smart obligation management for intelligent selection.

use crate::obligations::{SmartObligationManager, ObligationScore};

/// Injects obligations into a prompt by prepending each obligation as a statement.
///
/// # Arguments
/// * `prompt` - The original user prompt
/// * `obligations` - A slice of obligation strings to inject
///
/// # Returns
/// A new string with obligations prepended to the original prompt
///
/// # Example
/// ```
/// use shimmy_ds::prompt_injector::inject_obligations;
///
/// let prompt = "What happens next?";
/// let obligations = vec!["Harper was last seen in the attic".to_string()];
/// let injected = inject_obligations(prompt, &obligations);
///
/// assert!(injected.starts_with("Obligation: Harper was last seen in the attic."));
/// assert!(injected.ends_with("What happens next?"));
/// ```
pub fn inject_obligations(prompt: &str, obligations: &[String]) -> String {
    let mut injected = String::new();

    // Prepend each obligation as a statement
    for obligation in obligations {
        injected.push_str(&format!("Obligation: {}.\n", obligation));
    }

    // Add the original prompt
    injected.push_str(prompt);
    injected
}

/// Loads obligations using the ObliSelect smart obligation management system.
///
/// This function interfaces with the SmartObligationManager to intelligently
/// select the most relevant obligations based on current narrative context,
/// urgency, freshness, and other scoring factors.
///
/// # Arguments
/// * `obligation_manager` - Reference to the SmartObligationManager
/// * `max_obligations` - Optional maximum number of obligations to select
///
/// # Returns
/// A vector of selected obligation content strings
pub fn load_smart_obligations(
    obligation_manager: &mut SmartObligationManager,
    max_obligations: Option<usize>
) -> Vec<String> {
    let selected_scores = obligation_manager.select_obligations(max_obligations);

    selected_scores.into_iter()
        .filter_map(|score| {
            obligation_manager.get_all_obligations()
                .get(&score.obligation_id)
                .map(|obligation| obligation.content.clone())
        })
        .collect()
}

/// Loads obligations with detailed scoring information for analysis and debugging.
///
/// # Arguments
/// * `obligation_manager` - Reference to the SmartObligationManager
/// * `max_obligations` - Optional maximum number of obligations to select
///
/// # Returns
/// A vector of ObligationScore structs with detailed scoring information
pub fn load_smart_obligations_with_scores(
    obligation_manager: &mut SmartObligationManager,
    max_obligations: Option<usize>
) -> Vec<ObligationScore> {
    obligation_manager.select_obligations(max_obligations)
}

/// Legacy stub function to load obligations from persistent state.
///
/// In a full implementation, this would read from a file, database, or other
/// persistent storage mechanism to retrieve the current list of unresolved
/// narrative obligations.
///
/// # Returns
/// A vector of obligation strings
pub fn load_obligations() -> Vec<String> {
    vec!["Harper was last seen in the attic".to_string()]
}

/// Enhanced obligation injection that uses ObliSelect for intelligent selection.
///
/// This function combines the traditional obligation injection with smart
/// obligation management, selecting the most contextually relevant obligations
/// based on narrative state and scoring algorithms.
///
/// # Arguments
/// * `prompt` - The original user prompt
/// * `obligation_manager` - Mutable reference to the SmartObligationManager
/// * `max_obligations` - Optional maximum number of obligations to inject
///
/// # Returns
/// A new string with intelligently selected obligations prepended to the original prompt
pub fn inject_smart_obligations(
    prompt: &str,
    obligation_manager: &mut SmartObligationManager,
    max_obligations: Option<usize>
) -> String {
    let selected_obligations = load_smart_obligations(obligation_manager, max_obligations);
    inject_obligations(prompt, &selected_obligations)
}

/// Enhanced obligation injection with scoring details for debugging and analysis.
///
/// # Arguments
/// * `prompt` - The original user prompt
/// * `obligation_manager` - Mutable reference to the SmartObligationManager
/// * `max_obligations` - Optional maximum number of obligations to inject
///
/// # Returns
/// A tuple containing the injected prompt string and the detailed scoring information
pub fn inject_smart_obligations_with_details(
    prompt: &str,
    obligation_manager: &mut SmartObligationManager,
    max_obligations: Option<usize>
) -> (String, Vec<ObligationScore>) {
    let obligation_scores = load_smart_obligations_with_scores(obligation_manager, max_obligations);

    let selected_obligations: Vec<String> = obligation_scores.iter()
        .filter_map(|score| {
            obligation_manager.get_all_obligations()
                .get(&score.obligation_id)
                .map(|obligation| obligation.content.clone())
        })
        .collect();

    let injected_prompt = inject_obligations(prompt, &selected_obligations);
    (injected_prompt, obligation_scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_obligations_empty() {
        let prompt = "Continue the story";
        let obligations = vec![];
        let result = inject_obligations(prompt, &obligations);
        assert_eq!(result, "Continue the story");
    }

    #[test]
    fn test_inject_obligations_single() {
        let prompt = "What happens next?";
        let obligations = vec!["Harper was last seen in the attic".to_string()];
        let result = inject_obligations(prompt, &obligations);

        let expected = "Obligation: Harper was last seen in the attic.\nWhat happens next?";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_inject_obligations_multiple() {
        let prompt = "Continue";
        let obligations = vec![
            "Harper was last seen in the attic".to_string(),
            "The door was left unlocked".to_string(),
            "Strange sounds came from upstairs".to_string(),
        ];
        let result = inject_obligations(prompt, &obligations);

        let expected = "Obligation: Harper was last seen in the attic.\n\
                       Obligation: The door was left unlocked.\n\
                       Obligation: Strange sounds came from upstairs.\n\
                       Continue";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_load_obligations_stub() {
        let obligations = load_obligations();
        assert_eq!(obligations.len(), 1);
        assert_eq!(obligations[0], "Harper was last seen in the attic");
    }

    #[test]
    fn test_inject_obligations_preserves_prompt_formatting() {
        let prompt = "Multi-line\nprompt\nwith formatting";
        let obligations = vec!["Test obligation".to_string()];
        let result = inject_obligations(prompt, &obligations);

        assert!(result.ends_with("Multi-line\nprompt\nwith formatting"));
        assert!(result.starts_with("Obligation: Test obligation.\n"));
    }

    #[test]
    fn test_inject_obligations_empty_prompt() {
        let prompt = "";
        let obligations = vec!["Test obligation".to_string()];
        let result = inject_obligations(prompt, &obligations);

        assert_eq!(result, "Obligation: Test obligation.\n");
    }

    #[test]
    fn test_inject_obligations_special_characters() {
        let prompt = "What about the mysterious symbols: @#$%?";
        let obligations = vec!["The rune ᚾ glowed dimly".to_string()];
        let result = inject_obligations(prompt, &obligations);

        assert!(result.contains("Obligation: The rune ᚾ glowed dimly."));
        assert!(result.contains("What about the mysterious symbols: @#$%?"));
    }
}