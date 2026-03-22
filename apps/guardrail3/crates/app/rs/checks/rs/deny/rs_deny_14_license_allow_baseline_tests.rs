use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_license_baseline_is_missing() {
    let deny = canonical_deny_toml_service().replace("\"MIT\",\n", "");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-14" && r.message.contains("MIT")));
}
