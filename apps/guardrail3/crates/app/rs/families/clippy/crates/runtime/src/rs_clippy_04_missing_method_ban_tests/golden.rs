use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn inventories_every_expected_method_ban_from_generated_service_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(!results.is_empty());
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-04"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "method ban present"
            && result.file.as_deref() == Some("clippy.toml")
    }));
    assert!(
        results
            .iter()
            .any(|result| result.message == "`std::env::var` is banned.")
    );
    assert!(
        results
            .iter()
            .any(|result| result.message == "`std::process::abort` is banned.")
    );
}
