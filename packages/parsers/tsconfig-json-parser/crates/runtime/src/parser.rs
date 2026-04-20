use serde_json::Value;
use tsconfig_json_parser_types::document::{
    TsconfigCompilerOptions, TsconfigDocument, TsconfigParseState, TsconfigSnapshot,
};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized tsconfig JSONC parser"
)]
pub fn parse(input: &str) -> Result<TsconfigSnapshot, crate::error::Error> {
    let raw: Value = jsonc_parser::parse_to_serde_value(input, &Default::default())
        .map_err(|err| crate::error::Error::Jsonc(err.to_string()))?;
    normalize_snapshot(&raw).map_err(crate::error::Error::Jsonc)
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized tsconfig JSONC parser"
)]
pub fn parse_document(input: &str) -> Result<TsconfigDocument, crate::error::Error> {
    let raw: Value = jsonc_parser::parse_to_serde_value(input, &Default::default())
        .map_err(|err| crate::error::Error::Jsonc(err.to_string()))?;
    let typed = match normalize_snapshot(&raw) {
        Ok(snapshot) => TsconfigParseState::Parsed(snapshot),
        Err(reason) => TsconfigParseState::Invalid(reason),
    };
    Ok(TsconfigDocument { raw, typed })
}

pub fn from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<TsconfigSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<TsconfigDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

fn normalize_snapshot(raw: &Value) -> Result<TsconfigSnapshot, String> {
    let root = raw
        .as_object()
        .ok_or_else(|| "tsconfig root must be a JSON object".to_owned())?;
    let extends = normalize_extends(root.get("extends"))?;
    let compiler_options = normalize_compiler_options(root.get("compilerOptions"))?;
    Ok(TsconfigSnapshot {
        extends,
        compiler_options,
    })
}

fn normalize_extends(value: Option<&Value>) -> Result<Vec<String>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    match value {
        Value::String(single) => Ok(vec![single.clone()]),
        Value::Array(items) => items
            .iter()
            .map(|item| match item {
                Value::String(entry) => Ok(entry.clone()),
                _ => Err("tsconfig extends array must contain only strings".to_owned()),
            })
            .collect(),
        _ => Err("tsconfig extends must be a string or string array".to_owned()),
    }
}

fn normalize_compiler_options(value: Option<&Value>) -> Result<TsconfigCompilerOptions, String> {
    let Some(value) = value else {
        return Ok(TsconfigCompilerOptions::default());
    };
    let options = value
        .as_object()
        .ok_or_else(|| "tsconfig compilerOptions must be a JSON object".to_owned())?;

    Ok(TsconfigCompilerOptions {
        strict: bool_value(options.get("strict")),
        no_implicit_returns: bool_value(options.get("noImplicitReturns")),
        no_unused_locals: bool_value(options.get("noUnusedLocals")),
        no_unused_parameters: bool_value(options.get("noUnusedParameters")),
        no_unchecked_indexed_access: bool_value(options.get("noUncheckedIndexedAccess")),
        exact_optional_property_types: bool_value(options.get("exactOptionalPropertyTypes")),
        isolated_modules: bool_value(options.get("isolatedModules")),
        no_property_access_from_index_signature: bool_value(
            options.get("noPropertyAccessFromIndexSignature"),
        ),
        no_implicit_override: bool_value(options.get("noImplicitOverride")),
        no_fallthrough_cases_in_switch: bool_value(options.get("noFallthroughCasesInSwitch")),
        force_consistent_casing_in_file_names: bool_value(
            options.get("forceConsistentCasingInFileNames"),
        ),
        allow_unreachable_code: bool_value(options.get("allowUnreachableCode")),
        allow_unused_labels: bool_value(options.get("allowUnusedLabels")),
    })
}

fn bool_value(value: Option<&Value>) -> Option<bool> {
    value.and_then(Value::as_bool)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
