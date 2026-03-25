use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;

pub struct RustHookCommandInput<'a> {
    pub rel_path: &'a str,
    pub parsed: &'a ParsedShellScript<'a>,
}
