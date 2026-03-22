use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_and_inventories_allow_git_entries() {
    let deny = canonical_deny_toml_service().replace(
        "allow-git = []",
        "allow-git = [\"https://github.com/example/repo\"]",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-20" && !r.inventory));
    assert!(results.iter().any(|r| r.id == "RS-DENY-20" && r.inventory));
}
