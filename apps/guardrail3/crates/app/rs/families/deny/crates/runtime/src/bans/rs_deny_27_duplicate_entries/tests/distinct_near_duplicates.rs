use super::super::{
    add_skip_entry, build_fixture_deny_toml, set_advisory_ignores, set_feature_entries,
};

fn skip_entry(name: &str, version: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "version".to_owned(),
            toml::Value::String(version.to_owned()),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn feature_entry(name: &str, deny_feature: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(vec![toml::Value::String(deny_feature.to_owned())]),
        ),
        (
            "allow".to_owned(),
            toml::Value::Array(vec![toml::Value::String("rt-multi-thread".to_owned())]),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

#[test]
fn does_not_warn_for_distinct_near_duplicate_skip_ignore_and_feature_entries() {
    let with_skip = add_skip_entry(
        &add_skip_entry(
            &build_fixture_deny_toml("service"),
            skip_entry("windows-sys", "0.59.0"),
        ),
        skip_entry("windows-sys", "0.60.0"),
    );
    let with_ignores = set_advisory_ignores(
        &with_skip,
        vec![
            toml::Value::String("RUSTSEC-2020-0001".to_owned()),
            toml::Value::String("RUSTSEC-2020-0002".to_owned()),
        ],
    );
    let deny = set_feature_entries(
        &with_ignores,
        vec![
            feature_entry("tokio", "full"),
            feature_entry("tokio-util", "codec"),
        ],
    );
    let results = super::super::run_check(&deny);

    assert!(
        results.is_empty(),
        "distinct near-duplicate identities should not warn: {results:#?}"
    );
}
