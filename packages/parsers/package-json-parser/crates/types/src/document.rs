use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageJsonDocument {
    pub raw: Value,
    pub typed: PackageJsonParseState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackageJsonParseState {
    Parsed(PackageJsonSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageJsonSnapshot {
    pub private_field: Option<bool>,
    pub package_manager: Option<String>,
    pub engines_node: Option<String>,
    pub engines_pnpm: Option<String>,
    pub scripts: BTreeMap<String, String>,
    pub pnpm_override_keys: Vec<String>,
    pub pnpm_only_built_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageJsonBoolFieldState<'a> {
    Missing,
    Value(bool),
    WrongType(&'a Value),
}
