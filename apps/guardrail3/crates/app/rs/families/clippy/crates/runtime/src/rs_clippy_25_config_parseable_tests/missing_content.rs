use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn errors_when_allowed_clippy_config_content_is_missing() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![("Cargo.toml", "[workspace]\nmembers = []".to_owned())],
    );
    let results = run_for_tests(&tree, "clippy.toml");
    let result = results
        .iter()
        .find(|result| result.id == "RS-CLIPPY-25")
        .expect("expected RS-CLIPPY-25 result");

    assert_eq!(result.severity, guardrail3_domain_report::Severity::Error);
    assert_eq!(result.title, "clippy.toml parse error");
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
    assert_eq!(
        result.message,
        "Failed to parse `clippy.toml`: clippy.toml content missing from ProjectTree"
    );
}
