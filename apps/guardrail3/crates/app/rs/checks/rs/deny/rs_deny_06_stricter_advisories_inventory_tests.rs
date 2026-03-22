use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn inventories_stricter_advisory_settings() {
    let deny = canonical_deny_toml_service().replace("yanked = \"warn\"", "yanked = \"deny\"");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-06" && r.inventory));
}
