#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncpackConfigDocument {
    pub raw: Value,
    pub typed: SyncpackConfigParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncpackConfigParseState {
    Parsed(Box<SyncpackConfigSnapshot>),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SyncpackConfigSnapshot {
    pub source: Vec<String>,
    pub version_groups: Vec<SyncpackVersionGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncpackVersionGroup {
    pub label: Option<String>,
    pub dependencies: Vec<String>,
    pub dependency_types: Vec<String>,
    pub packages: Vec<String>,
    pub specifier_types: Vec<String>,
    pub pin_version: Option<String>,
    pub is_banned: bool,
    pub is_ignored: bool,
}
