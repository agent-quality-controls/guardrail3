use super::super::run_for_tests;

#[test]
fn inventories_when_clippy_config_parses() {
    let tree = test_support::root_workspace_tree(
        guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", ""),
    );
    let results = run_for_tests(&tree, "clippy.toml");
    let result = results
        .iter()
        .find(|result| result.id == "RS-CLIPPY-25")
        .expect("expected RS-CLIPPY-25 result");

    assert_eq!(result.severity, guardrail3_domain_report::Severity::Info);
    assert_eq!(result.title, "clippy.toml parseable");
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
}
