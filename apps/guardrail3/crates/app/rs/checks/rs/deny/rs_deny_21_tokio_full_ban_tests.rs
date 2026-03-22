use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_when_tokio_full_is_not_banned() {
    let deny = canonical_deny_toml_service().replace("deny = [\"full\"]", "deny = []");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-21"));
}
