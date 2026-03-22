use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_managed_wrappers_change() {
    let deny = canonical_deny_toml_service().replace(
        "{ name = \"regex\", wrappers = [\"tree-sitter\", \"globset\", \"ignore\"] },",
        "{ name = \"regex\", wrappers = [] },",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-30"));
}
