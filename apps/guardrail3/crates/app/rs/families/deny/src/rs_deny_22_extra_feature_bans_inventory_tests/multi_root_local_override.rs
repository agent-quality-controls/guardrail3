use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_deny_toml_service, copy_fixture, set_feature_entries, write_file,
};

fn feature_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(vec![toml::Value::String("derive".to_owned())]),
        ),
    ]))
}

#[test]
fn local_extra_feature_bans_inventory_stays_owned_by_the_local_root() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let existing = parsed
        .get("bans")
        .and_then(|b| b.get("features"))
        .and_then(toml::Value::as_array)
        .cloned()
        .expect("feature entries");
    let mut local_entries = existing;
    local_entries.push(feature_entry("serde"));

    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_feature_entries(&build_deny_toml("service", "", "", ""), local_entries),
    );

    let results = super::super::super::test_support::run_family(tmp.path());
    let feature_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-22")
        .collect::<Vec<_>>();

    assert_eq!(feature_results.len(), 1, "{feature_results:#?}");
    let result = feature_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra feature ban");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` has extra feature-ban entry for `serde`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
