use cspell_config_parser_types::document::{
    CspellConfigDocument, CspellConfigParseState, CspellConfigSnapshot,
};
use serde_json::Value;

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized cspell JSON config parser"
)]
pub fn parse_document(input: &str) -> Result<CspellConfigDocument, crate::error::Error> {
    let raw = serde_json::from_str::<Value>(strip_bom(input))
        .map_err(|error| crate::error::Error::Json(error.to_string()))?;
    let typed = match normalize_value(raw.clone()) {
        Ok(snapshot) => CspellConfigParseState::Parsed(snapshot),
        Err(reason) => CspellConfigParseState::Invalid(reason),
    };
    Ok(CspellConfigDocument { raw, typed })
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<CspellConfigDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

fn normalize_value(raw: Value) -> Result<CspellConfigSnapshot, String> {
    let _object = raw
        .as_object()
        .ok_or_else(|| "cspell config root must be a JSON object".to_owned())?;
    Ok(CspellConfigSnapshot { raw })
}

fn strip_bom(input: &str) -> &str {
    input.strip_prefix('\u{FEFF}').unwrap_or(input)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
