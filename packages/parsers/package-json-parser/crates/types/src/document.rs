#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageJsonDocument {
    pub raw: Value,
    pub typed: PackageJsonParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageJsonParseState {
    Parsed(Box<PackageJsonSnapshot>),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageJsonSnapshot {
    pub name: Option<String>,
    pub private_field: Option<bool>,
    pub package_manager: Option<String>,
    pub engines_node: Option<String>,
    pub engines_pnpm: Option<String>,
    pub scripts: BTreeMap<String, String>,
    pub pnpm_override_keys: Vec<String>,
    pub pnpm_only_built_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub peer_dependencies: Vec<String>,
    pub dependency_declarations: Vec<PackageJsonDependencyDeclarationSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageJsonDependencyDeclarationSnapshot {
    pub name: String,
    pub lane: String,
    pub specifier_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageJsonBoolFieldState<'a> {
    Missing,
    Value(bool),
    WrongType(&'a Value),
}
