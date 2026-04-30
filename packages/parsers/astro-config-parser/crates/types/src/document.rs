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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstroConfigSnapshot {
    pub selected_config: AstroConfigSelectedFile,
    pub site: Option<String>,
    pub output: Option<AstroOutputMode>,
    pub out_dir: Option<String>,
    pub trailing_slash: Option<AstroTrailingSlashPolicy>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AstroTrailingSlashPolicy {
    Always,
    Never,
    Ignore,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AstroIntegrationSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
    pub imported_name: Option<String>,
    pub call: Option<AstroCallSnapshot>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AstroAdapterSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
    pub imported_name: Option<String>,
    pub call: Option<AstroCallSnapshot>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AstroCallSnapshot {
    pub first_arg: Option<AstroStaticValue>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AstroStaticValue {
    Bool(bool),
    Number(f64),
    String(String),
    Null,
    Array(Vec<AstroStaticValue>),
    Object(Vec<AstroStaticObjectProperty>),
    ImportedIdentifier {
        local_name: String,
        source_module: Option<String>,
        imported_name: Option<String>,
    },
    UnsupportedExpression {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AstroStaticObjectProperty {
    pub key: String,
    pub value: AstroStaticValue,
}
