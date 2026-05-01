pub fn assert_scoped_style_globs(actual: Option<Vec<String>>, expected: &[&str]) {
    assert_eq!(
        actual,
        Some(expected.iter().map(|value| (*value).to_owned()).collect()),
        "style directive source globs should be app-scoped"
    );
}

pub fn assert_glob_matches(compiled: &globset::GlobSet, rel_path: &str) {
    assert!(
        compiled.is_match(rel_path),
        "compiled style source glob should match `{rel_path}`"
    );
}

pub fn assert_glob_does_not_match(compiled: &globset::GlobSet, rel_path: &str) {
    assert!(
        !compiled.is_match(rel_path),
        "compiled style source glob should not match `{rel_path}`"
    );
}
