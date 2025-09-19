/// 🎭 PHASE 4: Emotional Resonance Hook
///
/// Adds emotional field injection from character resonance.
/// This module provides functionality to inject emotional context into prompts
/// to maintain emotional continuity and enhance narrative depth.

/// Represents an emotional state with intensity
#[derive(Debug, Clone, PartialEq)]
pub struct EmotionalState {
    /// The primary emotion (e.g., "joy", "fear", "anger", "sadness")
    pub emotion: String,
    /// Intensity from 0.0 (barely perceptible) to 1.0 (overwhelming)
    pub intensity: f32,
    /// Optional secondary emotion that might be mixed in
    pub secondary_emotion: Option<String>,
    /// Secondary emotion intensity (if present)
    pub secondary_intensity: f32,
}

impl EmotionalState {
    /// Creates a new emotional state with primary emotion and intensity
    pub fn new(emotion: impl Into<String>, intensity: f32) -> Self {
        Self {
            emotion: emotion.into(),
            intensity: intensity.clamp(0.0, 1.0),
            secondary_emotion: None,
            secondary_intensity: 0.0,
        }
    }

    /// Creates an emotional state with both primary and secondary emotions
    pub fn with_secondary(
        emotion: impl Into<String>,
        intensity: f32,
        secondary: impl Into<String>,
        secondary_intensity: f32,
    ) -> Self {
        Self {
            emotion: emotion.into(),
            intensity: intensity.clamp(0.0, 1.0),
            secondary_emotion: Some(secondary.into()),
            secondary_intensity: secondary_intensity.clamp(0.0, 1.0),
        }
    }

    /// Gets the dominant emotion (highest intensity)
    pub fn dominant_emotion(&self) -> &str {
        if let Some(ref secondary) = self.secondary_emotion {
            if self.secondary_intensity > self.intensity {
                secondary
            } else {
                &self.emotion
            }
        } else {
            &self.emotion
        }
    }

    /// Gets the combined emotional intensity
    pub fn total_intensity(&self) -> f32 {
        (self.intensity + self.secondary_intensity).min(1.0)
    }

    /// Describes the emotional state as a human-readable string
    pub fn describe(&self) -> String {
        if let Some(ref secondary) = self.secondary_emotion {
            if self.secondary_intensity > 0.1 {
                format!(
                    "{} ({:.1}) mixed with {} ({:.1})",
                    self.emotion, self.intensity, secondary, self.secondary_intensity
                )
            } else {
                format!("{} ({:.1})", self.emotion, self.intensity)
            }
        } else {
            format!("{} ({:.1})", self.emotion, self.intensity)
        }
    }
}

/// Injects emotional context into a prompt to guide generation
///
/// # Arguments
/// * `prompt` - The original prompt text
/// * `emotion` - The emotional state name (e.g., "guilt", "joy", "fear")
/// * `intensity` - Emotional intensity from 0.0 to 1.0
///
/// # Returns
/// A new string with emotional context prepended to the original prompt
///
/// # Example
/// ```
/// use shimmy_ds::emotion_resonance::inject_emotion;
///
/// let prompt = "What happens next?";
/// let emotional_prompt = inject_emotion(prompt, "guilt", 0.8);
/// assert!(emotional_prompt.contains("guilt"));
/// assert!(emotional_prompt.contains("0.80"));
/// ```
pub fn inject_emotion(prompt: &str, emotion: &str, intensity: f32) -> String {
    let clamped_intensity = intensity.clamp(0.0, 1.0);
    let intensity_descriptor = match clamped_intensity {
        i if i >= 0.9 => "overwhelming",
        i if i >= 0.7 => "intense",
        i if i >= 0.5 => "moderate",
        i if i >= 0.3 => "subtle",
        _ => "faint",
    };

    let mut capsule = format!(
        "Current emotion field: {} (intensity {:.2}, {})\n",
        emotion, clamped_intensity, intensity_descriptor
    );
    capsule.push_str(prompt);
    capsule
}

/// Injects complex emotional state into a prompt
///
/// # Arguments
/// * `prompt` - The original prompt text
/// * `emotional_state` - The complete emotional state to inject
///
/// # Returns
/// A new string with detailed emotional context
pub fn inject_emotional_state(prompt: &str, emotional_state: &EmotionalState) -> String {
    let description = emotional_state.describe();
    let dominant = emotional_state.dominant_emotion();
    let total_intensity = emotional_state.total_intensity();

    let atmosphere_descriptor = match total_intensity {
        i if i >= 0.8 => "charged with emotional tension",
        i if i >= 0.6 => "emotionally intense",
        i if i >= 0.4 => "emotionally present",
        i if i >= 0.2 => "with subtle emotional undertones",
        _ => "emotionally neutral",
    };

    let mut capsule = format!(
        "Emotional field: {} - atmosphere is {}\n",
        description, atmosphere_descriptor
    );

    // Add specific guidance based on dominant emotion
    match dominant {
        "guilt" => capsule.push_str("Note: Actions and dialogue should reflect weight of conscience.\n"),
        "fear" => capsule.push_str("Note: Heightened awareness, tension in descriptions.\n"),
        "joy" => capsule.push_str("Note: Lightness in tone, optimistic possibilities.\n"),
        "anger" => capsule.push_str("Note: Sharp edges in language, potential for conflict.\n"),
        "sadness" => capsule.push_str("Note: Melancholic undertones, introspective moments.\n"),
        "love" => capsule.push_str("Note: Warmth and connection in interactions.\n"),
        "curiosity" => capsule.push_str("Note: Exploratory tendency, question-driven narrative.\n"),
        _ => capsule.push_str("Note: Let emotion subtly influence tone and choices.\n"),
    }

    capsule.push_str(prompt);
    capsule
}

