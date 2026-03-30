use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn reports_malformed_type_sections_instead_of_clean_inventory() {
    let tree = root_workspace_tree("disallowed-types = {}\n");
    let results = run_for_tests(&tree, "clippy.toml");

    assert!(
        results.iter().any(|result| {
            result.title == "disallowed-types section malformed"
                && result.message == "`disallowed-types` must be an array, found table."
        }),
        "expected malformed section error: {results:#?}"
    );
    assert!(
        results.iter().all(|result| !result.inventory),
        "malformed sections must not produce clean inventory: {results:#?}"
    );
}
