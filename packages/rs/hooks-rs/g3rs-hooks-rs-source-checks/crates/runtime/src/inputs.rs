use hook_shell_parser::ParsedShellScript;

pub(crate) struct RustHookCommandInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) parsed: &'a ParsedShellScript<'a>,
    pub(crate) is_workspace_project: bool,
}
