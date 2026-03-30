use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;

pub struct RustHookCommandInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) parsed: &'a ParsedShellScript<'a>,
}
