use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_unknown_git_policy_is_weakened() {
    let deny = canonical_deny_toml_service().replace("unknown-git = \"deny\"", "unknown-git = \"allow\"");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-18"));
}
