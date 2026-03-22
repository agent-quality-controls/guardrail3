use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_no_default_features_is_true() {
    let deny = canonical_deny_toml_service()
        .replace("no-default-features = false", "no-default-features = true");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-08"));
}
