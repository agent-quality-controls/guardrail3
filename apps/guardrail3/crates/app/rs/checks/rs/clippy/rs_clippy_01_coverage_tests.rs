use crate::domain::report::Severity;

use super::super::test_support::{
    collected_facts, covered_input, root_coverage_tree, uncovered_input, uncovered_standalone_tree,
};
use super::{check_covered, check_uncovered};

#[test]
fn inventories_covered_units() {
    let facts = collected_facts(&root_coverage_tree());

    let workspace = covered_input(&facts, "workspace");
    let mut workspace_results = Vec::new();
    check_covered(&workspace, &mut workspace_results);

    let standalone = covered_input(&facts, "standalone");
    let mut standalone_results = Vec::new();
    check_covered(&standalone, &mut standalone_results);

    assert_eq!(workspace_results[0].severity, Severity::Info);
    assert!(workspace_results[0].inventory);
    assert_eq!(
        workspace_results[0].message,
        "workspace root `workspace` is covered by `clippy.toml`."
    );

    assert_eq!(standalone_results[0].severity, Severity::Info);
    assert!(standalone_results[0].inventory);
    assert_eq!(
        standalone_results[0].message,
        "standalone package root `standalone` is covered by `clippy.toml`."
    );
}

#[test]
fn errors_on_uncovered_unit() {
    let facts = collected_facts(&uncovered_standalone_tree());
    let input = uncovered_input(&facts, "standalone");
    let mut results = Vec::new();

    check_uncovered(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-01");
    assert_eq!(result.severity, Severity::Error);
    assert!(!result.inventory);
    assert_eq!(result.file.as_deref(), Some("standalone"));
    assert_eq!(
        result.message,
        "standalone package root `standalone` is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
    );
}
