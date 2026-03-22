use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn inventories_license_exceptions() {
    let deny = format!(
        "{}\n[[licenses.exceptions]]\nname = \"demo\"\nallow = [\"MIT\"]\n",
        canonical_deny_toml_service()
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-17" && r.inventory));
}
