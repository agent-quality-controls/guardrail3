use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn errors_when_managed_wrappers_change() {
    let deny = canonical_deny_toml_service().replace(
        "{ name = \"anyhow\", wrappers = [] },",
        "{ name = \"anyhow\", wrappers = [\"texting_robots\"] },",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-30"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "project-specific ban wrappers"
            && result.message.contains("anyhow")
    }));
}

#[test]
fn inventories_project_specific_wrappers_for_non_canonical_bans() {
    let deny = canonical_deny_toml_service().replace(
        "{ name = \"lazy_static\", wrappers = [] },",
        "{ name = \"lazy_static\", wrappers = [] },\n    { name = \"custom-crate\", wrappers = [\"adapter\"] },",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-30"
            && result.inventory
            && result.title == "project-specific ban wrappers"
            && result.message.contains("custom-crate")
            && result.message.contains("adapter")
    }));
}
