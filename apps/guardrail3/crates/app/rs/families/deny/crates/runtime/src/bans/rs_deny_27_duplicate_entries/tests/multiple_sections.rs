use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_20_duplicate_entries as assertions;

use super::helpers::{
    add_deny_ban_entry, add_skip_entry, build_fixture_deny_toml, set_advisory_ignores,
    set_feature_entries,
};

fn deny_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn skip_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "version".to_owned(),
            toml::Value::String("1.0.0".to_owned()),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn ignore_entry(id: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("id".to_owned(), toml::Value::String(id.to_owned())),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn feature_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(vec![toml::Value::String("full".to_owned())]),
        ),
        (
            "allow".to_owned(),
            toml::Value::Array(vec![toml::Value::String("fs".to_owned())]),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

#[test]
fn warns_once_per_duplicated_entry_family() {
    let with_bans = add_deny_ban_entry(
        &add_deny_ban_entry(&build_fixture_deny_toml("service"), deny_entry("json5")),
        deny_entry("json5"),
    );
    let with_skip = add_skip_entry(
        &add_skip_entry(&with_bans, skip_entry("demo")),
        skip_entry("demo"),
    );
    let with_ignores = set_advisory_ignores(
        &with_skip,
        vec![
            ignore_entry("RUSTSEC-2020-0001"),
            ignore_entry("RUSTSEC-2020-0001"),
        ],
    );
    let deny = set_feature_entries(
        &with_ignores,
        vec![feature_entry("tokio"), feature_entry("tokio")],
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "duplicate advisory ignore entry",
                "`deny.toml` has duplicate advisory ignore `RUSTSEC-2020-0001`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "duplicate deny entry",
                "`deny.toml` has duplicate deny entry `json5`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "duplicate feature-ban entry",
                "`deny.toml` has duplicate `[[bans.features]]` for `tokio`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "duplicate skip entry",
                "`deny.toml` has duplicate skip entry `demo@1.0.0`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
