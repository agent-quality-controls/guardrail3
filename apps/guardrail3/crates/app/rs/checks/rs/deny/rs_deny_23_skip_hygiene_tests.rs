use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_malformed_skip_entries() {
    let deny = canonical_deny_toml_service().replace("skip = [\n]\n", "skip = [\n    { version = \"1.0.0\" },\n]\n");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-23" && !r.inventory));
}
