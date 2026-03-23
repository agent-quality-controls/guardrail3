use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn inventories_added_wrappers_for_bans_without_managed_wrapper_policy() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "{ name = \"anyhow\", wrappers = [] },",
        "{ name = \"anyhow\", wrappers = [\"texting_robots\"] },",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-30");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "project-specific ban wrappers");
    assert_eq!(
        result.message,
        "`deny.toml` ban `anyhow` adds project-specific wrappers `texting_robots`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}

#[test]
fn inventories_project_specific_wrappers_for_non_canonical_bans() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "deny = [\n",
        "deny = [\n    { name = \"custom-crate\", wrappers = [\"adapter\"] },\n",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "`deny.toml` ban `custom-crate` adds project-specific wrappers `adapter`.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-30"
            && result.severity == Severity::Info
            && result.title == "project-specific ban wrappers"
            && result.file.as_deref() == Some("deny.toml")
            && result.inventory
    }));
}
