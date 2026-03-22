use super::super::check;
use super::super::test_support::{root_coverage_tree, uncovered_standalone_tree};

#[test]
fn inventories_covered_units() {
    let results = check(&root_coverage_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-01" && r.inventory && r.message.contains("workspace root `workspace`")));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-01" && r.inventory && r.message.contains("standalone package root `standalone`")));
}

#[test]
fn errors_on_uncovered_unit() {
    let results = check(&uncovered_standalone_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-01" && !r.inventory && r.message.contains("standalone package root `standalone`")));
}
