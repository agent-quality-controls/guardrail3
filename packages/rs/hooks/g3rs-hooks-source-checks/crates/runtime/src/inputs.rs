use super::facts::HookScriptKind;
use hook_shell_parser::ParsedShellScript;

pub(crate) struct RustHookCommandInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) parsed: &'a ParsedShellScript,
    pub(crate) is_workspace_project: bool,
}

pub(crate) struct ExecutableCommandContextInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) kind: HookScriptKind,
    pub(crate) parsed: &'a ParsedShellScript,
}

pub(crate) struct DispatcherSyntaxInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) has_modular_dir: bool,
    pub(crate) parsed: &'a ParsedShellScript,
}

pub(crate) struct FailOpenWrapperInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) parsed: &'a ParsedShellScript,
}
