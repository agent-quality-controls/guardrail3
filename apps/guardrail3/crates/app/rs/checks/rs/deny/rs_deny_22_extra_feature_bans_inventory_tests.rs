use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn inventories_extra_feature_bans() {
    let deny = format!(
        "{}\n[[bans.features]]\nname = \"serde\"\ndeny = [\"derive\"]\n",
        canonical_deny_toml_service()
    );
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-22" && r.inventory));
}
