#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain and document role"
)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageScriptCommandDocument {
    pub raw: String,
    pub typed: PackageScriptParseFact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageScriptParseFact {
    pub script_name: String,
    pub commands: Vec<PackageScriptCommand>,
    pub tool_invocations: Vec<PackageScriptToolInvocation>,
    pub state: PackageScriptParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageScriptParseState {
    Parsed {
        commands: Vec<PackageScriptCommand>,
        eslint_invocations: Vec<EslintInvocation>,
    },
    NoEslintInvocation,
    Unsupported {
        reason: String,
    },
    ParseError {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageScriptCommand {
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<PackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageScriptToolInvocation {
    pub script_name: String,
    pub command_index: usize,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<PackageScriptCommandSeparator>,
    pub followed_by: Option<PackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EslintInvocation {
    pub script_name: String,
    pub command_index: usize,
    pub invocation: String,
    pub args: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub ignore_path: Option<String>,
    pub config_path: Option<String>,
}