/// Analyzes text to detect emotional indicators
///
/// # Arguments
/// * `text` - The text to analyze for emotional content
///
/// # Returns
/// A vector of detected emotions with confidence scores
pub fn detect_emotions(text: &str) -> Vec<(String, f32)> {
    let emotion_keywords = vec![
        ("anger", vec!["angry", "furious", "rage", "mad", "irritated", "frustrated"]),
        ("fear", vec!["afraid", "scared", "terrified", "anxious", "worried", "nervous"]),
        ("joy", vec!["happy", "joyful", "delighted", "pleased", "cheerful", "glad"]),
        ("sadness", vec!["sad", "depressed", "melancholy", "sorrowful", "gloomy", "dejected"]),
        ("guilt", vec!["guilty", "ashamed", "regretful", "remorseful", "sorry"]),
        ("love", vec!["love", "affection", "adore", "cherish", "tender", "caring"]),
        ("curiosity", vec!["curious", "wondering", "intrigued", "puzzled", "interested"]),
        ("surprise", vec!["surprised", "astonished", "amazed", "shocked", "startled"]),
    ];

    let text_lower = text.to_lowercase();
    let mut detected = Vec::new();

    for (emotion, keywords) in emotion_keywords {
        let mut matches = 0;
        let total_keywords = keywords.len();

        for keyword in keywords {
            if text_lower.contains(keyword) {
                matches += 1;
            }
        }

        if matches > 0 {
            let confidence = matches as f32 / total_keywords as f32;
            detected.push((emotion.to_string(), confidence));
        }
    }

    // Sort by confidence (highest first)
    detected.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    detected
}

/// Creates an emotional resonance report for analysis
///
/// # Arguments
/// * `text` - The text to analyze
/// * `current_state` - Optional current emotional state for comparison
///
/// # Returns
/// A formatted report string
pub fn generate_emotion_report(text: &str, current_state: Option<&EmotionalState>) -> String {
    let detected = detect_emotions(text);
    let mut report = String::new();

    report.push_str("🎭 EMOTIONAL RESONANCE REPORT\n");
    report.push_str("==============================\n");

    if let Some(state) = current_state {
        report.push_str(&format!("Current State: {}\n", state.describe()));
        report.push_str(&format!("Dominant Emotion: {}\n", state.dominant_emotion()));
        report.push_str(&format!("Total Intensity: {:.2}\n\n", state.total_intensity()));
    }

    if detected.is_empty() {
        report.push_str("No strong emotional indicators detected in text.\n");
    } else {
        report.push_str("Detected Emotions:\n");
        for (emotion, confidence) in detected {
            let strength = match confidence {
                c if c >= 0.7 => "Strong",
                c if c >= 0.4 => "Moderate",
                c if c >= 0.2 => "Weak",
                _ => "Minimal",
            };
            report.push_str(&format!("  • {} ({} - {:.2})\n", emotion, strength, confidence));
        }
    }

    report
}

