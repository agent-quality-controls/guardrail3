pub fn assert_version_group_dependency(
    group: &g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot,
    expected: &str,
) {
    assert!(
        group.dependencies.iter().any(|dependency| dependency == expected),
        "normalized Syncpack version group should include dependency `{expected}`"
    );
}

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
