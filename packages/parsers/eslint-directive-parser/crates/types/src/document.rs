#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslintDirectiveDocument {
    pub raw: String,
    pub typed: EslintDirectiveFileState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslintDirectiveFileState {
    pub rel_path: String,
    pub state: EslintDirectiveParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EslintDirectiveParseState {
    Parsed { findings: Vec<EslintDirectiveFinding> },
    Unsupported { reason: String },
    ParseError { reason: String },
    Ambiguous { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslintDirectiveFinding {
    pub rel_path: String,
    pub directive_kind: EslintDirectiveKind,
    pub disabled_rules: EslintDisabledRuleSet,
    pub line: u32,
    pub target_line: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EslintDirectiveKind {
    Disable,
    DisableLine,
    DisableNextLine,
    Enable,
    InlineConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EslintDisabledRuleSet {
    AllRules,
    Rules(Vec<String>),
}
