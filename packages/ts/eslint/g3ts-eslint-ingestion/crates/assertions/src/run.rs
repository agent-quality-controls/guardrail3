use g3ts_eslint_types::G3TsEslintConfigState;

/// Fails the calling test when `input.config` is not `Parsed` with the expected `rel_path`.
///
/// # Panics
/// Panics on state or path mismatch, which the assertion treats as a test failure.
pub fn assert_parsed_rel_path(
    input: &g3ts_eslint_types::G3TsEslintConfigChecksInput,
    expected: &str,
) {
    let G3TsEslintConfigState::Parsed { snapshot } = &input.config else {
        assert!(
            matches!(input.config, G3TsEslintConfigState::Parsed { .. }),
            "expected parsed config state, got: {:?}",
            input.config
        );
        return;
    };
    assert_eq!(
        snapshot.selected_config.rel_path, expected,
        "parsed config path mismatch"
    );
}

/// Fails the calling test when `input.config` is not `Missing`.
///
/// # Panics
/// Panics on state mismatch, which the assertion treats as a test failure.
pub fn assert_missing(input: &g3ts_eslint_types::G3TsEslintConfigChecksInput) {
    assert!(
        matches!(input.config, G3TsEslintConfigState::Missing),
        "expected missing config state, got: {:?}",
        input.config
    );
}

/// Fails the calling test when `input.config` is not `Unreadable` with the expected `rel_path`.
///
/// # Panics
/// Panics on state or path mismatch, which the assertion treats as a test failure.
pub fn assert_unreadable(
    input: &g3ts_eslint_types::G3TsEslintConfigChecksInput,
    expected_rel_path: &str,
) {
    let G3TsEslintConfigState::Unreadable { rel_path, .. } = &input.config else {
        assert!(
            matches!(input.config, G3TsEslintConfigState::Unreadable { .. }),
            "expected unreadable config state, got: {:?}",
            input.config
        );
        return;
    };
    assert_eq!(
        rel_path, expected_rel_path,
        "unreadable config path mismatch"
    );
}
