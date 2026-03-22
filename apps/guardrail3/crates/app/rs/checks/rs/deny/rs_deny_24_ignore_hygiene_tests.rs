use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_malformed_ignore_entries() {
    let deny = canonical_deny_toml_service().replace("ignore = []", "ignore = [{ reason = 1 }]");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-24" && !r.inventory));
}
