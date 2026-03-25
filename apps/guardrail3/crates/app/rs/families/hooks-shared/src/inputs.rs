use super::facts::HookScriptKind;
use crate::hook_shell::{ExecutableLine, ParsedShellScript};

pub struct ExecutableCommandContextInput<'a> {
    pub rel_path: &'a str,
    pub kind: HookScriptKind,
    pub content: &'a str,
    pub parsed: &'a ParsedShellScript<'a>,
}

pub struct DispatcherSyntaxInput<'a> {
    pub rel_path: &'a str,
    pub has_modular_dir: bool,
    pub parsed: &'a ParsedShellScript<'a>,
}

pub struct FailOpenWrapperInput<'a> {
    pub rel_path: &'a str,
    pub executable_lines: &'a [ExecutableLine<'a>],
}
