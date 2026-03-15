pub mod types;

use std::path::Path;

pub fn load_config(path: &Path) -> Option<types::GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = std::fs::read_to_string(&config_path).ok()?;
    match toml::from_str(&content) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            eprintln!("Error parsing guardrail3.toml: {e}");
            None
        }
    }
}
