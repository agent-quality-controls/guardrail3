use jsonc_parser::ParseOptions;
use serde_json::Value;
use tsconfig_json_parser_types::document::{
    TsconfigCompilerOptions, TsconfigDocument, TsconfigParseState, TsconfigSnapshot,
};

/// Internal `Result` alias used by the typed-snapshot normalizers.
type NormalizeResult<T> = Result<T, String>;

/// Parses `tsconfig.json` content into a typed snapshot.
///
/// # Errors
/// Returns `Error::Jsonc` when the input is not valid JSONC or its shape is not a `tsconfig.json` object.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized tsconfig JSONC parser"
)]
pub fn parse(input: &str) -> Result<TsconfigSnapshot, crate::error::Error> {
    let raw = parse_jsonc_value(input)?;
    normalize_snapshot(&raw).map_err(crate::error::Error::Jsonc)
}

/// Parses `tsconfig.json` content into a typed document, capturing schema mismatches as `Invalid`.
///
/// # Errors
/// Returns `Error::Jsonc` when the input is not valid JSONC.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized tsconfig JSONC parser"
)]
pub fn parse_document(input: &str) -> Result<TsconfigDocument, crate::error::Error> {
    let raw = parse_jsonc_value(input)?;
    let typed = match normalize_snapshot(&raw) {
        Ok(snapshot) => TsconfigParseState::Parsed(snapshot),
        Err(reason) => TsconfigParseState::Invalid(reason),
    };
    Ok(TsconfigDocument { raw, typed })
}

/// Parses JSONC into a concrete JSON value and rejects empty/comment-only documents.
fn parse_jsonc_value(input: &str) -> Result<Value, crate::error::Error> {
    jsonc_parser::parse_to_serde_value(input, &ParseOptions::default())
        .map_err(|err| crate::error::Error::Jsonc(err.to_string()))?
        .ok_or_else(|| crate::error::Error::Jsonc("tsconfig JSONC document is empty".to_owned()))
}

/// Reads `tsconfig.json` from `path` and parses it into a typed snapshot.
///
/// # Errors
/// Returns `Error::Io` if the file cannot be read, or `Error::Jsonc` if it is not valid JSONC.
pub fn from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<TsconfigSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

/// Reads `tsconfig.json` from `path` and parses it into a typed document.
///
/// # Errors
/// Returns `Error::Io` if the file cannot be read, or `Error::Jsonc` if it is not valid JSONC.
pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<TsconfigDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

/// Asserts the JSONC root is an object and produces a typed `TsconfigSnapshot` from its known fields.
fn normalize_snapshot(raw: &Value) -> NormalizeResult<TsconfigSnapshot> {
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

/// Normalizes the `extends` field, which can be missing, a single string, or an array of strings.
fn normalize_extends(value: Option<&Value>) -> NormalizeResult<Vec<String>> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    match value {
        Value::String(single) => Ok(vec![single.clone()]),
        Value::Array(items) => items
            .iter()
            .map(|item| match item {
                Value::String(entry) => Ok(entry.clone()),
                Value::Null
                | Value::Bool(_)
                | Value::Number(_)
                | Value::Array(_)
                | Value::Object(_) => {
                    Err("tsconfig extends array must contain only strings".to_owned())
                }
            })
            .collect(),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::Object(_) => {
            Err("tsconfig extends must be a string or string array".to_owned())
        }
    }
}

/// Extracts the strict-mode `compilerOptions` flags this parser cares about into a typed snapshot.
fn normalize_compiler_options(value: Option<&Value>) -> NormalizeResult<TsconfigCompilerOptions> {
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

/// Returns `Some(bool)` when `value` is a JSON boolean, otherwise `None`.
fn bool_value(value: Option<&Value>) -> Option<bool> {
    value.and_then(Value::as_bool)
}
