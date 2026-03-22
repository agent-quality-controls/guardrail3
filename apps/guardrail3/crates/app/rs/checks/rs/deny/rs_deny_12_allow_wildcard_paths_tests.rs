use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_allow_wildcard_paths_is_false() {
    let deny = canonical_deny_toml_service()
        .replace("allow-wildcard-paths = true", "allow-wildcard-paths = false");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-12"));
}
