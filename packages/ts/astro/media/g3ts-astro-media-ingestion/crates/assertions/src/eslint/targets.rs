pub fn assert_target(
    target: &eslint_config_parser::types::EslintProbeTarget,
    expected_path: &str,
    expected_kind: eslint_config_parser::types::EslintProbeKind,
) {
    assert_eq!(target.rel_path, expected_path);
    assert_eq!(target.probe, expected_kind);
}
