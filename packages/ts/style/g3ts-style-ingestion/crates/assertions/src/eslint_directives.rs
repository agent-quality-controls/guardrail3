/// Assert that `actual` is `Some(expected)` (treating string content as
/// app-scoped style directive source globs).
///
/// # Panics
///
/// Panics when `actual` is `None` or differs from `expected`.
pub fn assert_scoped_style_globs(actual: Option<&[String]>, expected: &[&str]) {
    let expected: Vec<String> = expected.iter().map(|value| (*value).to_owned()).collect();
    assert_eq!(
        actual,
        Some(expected.as_slice()),
        "style directive source globs should be app-scoped"
    );
}

/// Assert that `compiled` matches `rel_path`.
///
/// # Panics
///
/// Panics when `compiled.is_match(rel_path)` returns false.
pub fn assert_glob_matches(compiled: &globset::GlobSet, rel_path: &str) {
    assert!(
        compiled.is_match(rel_path),
        "compiled style source glob should match `{rel_path}`"
    );
}

/// Assert that `compiled` does not match `rel_path`.
///
/// # Panics
///
/// Panics when `compiled.is_match(rel_path)` returns true.
pub fn assert_glob_does_not_match(compiled: &globset::GlobSet, rel_path: &str) {
    assert!(
        !compiled.is_match(rel_path),
        "compiled style source glob should not match `{rel_path}`"
    );
}
