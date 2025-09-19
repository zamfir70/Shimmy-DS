/**
 * Entropy Calculation Helpers for AdaptIQ
 * =======================================
 *
 * Utilities for calculating content entropy, prompt complexity,
 * and other metrics that inform AdaptIQ decision making.
 */

use std::collections::HashMap;
use std::collections::HashSet;

/// Calculate prompt entropy based on token variety and complexity
pub fn entropy_score(prompt: &str) -> f32 {
    if prompt.trim().is_empty() {
        return 0.0;
    }

    let lexical_entropy = calculate_lexical_entropy(prompt);
    let structural_entropy = calculate_structural_entropy(prompt);
    let semantic_entropy = calculate_semantic_entropy(prompt);

    // Weighted combination of different entropy measures
    (lexical_entropy * 0.4 + structural_entropy * 0.3 + semantic_entropy * 0.3).clamp(0.0, 1.0)
}

/// Calculate lexical entropy based on word frequency distribution
fn calculate_lexical_entropy(text: &str) -> f32 {
    let words: Vec<&str> = text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();

    if words.is_empty() {
        return 0.0;
    }

    let mut word_counts: HashMap<&str, usize> = HashMap::new();
    for word in &words {
        *word_counts.entry(word.to_lowercase().as_str()).or_insert(0) += 1;
    }

    let total_words = words.len() as f32;
    let mut entropy = 0.0;

    for count in word_counts.values() {
        let probability = *count as f32 / total_words;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }

    // Normalize by theoretical maximum entropy (all words unique)
    let max_entropy = (total_words as f32).log2();
    if max_entropy > 0.0 {
        entropy / max_entropy
    } else {
        0.0
    }
}

/// Calculate structural entropy based on syntax variety
fn calculate_structural_entropy(text: &str) -> f32 {
    let mut structure_features = Vec::new();

    // Sentence length variety
    let sentences: Vec<&str> = text.split(&['.', '!', '?'][..]).collect();
    let sentence_lengths: Vec<usize> = sentences.iter()
        .map(|s| s.split_whitespace().count())
        .filter(|&len| len > 0)
        .collect();

    if !sentence_lengths.is_empty() {
        let avg_length = sentence_lengths.iter().sum::<usize>() as f32 / sentence_lengths.len() as f32;
        let length_variance = sentence_lengths.iter()
            .map(|&len| (len as f32 - avg_length).powi(2))
            .sum::<f32>() / sentence_lengths.len() as f32;
        structure_features.push((length_variance / 100.0).clamp(0.0, 1.0));
    }

    // Punctuation variety
    let punctuation_chars: HashSet<char> = text.chars()
        .filter(|c| c.is_ascii_punctuation())
        .collect();
    let punctuation_variety = (punctuation_chars.len() as f32 / 10.0).clamp(0.0, 1.0);
    structure_features.push(punctuation_variety);

    // Question complexity (multiple question words, nested questions)
    let question_words = ["what", "where", "when", "why", "how", "who", "which"];
    let question_count = question_words.iter()
        .map(|&qw| text.to_lowercase().matches(qw).count())
        .sum::<usize>();
    let question_complexity = (question_count as f32 / 5.0).clamp(0.0, 1.0);
    structure_features.push(question_complexity);

    // Calculate average of structural features
    if structure_features.is_empty() {
        0.0
    } else {
        structure_features.iter().sum::<f32>() / structure_features.len() as f32
    }
}

