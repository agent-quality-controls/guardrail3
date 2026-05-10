/// Returns `rel_dir` for display, substituting `.` for the empty unit-root case.
pub(crate) const fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}
