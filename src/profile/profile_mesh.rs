// src/profile/profile_mesh.rs
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteVector {
    pub curiosity: f32,
    pub coherence_pleasure: f32,
    pub delight: f32,
    pub awe: f32,
    pub unease: f32,
    pub boredom: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MistakePattern {
    pub description: String,
    pub frequency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMesh {
    pub taste: TasteVector,
    pub preferred_complexity: f32,
    pub common_mistakes: Vec<MistakePattern>,
}

impl Default for TasteVector {
    fn default() -> Self {
        Self {
            curiosity: 0.05,
            coherence_pleasure: 0.05,
            delight: 0.0,
            awe: 0.0,
            unease: 0.0,
            boredom: 0.0,
        }
    }
}

impl ProfileMesh {
    pub fn default() -> Self {
        Self {
            taste: TasteVector::default(),
            preferred_complexity: 0.5,
            common_mistakes: vec![],
        }
    }

    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_else(|_| Self::default())
        } else {
            Self::default()
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn update_taste(&mut self, delta: TasteVector) {
        self.taste.curiosity += delta.curiosity;
        self.taste.coherence_pleasure += delta.coherence_pleasure;
        self.taste.delight += delta.delight;
        self.taste.awe += delta.awe;
        self.taste.unease += delta.unease;
        self.taste.boredom += delta.boredom;

        // Clamp values to prevent extreme drift
        self.taste.curiosity = self.taste.curiosity.clamp(-1.0, 1.0);
        self.taste.coherence_pleasure = self.taste.coherence_pleasure.clamp(-1.0, 1.0);
        self.taste.delight = self.taste.delight.clamp(-1.0, 1.0);
        self.taste.awe = self.taste.awe.clamp(-1.0, 1.0);
        self.taste.unease = self.taste.unease.clamp(-1.0, 1.0);
        self.taste.boredom = self.taste.boredom.clamp(-1.0, 1.0);
    }

    pub fn register_mistake(&mut self, desc: &str) {
        if let Some(p) = self.common_mistakes.iter_mut().find(|m| m.description == desc) {
            p.frequency += 1;
        } else {
            self.common_mistakes.push(MistakePattern {
                description: desc.to_string(),
                frequency: 1,
            });
        }

        // Keep only top 20 most frequent mistakes to prevent bloat
        self.common_mistakes.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        self.common_mistakes.truncate(20);
    }

    pub fn get_mistake_weight(&self, desc: &str) -> f32 {
        self.common_mistakes
            .iter()
            .find(|m| m.description == desc)
            .map(|m| m.frequency as f32 / 100.0) // Normalize frequency to weight
            .unwrap_or(0.0)
    }

    pub fn complexity_bias(&self) -> f32 {
        // Return complexity preference as bias for AdaptIQ
        (self.preferred_complexity - 0.5) * 2.0 // Scale to [-1, 1]
    }
}