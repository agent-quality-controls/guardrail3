use toml::Value;

use crate::clippy_toml::ClippyToml;

#[derive(Debug, Clone, PartialEq)]
#[expect(
    clippy::module_name_repetitions,
    reason = "public API type consumed across workspaces; rename would break g3rs-clippy-types and g3rs-clippy-config-checks"
)]
pub struct ClippyTomlDocument {
    pub raw: Value,
    pub typed: ClippyTomlParseState,
}

#[derive(Debug, Clone, PartialEq)]
#[expect(
    clippy::large_enum_variant,
    reason = "ClippyToml is the parsed payload; boxing breaks downstream const fn that returns &ClippyToml from &Box<ClippyToml>"
)]
pub enum ClippyTomlParseState {
    Parsed(ClippyToml),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClippyBanEntry {
    pub path: String,
    pub reason: Option<String>,
    pub is_plain_string: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClippyBanSection {
    pub entries: Vec<ClippyBanEntry>,
    pub malformed_messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClippyBoolSetting<'a> {
    Missing,
    WrongType(&'a Value),
    Value(bool),
}
