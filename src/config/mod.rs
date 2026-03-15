pub mod types;

use std::path::Path;

#[allow(clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI tool — config parse errors reported to stderr; guardrail3 config parsing — no garde validation needed for own config
pub fn load_config(path: &Path) -> Option<types::GuardrailConfig> {
    let config_path = path.join("guardrail3.toml");
    let content = crate::fs::read_file(&config_path)?;
    match toml::from_str(&content) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            eprintln!("Error parsing guardrail3.toml: {e}");
            None
        }
    }
}
