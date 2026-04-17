use toml::Value;

use crate::cargo_toml::{CargoToml, ToolLints};

#[derive(Debug, Clone, PartialEq)]
pub struct CargoTomlDocument {
    pub raw: Value,
    pub typed: CargoTomlParseState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CargoTomlParseState {
    Parsed(CargoToml),
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoTomlDocumentKind {
    WorkspaceRoot,
    PackageRoot,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoStringFieldState<'a> {
    Missing,
    Value(&'a str),
    Inherit,
    WrongType(&'a Value),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoBoolFieldState<'a> {
    Missing,
    Value(bool),
    WrongType(&'a Value),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoLintTableState<'a> {
    Missing,
    Parsed(&'a ToolLints),
    WrongType(&'a Value),
}
