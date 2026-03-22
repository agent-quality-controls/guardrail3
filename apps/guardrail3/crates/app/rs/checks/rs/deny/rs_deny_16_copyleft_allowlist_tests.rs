use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_copyleft_license() {
    let deny = canonical_deny_toml_service().replace(
        "\"MIT\",\n",
        "\"MIT\",\n    \"GPL-3.0\",\n",
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-16" && r.message.contains("GPL-3.0")));
}
