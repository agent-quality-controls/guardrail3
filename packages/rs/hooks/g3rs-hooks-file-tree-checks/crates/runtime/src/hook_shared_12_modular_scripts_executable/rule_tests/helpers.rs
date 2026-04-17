pub(super) fn script(
    rel_path: &str,
    line_count: usize,
    byte_count: usize,
    executable: Option<bool>,
) -> g3rs_hooks_types::G3RsHooksScriptFileFact {
    g3rs_hooks_types::G3RsHooksScriptFileFact {
        rel_path: rel_path.to_owned(),
        line_count,
        byte_count,
        executable,
    }
}