/// Calculate semantic entropy based on concept diversity
fn calculate_semantic_entropy(text: &str) -> f32 {
    let mut semantic_features = Vec::new();

    // Abstract vs concrete language ratio
    let abstract_indicators = ["concept", "idea", "notion", "theory", "principle", "essence",
                             "nature", "meaning", "purpose", "significance", "implication"];
    let concrete_indicators = ["object", "thing", "item", "place", "person", "action",
                             "event", "time", "location", "material", "physical"];

    let abstract_count = abstract_indicators.iter()
        .map(|&word| text.to_lowercase().matches(word).count())
        .sum::<usize>();
    let concrete_count = concrete_indicators.iter()
        .map(|&word| text.to_lowercase().matches(word).count())
        .sum::<usize>();

    let total_indicators = abstract_count + concrete_count;
    if total_indicators > 0 {
        let abstract_ratio = abstract_count as f32 / total_indicators as f32;
        // Higher entropy for balanced abstract/concrete mix
        let abstraction_entropy = 1.0 - (abstract_ratio - 0.5).abs() * 2.0;
        semantic_features.push(abstraction_entropy.clamp(0.0, 1.0));
    }

    // Emotional language diversity
    let emotional_words = ["love", "hate", "fear", "joy", "anger", "sadness", "surprise",
                          "disgust", "trust", "anticipation", "hope", "despair"];
    let emotion_count = emotional_words.iter()
        .map(|&word| text.to_lowercase().matches(word).count())
        .sum::<usize>();
    let emotion_diversity = (emotion_count as f32 / 6.0).clamp(0.0, 1.0);
    semantic_features.push(emotion_diversity);

    // Temporal complexity (past, present, future references)
    let temporal_words = ["was", "were", "had", "been", "is", "are", "am", "being",
                         "will", "shall", "going", "future", "past", "present"];
    let temporal_count = temporal_words.iter()
        .map(|&word| text.to_lowercase().matches(word).count())
        .sum::<usize>();
    let temporal_complexity = (temporal_count as f32 / 8.0).clamp(0.0, 1.0);
    semantic_features.push(temporal_complexity);

    // Calculate average of semantic features
    if semantic_features.is_empty() {
        0.0
    } else {
        semantic_features.iter().sum::<f32>() / semantic_features.len() as f32
    }
}

/// Analyze question complexity for prompt assessment
pub fn analyze_question_complexity(prompt: &str) -> QuestionComplexity {
    let text = prompt.to_lowercase();

    let mut complexity = QuestionComplexity::default();

    // Count question types
    if text.contains("what") { complexity.what_questions += 1; }
    if text.contains("why") { complexity.why_questions += 1; }
    if text.contains("how") { complexity.how_questions += 1; }
    if text.contains("when") { complexity.when_questions += 1; }
    if text.contains("where") { complexity.where_questions += 1; }
    if text.contains("who") { complexity.who_questions += 1; }

    // Detect nested questions
    let question_marks = text.matches('?').count();
    complexity.nested_questions = question_marks.saturating_sub(1);

    // Detect conditional questions
    if text.contains("if") || text.contains("suppose") || text.contains("imagine") {
        complexity.conditional_questions += 1;
    }

    // Detect comparative questions
    if text.contains("better") || text.contains("worse") || text.contains("compare")
       || text.contains("versus") || text.contains("vs") {
        complexity.comparative_questions += 1;
    }

    // Calculate overall complexity score
    complexity.overall_score = calculate_question_complexity_score(&complexity);

    complexity
}

/// Question complexity analysis result
#[derive(Debug, Default, Clone)]
pub struct QuestionComplexity {
    pub what_questions: usize,
    pub why_questions: usize,
    pub how_questions: usize,
    pub when_questions: usize,
    pub where_questions: usize,
    pub who_questions: usize,
    pub nested_questions: usize,
    pub conditional_questions: usize,
    pub comparative_questions: usize,
    pub overall_score: f32,
}

fn calculate_question_complexity_score(complexity: &QuestionComplexity) -> f32 {
    let mut score = 0.0;

    // Basic question types (weighted by complexity)
    score += complexity.what_questions as f32 * 0.3;  // Factual
    score += complexity.when_questions as f32 * 0.3;  // Temporal
    score += complexity.where_questions as f32 * 0.3; // Spatial
    score += complexity.who_questions as f32 * 0.4;   // Personal
    score += complexity.how_questions as f32 * 0.6;   // Process
    score += complexity.why_questions as f32 * 0.8;   // Causal

    // Advanced question types
    score += complexity.nested_questions as f32 * 0.5;
    score += complexity.conditional_questions as f32 * 0.7;
    score += complexity.comparative_questions as f32 * 0.6;

    // Normalize to 0-1 range
    (score / 10.0).clamp(0.0, 1.0)
}

