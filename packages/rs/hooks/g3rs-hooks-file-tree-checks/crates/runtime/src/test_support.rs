use g3rs_hooks_file_tree_checks_types::G3RsHooksScriptFileFact;

pub(crate) fn script(
    rel_path: &str,
    line_count: usize,
    byte_count: usize,
    executable: Option<bool>,
) -> G3RsHooksScriptFileFact {
    G3RsHooksScriptFileFact {
        rel_path: rel_path.to_owned(),
        line_count,
        byte_count,
        executable,
    }
}
