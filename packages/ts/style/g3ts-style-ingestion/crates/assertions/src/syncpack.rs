/// Assert that `group.dependencies` contains `expected`.
///
/// # Panics
///
/// Panics when `expected` is not present in `group.dependencies`.
pub fn assert_version_group_dependency(
    group: &g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot,
    expected: &str,
) {
    assert!(
        group
            .dependencies
            .iter()
            .any(|dependency| dependency == expected),
        "normalized Syncpack version group should include dependency `{expected}`"
    );
}

/// Assert that `group.pin_version` equals `Some(expected)`.
///
/// # Panics
///
/// Panics when `group.pin_version` is None or differs from `expected`.
pub fn assert_version_group_pin(
    group: &g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot,
    expected: &str,
) {
    assert_eq!(
        group.pin_version.as_deref(),
        Some(expected),
        "normalized Syncpack version group should preserve pin version"
    );
}
