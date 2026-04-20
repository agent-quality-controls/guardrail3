use g3ts_eslint_types::G3TsEslintConfigState;

pub fn assert_parsed_rel_path(
    input: &g3ts_eslint_types::G3TsEslintConfigChecksInput,
    expected: &str,
) {
    match &input.config {
        G3TsEslintConfigState::Parsed { rel_path, .. } => {
            assert_eq!(rel_path, expected, "parsed config path mismatch");
        }
        other => assert!(false, "expected parsed config state, got: {other:?}"),
    }
}

pub fn assert_missing(input: &g3ts_eslint_types::G3TsEslintConfigChecksInput) {
    match &input.config {
        G3TsEslintConfigState::Missing => {}
        other => assert!(false, "expected missing config state, got: {other:?}"),
    }
}
