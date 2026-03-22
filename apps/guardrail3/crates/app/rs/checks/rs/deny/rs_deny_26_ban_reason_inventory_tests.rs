use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn inventories_named_ban_entries_without_reason() {
    let deny = canonical_deny_toml_service().replace(
        "{ name = \"lazy_static\", wrappers = [] },",
        "{ name = \"lazy_static\", wrappers = [] },",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-26"
            && result.inventory
            && result.title == "ban entry missing reason"
            && result.message.contains("lazy_static")
    }));
}

#[test]
fn inventories_plain_string_ban_entries_without_reason() {
    let deny = canonical_deny_toml_service().replace(
        "{ name = \"lazy_static\", wrappers = [] },",
        "\"lazy_static\",",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-26"
            && result.inventory
            && result.message.contains("lazy_static")
    }));
}
