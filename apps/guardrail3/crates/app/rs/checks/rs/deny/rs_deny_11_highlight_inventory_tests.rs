use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn inventories_nonbaseline_highlight() {
    let deny = canonical_deny_toml_service().replace("highlight = \"all\"", "highlight = \"simplest\"");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-11" && r.inventory));
}
