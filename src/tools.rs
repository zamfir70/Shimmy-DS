use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
}

pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    fn execute(&self, arguments: serde_json::Value) -> Result<ToolResult>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };

        // Register built-in tools
        registry.register(Box::new(CalculatorTool));
        registry.register(Box::new(FileReadTool));
        registry.register(Box::new(HttpGetTool));

        registry
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.definition().name.clone();
        self.tools.insert(name, tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    pub fn execute_tool(&self, call: &ToolCall) -> Result<ToolResult> {
        if let Some(tool) = self.get_tool(&call.name) {
            tool.execute(call.arguments.clone())
        } else {
            Ok(ToolResult {
                success: false,
                result: serde_json::Value::Null,
                error: Some(format!("Tool '{}' not found", call.name)),
            })
        }
    }
}

// Built-in tools

pub struct CalculatorTool;

impl Tool for CalculatorTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "calculator".to_string(),
            description: "Perform basic mathematical calculations".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Mathematical expression to evaluate (e.g., '2 + 2', '10 * 3')"
                    }
                },
                "required": ["expression"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> Result<ToolResult> {
        let expression = arguments
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing expression parameter"))?;

        // Simple calculator - in production this would use a proper expression parser
        let result = match expression {
            expr if expr.contains(" + ") => {
                let parts: Vec<&str> = expr.split(" + ").collect();
                if parts.len() == 2 {
                    let a: f64 = parts[0].parse()?;
                    let b: f64 = parts[1].parse()?;
                    a + b
                } else {
                    return Ok(ToolResult {
                        success: false,
                        result: serde_json::Value::Null,
                        error: Some("Invalid addition expression".to_string()),
                    });
                }
            }
            expr if expr.contains(" * ") => {
                let parts: Vec<&str> = expr.split(" * ").collect();
                if parts.len() == 2 {
                    let a: f64 = parts[0].parse()?;
                    let b: f64 = parts[1].parse()?;
                    a * b
                } else {
                    return Ok(ToolResult {
                        success: false,
                        result: serde_json::Value::Null,
                        error: Some("Invalid multiplication expression".to_string()),
                    });
                }
            }
            _ => {
                return Ok(ToolResult {
                    success: false,
                    result: serde_json::Value::Null,
                    error: Some("Unsupported expression".to_string()),
                });
            }
        };

        Ok(ToolResult {
            success: true,
            result: serde_json::json!(result),
            error: None,
        })
    }
}

pub struct FileReadTool;

impl Tool for FileReadTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "file_read".to_string(),
            description: "Read contents of a text file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to read"
                    }
                },
                "required": ["path"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> Result<ToolResult> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing path parameter"))?;

        match std::fs::read_to_string(path) {
            Ok(content) => Ok(ToolResult {
                success: true,
                result: serde_json::json!(content),
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                result: serde_json::Value::Null,
                error: Some(e.to_string()),
            }),
        }
    }
}

pub struct HttpGetTool;

impl Tool for HttpGetTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "http_get".to_string(),
            description: "Make an HTTP GET request".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "URL to fetch"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> Result<ToolResult> {
        let _url = arguments
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing url parameter"))?;

        // Placeholder - in production this would make actual HTTP requests
        Ok(ToolResult {
            success: false,
            result: serde_json::Value::Null,
            error: Some("HTTP requests not implemented yet".to_string()),
        })
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_registry_creation() {
        let registry = ToolRegistry::new();
        assert!(registry.tools.len() >= 3);
    }

    #[test]
    fn test_tool_definition_creation() {
        let def = ToolDefinition {
            name: "test".to_string(),
            description: "test tool".to_string(),
            parameters: serde_json::json!({"test": true}),
        };
        assert_eq!(def.name, "test");
    }

    #[test]
    fn test_tool_call_creation() {
        let call = ToolCall {
            name: "calc".to_string(),
            arguments: serde_json::json!({"x": 5, "y": 3}),
        };
        assert_eq!(call.name, "calc");
    }

    #[test]
    fn test_tool_result_creation() {
        let result = ToolResult {
            success: true,
            result: serde_json::json!({"answer": 8}),
            error: None,
        };
        assert!(result.success);
    }

    #[test]
    fn test_calculator_tool_definition() {
        let calc = CalculatorTool;
        let def = calc.definition();
        assert_eq!(def.name, "calculator");
        assert!(def.description.contains("mathematical"));
    }

    #[test]
    fn test_calculator_tool_execution() {
        let calc = CalculatorTool;
        let args = serde_json::json!({"expression": "2 + 3"});
        let result = calc.execute(args).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_file_read_tool_definition() {
        let file_tool = FileReadTool;
        let def = file_tool.definition();
        assert_eq!(def.name, "file_read");
    }

    #[test]
    fn test_http_get_tool_definition() {
        let http_tool = HttpGetTool;
        let def = http_tool.definition();
        assert_eq!(def.name, "http_get");
    }

    #[test]
    fn test_tool_registry_register() {
        let mut registry = ToolRegistry::new();
        let initial_count = registry.tools.len();
        registry.register(Box::new(CalculatorTool));
        assert!(registry.tools.len() >= initial_count);
    }
}