/// Calculate content volatility based on emotional and conceptual shifts
pub fn calculate_content_volatility(text: &str) -> f32 {
    let sentences: Vec<&str> = text.split(&['.', '!', '?'][..])
        .filter(|s| !s.trim().is_empty())
        .collect();

    if sentences.len() < 2 {
        return 0.0;
    }

    let mut volatility_scores = Vec::new();

    // Calculate volatility between adjacent sentences
    for window in sentences.windows(2) {
        let sent1 = window[0];
        let sent2 = window[1];

        let emotional_shift = calculate_emotional_shift(sent1, sent2);
        let conceptual_shift = calculate_conceptual_shift(sent1, sent2);
        let structural_shift = calculate_structural_shift(sent1, sent2);

        let sentence_volatility = (emotional_shift + conceptual_shift + structural_shift) / 3.0;
        volatility_scores.push(sentence_volatility);
    }

    // Return average volatility
    volatility_scores.iter().sum::<f32>() / volatility_scores.len() as f32
}

fn calculate_emotional_shift(sent1: &str, sent2: &str) -> f32 {
    let positive_words = ["good", "great", "wonderful", "happy", "joy", "love", "excellent"];
    let negative_words = ["bad", "terrible", "awful", "sad", "hate", "fear", "horrible"];

    let score1 = calculate_emotional_score(sent1, &positive_words, &negative_words);
    let score2 = calculate_emotional_score(sent2, &positive_words, &negative_words);

    (score1 - score2).abs()
}

fn calculate_emotional_score(text: &str, positive_words: &[&str], negative_words: &[&str]) -> f32 {
    let text_lower = text.to_lowercase();
    let pos_count = positive_words.iter().map(|&word| text_lower.matches(word).count()).sum::<usize>();
    let neg_count = negative_words.iter().map(|&word| text_lower.matches(word).count()).sum::<usize>();

    let total = pos_count + neg_count;
    if total == 0 {
        0.0
    } else {
        (pos_count as f32 - neg_count as f32) / total as f32
    }
}

fn calculate_conceptual_shift(sent1: &str, sent2: &str) -> f32 {
    let words1: HashSet<&str> = sent1.split_whitespace().collect();
    let words2: HashSet<&str> = sent2.split_whitespace().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        1.0 - (intersection as f32 / union as f32)
    }
}

fn calculate_structural_shift(sent1: &str, sent2: &str) -> f32 {
    let len1 = sent1.split_whitespace().count();
    let len2 = sent2.split_whitespace().count();

    let max_len = len1.max(len2);
    if max_len == 0 {
        0.0
    } else {
        (len1 as f32 - len2 as f32).abs() / max_len as f32
    }
}

/// Estimate cognitive load required for processing given content
pub fn estimate_cognitive_load(text: &str) -> CognitiveLoad {
    let entropy = entropy_score(text);
    let question_complexity = analyze_question_complexity(text);
    let volatility = calculate_content_volatility(text);

    let reading_difficulty = estimate_reading_difficulty(text);
    let attention_demand = estimate_attention_demand(text);

    let overall_load = (entropy * 0.3 + question_complexity.overall_score * 0.3
                       + volatility * 0.2 + reading_difficulty * 0.1 + attention_demand * 0.1)
                       .clamp(0.0, 1.0);

    CognitiveLoad {
        entropy,
        question_complexity: question_complexity.overall_score,
        content_volatility: volatility,
        reading_difficulty,
        attention_demand,
        overall_load,
    }
}

/// Cognitive load assessment result
#[derive(Debug, Clone)]
pub struct CognitiveLoad {
    pub entropy: f32,
    pub question_complexity: f32,
    pub content_volatility: f32,
    pub reading_difficulty: f32,
    pub attention_demand: f32,
    pub overall_load: f32,
}

fn estimate_reading_difficulty(text: &str) -> f32 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return 0.0;
    }

    // Average word length
    let avg_word_length = words.iter()
        .map(|word| word.len())
        .sum::<usize>() as f32 / words.len() as f32;

    // Sentence length
    let sentences = text.split(&['.', '!', '?'][..]).count();
    let avg_sentence_length = if sentences > 0 {
        words.len() as f32 / sentences as f32
    } else {
        words.len() as f32
    };

    // Combine factors (simplified Flesch-like assessment)
    let difficulty = (avg_word_length / 10.0 + avg_sentence_length / 30.0).clamp(0.0, 1.0);
    difficulty
}

