use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn inventories_added_wrappers_for_non_canonical_bans() {
    let mut parsed =
        toml::from_str::<toml::Value>(&canonical_bans_toml("service")).expect("valid deny fixture");
    let deny_entries = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected bans.deny array");
    deny_entries.push(toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String("custom-crate".to_owned())),
        (
            "wrappers".to_owned(),
            toml::Value::Array(vec![toml::Value::String("adapter".to_owned())]),
        ),
    ])));
    let deny_toml = toml::to_string(&parsed).expect("serialize deny");

    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "project-specific ban wrappers",
            "`deny.toml` ban `custom-crate` adds project-specific wrappers `adapter`.",
            "deny.toml",
            true,
        )],
    );
}

#[test]
fn inventories_project_specific_wrappers_for_canonical_bans_without_managed_wrappers() {
    let deny_toml = canonical_bans_toml("service").replace(
        "\"anyhow\"",
        "{ name = \"anyhow\", wrappers = [\"texting_robots\"] }",
    );

    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "project-specific ban wrappers",
            "`deny.toml` ban `anyhow` adds project-specific wrappers `texting_robots`.",
            "deny.toml",
            true,
        )],
    );
}
