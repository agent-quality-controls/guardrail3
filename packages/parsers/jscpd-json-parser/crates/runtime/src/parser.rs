use std::collections::BTreeSet;

use jscpd_json_parser_types::document::{JscpdDocument, JscpdParseState, JscpdSnapshot};
use serde_json::Value;

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .jscpd.json parser"
)]
pub fn parse(input: &str) -> Result<JscpdSnapshot, crate::error::Error> {
    normalize_snapshot(input).map_err(crate::error::Error::Json)
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .jscpd.json parser"
)]
pub fn parse_document(input: &str) -> Result<JscpdDocument, crate::error::Error> {
    let raw = serde_json::from_str::<Value>(strip_bom(input))
        .map_err(|err| crate::error::Error::Json(err.to_string()))?;
    let typed = match normalize_value(raw.clone()) {
        Ok(snapshot) => JscpdParseState::Parsed(snapshot),
        Err(reason) => JscpdParseState::Invalid(reason),
    };
    Ok(JscpdDocument { raw, typed })
}

pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<JscpdSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<JscpdDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

fn normalize_snapshot(raw_input: &str) -> Result<JscpdSnapshot, String> {
    let raw = serde_json::from_str::<Value>(strip_bom(raw_input)).map_err(|err| err.to_string())?;
    normalize_value(raw)
}

fn normalize_value(raw: Value) -> Result<JscpdSnapshot, String> {
    let object = raw
        .as_object()
        .ok_or_else(|| ".jscpd.json root must be a JSON object".to_owned())?;

    let mut snapshot = serde_json::from_value::<JscpdSnapshot>(Value::Object(object.clone()))
        .map_err(|err| err.to_string())?;
    snapshot.extra_keys = object
        .keys()
        .filter(|key| {
            !matches!(
                key.as_str(),
                "$schema"
                    | "threshold"
                    | "minTokens"
                    | "reporters"
                    | "ignore"
                    | "absolute"
                    | "format"
            )
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok(snapshot)
}

fn strip_bom(input: &str) -> &str {
    input.strip_prefix('\u{FEFF}').unwrap_or(input)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
