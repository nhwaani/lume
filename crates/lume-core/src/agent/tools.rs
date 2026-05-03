use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml_edit::{value, DocumentMut};

#[derive(Deserialize, Serialize, Debug)]
pub struct ToolCall {
    pub tool: String,
    pub arguments: serde_json::Value,
}

pub struct ToolBridge;

impl ToolBridge {
    /// Modifies a value in the config.toml file
    pub fn modify_config(key_path: &str, new_value: &str) -> Result<String, String> {
        let path = Path::new("config.toml");
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;

        let mut doc = content
            .parse::<DocumentMut>()
            .map_err(|e| format!("Failed to parse TOML: {}", e))?;

        // Split "theme.accent" into ["theme", "accent"]
        let parts: Vec<&str> = key_path.split('.').collect();
        if parts.len() != 2 {
            return Err(
                "Key path must be in 'section.key' format (e.g., theme.accent)".to_string(),
            );
        }

        let section = parts[0];
        let key = parts[1];

        // Update the value in the TOML document
        doc[section][key] = value(new_value);

        fs::write(path, doc.to_string()).map_err(|e| format!("Failed to save config: {}", e))?;

        Ok(format!(
            "Successfully updated {} to {}",
            key_path, new_value
        ))
    }

    /// Dispatcher that routes LLM tool calls to Rust functions
    pub fn dispatch(call: ToolCall) -> String {
        match call.tool.as_str() {
            "modify_config" => {
                let key = call.arguments["key"].as_str().unwrap_or_default();
                let val = call.arguments["value"].as_str().unwrap_or_default();
                Self::modify_config(key, val).unwrap_or_else(|e| e)
            }
            _ => format!("Unknown tool: {}", call.tool),
        }
    }
}
