/// Fails the calling test when the target's `rel_path` or `probe` kind does not match `expected_path` and `expected_kind`.
///
/// # Panics
/// Panics on mismatch, which the assertion treats as a test failure.
pub fn assert_target(
    target: &eslint_config_parser::types::EslintProbeTarget,
    expected_path: &str,
    expected_kind: eslint_config_parser::types::EslintProbeKind,
) {
    assert_eq!(target.rel_path, expected_path, "target rel_path mismatch");
    assert_eq!(target.probe, expected_kind, "target probe kind mismatch");
}