fn estimate_attention_demand(text: &str) -> f32 {
    let mut demand_factors = Vec::new();

    // Text length factor
    let word_count = text.split_whitespace().count();
    let length_demand = (word_count as f32 / 500.0).clamp(0.0, 1.0); // Normalize by ~500 words
    demand_factors.push(length_demand);

    // Complexity markers
    let complexity_markers = ["however", "therefore", "moreover", "nevertheless", "consequently"];
    let complexity_count = complexity_markers.iter()
        .map(|&marker| text.to_lowercase().matches(marker).count())
        .sum::<usize>();
    let complexity_demand = (complexity_count as f32 / 3.0).clamp(0.0, 1.0);
    demand_factors.push(complexity_demand);

    // Technical language
    let technical_indicators = ["system", "process", "method", "analysis", "implementation"];
    let technical_count = technical_indicators.iter()
        .map(|&term| text.to_lowercase().matches(term).count())
        .sum::<usize>();
    let technical_demand = (technical_count as f32 / 5.0).clamp(0.0, 1.0);
    demand_factors.push(technical_demand);

    // Average demand factors
    demand_factors.iter().sum::<f32>() / demand_factors.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_score_basic() {
        let simple_text = "Hello world hello world";
        let complex_text = "The multifaceted paradigm encompasses various intricate methodologies";
        let empty_text = "";

        let simple_entropy = entropy_score(simple_text);
        let complex_entropy = entropy_score(complex_text);
        let empty_entropy = entropy_score(empty_text);

        assert!(complex_entropy > simple_entropy);
        assert_eq!(empty_entropy, 0.0);
        assert!(simple_entropy >= 0.0 && simple_entropy <= 1.0);
        assert!(complex_entropy >= 0.0 && complex_entropy <= 1.0);
    }

    #[test]
    fn test_question_complexity() {
        let simple_question = "What is your name?";
        let complex_question = "Why do you think that if we consider the implications, how would this affect our understanding?";

        let simple_complexity = analyze_question_complexity(simple_question);
        let complex_complexity = analyze_question_complexity(complex_question);

        assert!(complex_complexity.overall_score > simple_complexity.overall_score);
        assert_eq!(simple_complexity.what_questions, 1);
        assert!(complex_complexity.why_questions > 0);
        assert!(complex_complexity.conditional_questions > 0);
    }

    #[test]
    fn test_content_volatility() {
        let stable_text = "I am happy. I am very happy. I am extremely happy.";
        let volatile_text = "I am happy. I am terribly sad. This is wonderful.";

        let stable_volatility = calculate_content_volatility(stable_text);
        let volatile_volatility = calculate_content_volatility(volatile_text);

        assert!(volatile_volatility > stable_volatility);
    }

    #[test]
    fn test_cognitive_load() {
        let simple_text = "Hello. How are you?";
        let complex_text = "The epistemological ramifications of postmodern hermeneutical methodologies necessitate a comprehensive reevaluation of our fundamental assumptions.";

        let simple_load = estimate_cognitive_load(simple_text);
        let complex_load = estimate_cognitive_load(complex_text);

        assert!(complex_load.overall_load > simple_load.overall_load);
        assert!(complex_load.reading_difficulty > simple_load.reading_difficulty);
        assert!(complex_load.entropy >= simple_load.entropy);
    }

    #[test]
    fn test_lexical_entropy() {
        let repeated = "test test test test";
        let varied = "different unique words everywhere";

        let repeated_entropy = calculate_lexical_entropy(repeated);
        let varied_entropy = calculate_lexical_entropy(varied);

        assert!(varied_entropy > repeated_entropy);
    }

    #[test]
    fn test_structural_entropy() {
        let simple_structure = "Short. Simple. Basic.";
        let complex_structure = "This is a longer, more complex sentence with various punctuation marks! Isn't it interesting?";

        let simple_structural = calculate_structural_entropy(simple_structure);
        let complex_structural = calculate_structural_entropy(complex_structure);

        assert!(complex_structural >= simple_structural);
    }
}