use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_when_multiple_versions_is_weaker() {
    let deny = canonical_deny_toml_service()
        .replace("multiple-versions = \"deny\"", "multiple-versions = \"warn\"");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-10"));
}
