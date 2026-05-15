#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain (CargoToml) and document role"
)]

use serde::Serialize;
use toml::Value;

use crate::cargo_toml::{CargoToml, ToolLints};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CargoTomlDocument {
    pub raw: Value,
    pub typed: CargoTomlParseState,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum CargoTomlParseState {
    Parsed(Box<CargoToml>),
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
