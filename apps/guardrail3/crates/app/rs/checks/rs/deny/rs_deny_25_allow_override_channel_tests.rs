use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_on_bans_allow_entries() {
    let deny = canonical_deny_toml_service().replace("skip = [\n]\n\n", "skip = [\n]\nallow = [\"regex\"]\n\n");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-25"));
}
