/// 🔧 PHASE 1: Obligation Injection Hook
///
/// Allows shimmy-DS to prepend current unresolved obligations to the prompt
/// before sending to the model.
///
/// This module provides the core functionality for injecting narrative obligations
/// into user prompts to ensure story continuity and narrative coherence.

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

/// Stub function to load obligations from persistent state.
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