use super::facts::HookScriptKind;
use g3rs_hooks_contract_types::G3HookRequirement;
use hook_shell_parser::types::ParsedShellScript;

/// `RustHookCommandInput` struct.
pub(crate) struct RustHookCommandInput<'a> {
    /// `rel_path` item.
    pub(crate) rel_path: &'a str,
    /// `parsed` item.
    pub(crate) parsed: &'a ParsedShellScript,
    /// `is_workspace_project` item.
    pub(crate) is_workspace_project: bool,
    /// `requirements` item.
    pub(crate) requirements: &'a [G3HookRequirement],
}

/// `ExecutableCommandContextInput` struct.
pub(crate) struct ExecutableCommandContextInput<'a> {
    /// `rel_path` item.
    pub(crate) rel_path: &'a str,
    /// `kind` item.
    pub(crate) kind: HookScriptKind,
    /// `parsed` item.
    pub(crate) parsed: &'a ParsedShellScript,
}

/// `DispatcherSyntaxInput` struct.
pub(crate) struct DispatcherSyntaxInput<'a> {
    /// `rel_path` item.
    pub(crate) rel_path: &'a str,
    /// `has_modular_dir` item.
    pub(crate) has_modular_dir: bool,
    /// `parsed` item.
    pub(crate) parsed: &'a ParsedShellScript,
}

/// `FailOpenWrapperInput` struct.
pub(crate) struct FailOpenWrapperInput<'a> {
    /// `rel_path` item.
    pub(crate) rel_path: &'a str,
    /// `parsed` item.
    pub(crate) parsed: &'a ParsedShellScript,
    /// `requirements` item.
    pub(crate) requirements: &'a [G3HookRequirement],
}
