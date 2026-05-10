#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain (Tsconfig) and document role"
)]

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsconfigDocument {
    pub raw: Value,
    pub typed: TsconfigParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsconfigParseState {
    Parsed(TsconfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TsconfigSnapshot {
    pub extends: Vec<String>,
    pub compiler_options: TsconfigCompilerOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TsconfigCompilerOptions {
    pub strict: Option<bool>,
    pub no_implicit_returns: Option<bool>,
    pub no_unused_locals: Option<bool>,
    pub no_unused_parameters: Option<bool>,
    pub no_unchecked_indexed_access: Option<bool>,
    pub exact_optional_property_types: Option<bool>,
    pub isolated_modules: Option<bool>,
    pub no_property_access_from_index_signature: Option<bool>,
    pub no_implicit_override: Option<bool>,
    pub no_fallthrough_cases_in_switch: Option<bool>,
    pub force_consistent_casing_in_file_names: Option<bool>,
    pub allow_unreachable_code: Option<bool>,
    pub allow_unused_labels: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsconfigBoolFieldState<'a> {
    Missing,
    Value(bool),
    WrongType(&'a Value),
}
