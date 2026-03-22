use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_duplicate_entries() {
    let deny = canonical_deny_toml_service()
        .replace("{ name = \"regex\", wrappers = [] },\n", "{ name = \"regex\", wrappers = [] },\n    { name = \"regex\", wrappers = [] },\n")
        .replace("ignore = []", "ignore = [\"RUSTSEC-2020-0001\", \"RUSTSEC-2020-0001\"]");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-27"));
}
