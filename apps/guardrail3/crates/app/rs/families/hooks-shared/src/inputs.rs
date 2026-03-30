use super::facts::HookScriptKind;
use crate::hook_shell::{ExecutableLine, ParsedShellScript};

pub struct ExecutableCommandContextInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) kind: HookScriptKind,
    pub(crate) content: &'a str,
    pub(crate) parsed: &'a ParsedShellScript<'a>,
}

pub struct DispatcherSyntaxInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) has_modular_dir: bool,
    pub(crate) parsed: &'a ParsedShellScript<'a>,
}

pub struct FailOpenWrapperInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) executable_lines: &'a [ExecutableLine<'a>],
}
