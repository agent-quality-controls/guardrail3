use guardrail3_app_rs_family_deny_assertions::rs_deny_22_extra_feature_bans_inventory as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_feature_entries, write_file};

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
        toml::from_str::<toml::Value>(&build_fixture_deny_toml("service")).expect("valid deny TOML");
    let existing = parsed
        .get("bans")
        .and_then(|b| b.get("features"))
        .and_then(toml::Value::as_array)
        .cloned()
        .expect("feature entries");
    let mut local_entries = existing;
    local_entries.push(feature_entry("serde"));

    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_feature_entries(&build_fixture_deny_toml("service"), local_entries),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "extra feature ban",
            "`apps/devctl/deny.toml` has extra feature-ban entry for `serde`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
