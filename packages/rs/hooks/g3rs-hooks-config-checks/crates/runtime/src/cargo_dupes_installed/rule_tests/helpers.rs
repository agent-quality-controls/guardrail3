pub(super) fn hook(content: &str) -> g3rs_hooks_types::G3RsHooksSelectedHookConfigFact {
    g3rs_hooks_types::G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        parsed: hook_shell_parser::parse_script(content),
    }
}