/// Suggests emotional adjustments based on narrative context
///
/// # Arguments
/// * `current_emotion` - Current emotional state
/// * `narrative_pressure` - Pressure from obligation system (0.0 to 3.0+)
///
/// # Returns
/// Suggested emotional modifications
pub fn suggest_emotional_adjustments(current_emotion: &EmotionalState, narrative_pressure: f32) -> String {
    let base_intensity = current_emotion.total_intensity();

    match narrative_pressure {
        p if p >= 2.5 => {
            format!("🚨 HIGH PRESSURE: Consider amplifying emotional intensity to {:.2} and introducing stress-related emotions",
                   (base_intensity + 0.3).min(1.0))
        }
        p if p >= 1.5 => {
            format!("⚡ ELEVATED: Slight emotional intensification recommended to {:.2}",
                   (base_intensity + 0.2).min(1.0))
        }
        p if p >= 0.8 => {
            "📊 STABLE: Current emotional state is appropriate for narrative pressure".to_string()
        }
        _ => {
            format!("🔄 LOW PRESSURE: Consider deepening emotional exploration or introducing new emotional elements")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_state_creation() {
        let state = EmotionalState::new("joy", 0.8);
        assert_eq!(state.emotion, "joy");
        assert_eq!(state.intensity, 0.8);
        assert!(state.secondary_emotion.is_none());
    }

    #[test]
    fn test_emotional_state_clamping() {
        let state = EmotionalState::new("anger", 1.5);
        assert_eq!(state.intensity, 1.0);

        let state2 = EmotionalState::new("sadness", -0.5);
        assert_eq!(state2.intensity, 0.0);
    }

    #[test]
    fn test_emotional_state_with_secondary() {
        let state = EmotionalState::with_secondary("guilt", 0.7, "fear", 0.3);
        assert_eq!(state.emotion, "guilt");
        assert_eq!(state.intensity, 0.7);
        assert_eq!(state.secondary_emotion, Some("fear".to_string()));
        assert_eq!(state.secondary_intensity, 0.3);
    }

    #[test]
    fn test_dominant_emotion() {
        let state1 = EmotionalState::with_secondary("guilt", 0.7, "fear", 0.3);
        assert_eq!(state1.dominant_emotion(), "guilt");

        let state2 = EmotionalState::with_secondary("guilt", 0.3, "fear", 0.7);
        assert_eq!(state2.dominant_emotion(), "fear");
    }

    #[test]
    fn test_total_intensity() {
        let state = EmotionalState::with_secondary("guilt", 0.6, "fear", 0.3);
        assert_eq!(state.total_intensity(), 0.9);

        // Test clamping
        let state2 = EmotionalState::with_secondary("anger", 0.8, "frustration", 0.7);
        assert_eq!(state2.total_intensity(), 1.0);
    }

    #[test]
    fn test_inject_emotion_basic() {
        let prompt = "What happens next?";
        let result = inject_emotion(prompt, "guilt", 0.8);

        assert!(result.contains("guilt"));
        assert!(result.contains("0.80"));
        assert!(result.contains("intense"));
        assert!(result.ends_with("What happens next?"));
    }

    #[test]
    fn test_inject_emotion_intensity_descriptors() {
        let test_cases = vec![
            (0.95, "overwhelming"),
            (0.75, "intense"),
            (0.55, "moderate"),
            (0.35, "subtle"),
            (0.15, "faint"),
        ];

        for (intensity, expected_descriptor) in test_cases {
            let result = inject_emotion("test", "test", intensity);
            assert!(result.contains(expected_descriptor));
        }
    }

    #[test]
    fn test_inject_emotional_state() {
        let state = EmotionalState::with_secondary("guilt", 0.8, "fear", 0.2);
        let result = inject_emotional_state("Continue the story", &state);

        assert!(result.contains("guilt"));
        assert!(result.contains("fear"));
        assert!(result.contains("emotionally intense"));
        assert!(result.contains("weight of conscience"));
    }

    #[test]
    fn test_detect_emotions() {
        let text = "Harper was angry and afraid, feeling guilty about what happened";
        let detected = detect_emotions(text);

        assert!(!detected.is_empty());

        let emotion_names: Vec<&str> = detected.iter().map(|(name, _)| name.as_str()).collect();
        assert!(emotion_names.contains(&"anger"));
        assert!(emotion_names.contains(&"fear"));
        assert!(emotion_names.contains(&"guilt"));
    }

    #[test]
    fn test_detect_emotions_empty() {
        let text = "The weather is nice today";
        let detected = detect_emotions(text);

        // Should have few or no strong emotional indicators
        assert!(detected.is_empty() || detected.iter().all(|(_, confidence)| *confidence < 0.3));
    }

    #[test]
    fn test_generate_emotion_report() {
        let state = EmotionalState::new("guilt", 0.8);
        let text = "Harper felt angry and scared";
        let report = generate_emotion_report(text, Some(&state));

        assert!(report.contains("EMOTIONAL RESONANCE REPORT"));
        assert!(report.contains("Current State: guilt"));
        assert!(report.contains("Dominant Emotion: guilt"));
        assert!(report.contains("anger"));
        assert!(report.contains("fear"));
    }

    #[test]
    fn test_suggest_emotional_adjustments() {
        let state = EmotionalState::new("guilt", 0.5);

        let high_pressure = suggest_emotional_adjustments(&state, 3.0);
        assert!(high_pressure.contains("HIGH PRESSURE"));
        assert!(high_pressure.contains("amplifying"));

        let low_pressure = suggest_emotional_adjustments(&state, 0.5);
        assert!(low_pressure.contains("LOW PRESSURE"));

        let stable = suggest_emotional_adjustments(&state, 1.0);
        assert!(stable.contains("STABLE"));
    }

    #[test]
    fn test_emotional_state_describe() {
        let simple = EmotionalState::new("joy", 0.8);
        assert_eq!(simple.describe(), "joy (0.8)");

        let complex = EmotionalState::with_secondary("guilt", 0.6, "fear", 0.3);
        assert!(complex.describe().contains("guilt (0.6) mixed with fear (0.3)"));

        let minimal_secondary = EmotionalState::with_secondary("joy", 0.8, "surprise", 0.05);
        assert_eq!(minimal_secondary.describe(), "joy (0.8)");
    }

    #[test]
    fn test_emotion_specific_guidance() {
        let emotions = vec!["guilt", "fear", "joy", "anger", "sadness", "love", "curiosity"];

        for emotion in emotions {
            let state = EmotionalState::new(emotion, 0.7);
            let result = inject_emotional_state("test", &state);
            assert!(result.contains("Note:"));
        }
    }
}