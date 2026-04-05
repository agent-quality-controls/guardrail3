use guardrail3_domain_report::{CheckResult, Severity};
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_family_for_tests;

#[test]
fn inventories_migrated_threshold_rule_through_family_package_bridge() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_family_for_tests(&tree);

    let package_results = results_for_id(&results, "RS-CLIPPY-02");
    assert_eq!(package_results.len(), 1);

    let result = package_results[0];
    assert_eq!(result.severity(), Severity::Info);
    assert!(result.inventory());
    assert_eq!(result.title(), "max-struct-bools correct");
    assert_eq!(result.message(), "max-struct-bools = 3");
    assert_eq!(result.file(), Some("clippy.toml"));

    let parseability = results_for_id(&results, "RS-CLIPPY-25");
    assert_eq!(parseability.len(), 1);
    assert_eq!(parseability[0].severity(), Severity::Info);
    assert!(parseability[0].inventory());
}

#[test]
fn reports_migrated_threshold_rule_failure_through_family_package_bridge() {
    let tree = root_workspace_tree("max-struct-bools = 4\n");
    let results = run_family_for_tests(&tree);

    let package_results = results_for_id(&results, "RS-CLIPPY-02");
    assert_eq!(package_results.len(), 1);

    let result = package_results[0];
    assert_eq!(result.severity(), Severity::Error);
    assert!(!result.inventory());
    assert_eq!(result.title(), "max-struct-bools wrong value");
    assert_eq!(
        result.message(),
        "Expected 3, got 4. Set `max-struct-bools = 3` in clippy.toml."
    );
    assert_eq!(result.file(), Some("clippy.toml"));

    let parseability = results_for_id(&results, "RS-CLIPPY-25");
    assert_eq!(parseability.len(), 1);
    assert_eq!(parseability[0].severity(), Severity::Info);
    assert!(parseability[0].inventory());
}

fn results_for_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results.iter().filter(|result| result.id() == id).collect()
}
