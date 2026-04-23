use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct AstroConfigDocument {
    pub raw: Value,
    pub typed: AstroConfigParseState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstroConfigParseState {
    Parsed(AstroConfigSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstroConfigSnapshot {
    pub selected_config: AstroConfigSelectedFile,
    pub site: Option<String>,
    pub output: Option<AstroOutputMode>,
    pub integrations: Vec<AstroIntegrationSnapshot>,
    pub adapter: Option<AstroAdapterSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstroConfigSelectedFile {
    pub rel_path: String,
    pub kind: AstroConfigFileKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AstroConfigFileKind {
    Js,
    Mjs,
    Cjs,
    Ts,
    Mts,
    Cts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AstroOutputMode {
    Static,
    Server,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AstroIntegrationSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AstroAdapterSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
}
