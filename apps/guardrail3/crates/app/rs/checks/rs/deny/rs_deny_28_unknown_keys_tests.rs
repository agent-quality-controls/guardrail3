use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_unknown_schema_keys() {
    let deny = canonical_deny_toml_service().replace("[graph]\n", "[graph]\nextra-flag = true\n");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-28"));
}
