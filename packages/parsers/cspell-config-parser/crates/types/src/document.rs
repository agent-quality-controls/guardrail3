#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CspellConfigDocument {
    pub raw: Value,
    pub typed: CspellConfigParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CspellConfigParseState {
    Parsed(CspellConfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CspellConfigSnapshot {
    pub raw: Value,
}
