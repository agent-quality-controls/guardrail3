use super::super::run_for_tests;

#[test]
fn errors_when_clippy_config_cannot_be_parsed() {
    let tree = test_support::root_workspace_tree("not = [valid");
    let results = run_for_tests(&tree, "clippy.toml");
    let result = results
        .iter()
        .find(|result| result.id == "RS-CLIPPY-25")
        .expect("expected RS-CLIPPY-25 result");

    assert_eq!(result.severity, guardrail3_domain_report::Severity::Error);
    assert_eq!(result.title, "clippy.toml parse error");
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
    assert!(
        result
            .message
            .starts_with("Failed to parse `clippy.toml`: "),
        "unexpected message: {}",
        result.message
    );
}
