use super::engine::ModelSpec;
use crate::auto_discovery::{DiscoveredModel, ModelAutoDiscovery};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub name: String,
    pub base_path: PathBuf,
    pub lora_path: Option<PathBuf>,
    pub template: Option<String>,
    pub ctx_len: Option<usize>,
    pub n_threads: Option<i32>,
}

#[derive(Default, Clone)]
pub struct Registry {
    inner: HashMap<String, ModelEntry>,
    pub discovered_models: HashMap<String, DiscoveredModel>,
}

// Alias for backward compatibility and mission expectations
pub type ModelRegistry = Registry;

impl Registry {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            discovered_models: HashMap::new(),
        }
    }

    pub fn with_discovery() -> Self {
        let mut registry = Self::new();
        registry.refresh_discovered_models();
        registry
    }

    pub fn refresh_discovered_models(&mut self) {
        let discovery = ModelAutoDiscovery::new();
        if let Ok(models) = discovery.discover_models() {
            self.discovered_models.clear();
            for model in models {
                self.discovered_models.insert(model.name.clone(), model);
            }
        }
    }

    pub fn auto_register_discovered(&mut self) {
        // Convert discovered models to registry entries
        for (name, discovered) in &self.discovered_models {
            if !self.inner.contains_key(name) {
                let entry = ModelEntry {
                    name: name.clone(),
                    base_path: discovered.path.clone(),
                    lora_path: discovered.lora_path.clone(),
                    template: Some(self.infer_template(name)),
                    ctx_len: Some(self.infer_context_length(name)),
                    n_threads: None,
                };
                self.inner.insert(name.clone(), entry);
            }
        }
    }

    fn infer_template(&self, model_name: &str) -> String {
        let name_lower = model_name.to_lowercase();

        // Check model name patterns for better template detection
        if name_lower.contains("llama") {
            "llama3".to_string()
        } else if name_lower.contains("phi") {
            "chatml".to_string()
        } else if name_lower.contains("mistral") {
            "chatml".to_string()
        } else if name_lower.contains("qwen") {
            "chatml".to_string()
        } else if name_lower.contains("gemma") {
            "chatml".to_string()
        } else {
            "chatml".to_string() // Default to chatml for most models
        }
    }

    pub fn infer_context_length(&self, model_name: &str) -> usize {
        let name_lower = model_name.to_lowercase();

        // Check for high-context model indicators in name
        if name_lower.contains("128k") || name_lower.contains("131k") {
            131072 // 128k context
        } else if name_lower.contains("32k") || name_lower.contains("33k") {
            32768  // 32k context
        } else if name_lower.contains("16k") {
            16384  // 16k context
        } else if name_lower.contains("8k") {
            8192   // 8k context
        } else if name_lower.contains("phi-3") || name_lower.contains("phi3") {
            // Phi-3 models typically support 4k-128k
            if name_lower.contains("mini") && !name_lower.contains("128k") {
                4096  // Phi-3 Mini default
            } else {
                32768 // Assume larger context for other Phi-3 variants
            }
        } else if name_lower.contains("llama-3") || name_lower.contains("llama3") {
            // Llama 3 models support 8k+ context
            if name_lower.contains("8b") || name_lower.contains("70b") {
                8192  // Standard Llama 3 context
            } else {
                32768 // Larger variants may support more
            }
        } else if name_lower.contains("qwen") {
            // Qwen models often support large context
            32768
        } else if name_lower.contains("mistral") {
            // Mistral models typically 32k+
            32768
        } else {
            // Default to 16k for long-form writing (up from 4k)
            16384
        }
    }

    pub fn register(&mut self, e: ModelEntry) {
        self.inner.insert(e.name.clone(), e);
    }
    pub fn get(&self, name: &str) -> Option<&ModelEntry> {
        // First check manually registered models, then auto-discovered
        self.inner.get(name)
    }
    pub fn list(&self) -> Vec<&ModelEntry> {
        self.inner.values().collect()
    }
    pub fn list_all_available(&self) -> Vec<String> {
        let mut available = Vec::new();
        available.extend(self.inner.keys().cloned());
        available.extend(self.discovered_models.keys().cloned());
        available.sort();
        available.dedup();
        available
    }

    pub fn to_spec(&self, name: &str) -> Option<ModelSpec> {
        // Try manually registered first
        if let Some(e) = self.inner.get(name) {
            return Some(ModelSpec {
                name: e.name.clone(),
                base_path: e.base_path.clone(),
                lora_path: e.lora_path.clone(),
                template: e.template.clone(),
                ctx_len: e.ctx_len.unwrap_or(4096),
                n_threads: e.n_threads,
            });
        }

        // Fall back to discovered models
        if let Some(discovered) = self.discovered_models.get(name) {
            return Some(ModelSpec {
                name: discovered.name.clone(),
                base_path: discovered.path.clone(),
                lora_path: discovered.lora_path.clone(),
                template: Some(self.infer_template(&discovered.name)),
                ctx_len: 4096,
                n_threads: None,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = Registry::new();
        assert!(registry.inner.is_empty());
        assert!(registry.discovered_models.is_empty());
    }

    #[test]
    fn test_registry_default() {
        let registry = Registry::default();
        assert!(registry.inner.is_empty());
    }

    #[test]
    fn test_register_model() {
        let mut registry = Registry::new();
        let entry = ModelEntry {
            name: "test-model".to_string(),
            base_path: PathBuf::from("/path/to/model"),
            lora_path: None,
            template: Some("chatml".to_string()),
            ctx_len: Some(4096),
            n_threads: Some(4),
        };

        registry.register(entry.clone());
        assert_eq!(registry.inner.len(), 1);
        assert!(registry.get("test-model").is_some());
    }

    #[test]
    fn test_list_models() {
        let mut registry = Registry::new();
        let entry = ModelEntry {
            name: "test".to_string(),
            base_path: PathBuf::from("/test"),
            lora_path: None,
            template: None,
            ctx_len: None,
            n_threads: None,
        };

        registry.register(entry);
        let models = registry.list();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "test");
    }
}
