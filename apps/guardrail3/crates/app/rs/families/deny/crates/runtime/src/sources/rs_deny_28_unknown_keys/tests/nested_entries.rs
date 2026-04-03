use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::helpers::{
    add_skip_entry, build_fixture_deny_toml, set_advisory_ignores, set_feature_entries,
    set_license_exceptions,
};

#[test]
fn warns_on_unknown_nested_skip_ignore_exception_and_feature_keys() {
    let deny = set_feature_entries(
        &set_license_exceptions(
            &set_advisory_ignores(
                &add_skip_entry(
                    &build_fixture_deny_toml("service"),
                    toml::Value::Table(toml::map::Map::from_iter([
                        (
                            "crate".to_owned(),
                            toml::Value::String("serde@1.0.0".to_owned()),
                        ),
                        (
                            "reason".to_owned(),
                            toml::Value::String("good enough reason text".to_owned()),
                        ),
                        ("extra".to_owned(), toml::Value::Boolean(true)),
                    ])),
                ),
                vec![toml::Value::Table(toml::map::Map::from_iter([
                    (
                        "id".to_owned(),
                        toml::Value::String("RUSTSEC-2026-0001".to_owned()),
                    ),
                    (
                        "reason".to_owned(),
                        toml::Value::String("good enough reason text".to_owned()),
                    ),
                    ("extra".to_owned(), toml::Value::Boolean(true)),
                ]))],
            ),
            vec![toml::Value::Table(toml::map::Map::from_iter([
                ("name".to_owned(), toml::Value::String("ring".to_owned())),
                (
                    "allow".to_owned(),
                    toml::Value::Array(vec![toml::Value::String("ISC".to_owned())]),
                ),
                ("extra".to_owned(), toml::Value::Boolean(true)),
            ]))],
        ),
        vec![toml::Value::Table(toml::map::Map::from_iter([
            ("name".to_owned(), toml::Value::String("tokio".to_owned())),
            (
                "deny".to_owned(),
                toml::Value::Array(vec![toml::Value::String("full".to_owned())]),
            ),
            (
                "allow".to_owned(),
                toml::Value::Array(vec![toml::Value::String("rt-multi-thread".to_owned())]),
            ),
            ("extra".to_owned(), toml::Value::Boolean(true)),
        ]))],
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unknown advisories.ignore key",
                "`deny.toml` uses unknown `[[advisories.ignore]].extra` at index 0.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown bans.skip key",
                "`deny.toml` uses unknown `[[bans.skip]].extra` at index 0.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown feature-ban key",
                "`deny.toml` uses unknown `[[bans.features]].extra`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown licenses.exceptions key",
                "`deny.toml` uses unknown `[[licenses.exceptions]].extra` at index 0.",
                "deny.toml",
                false,
            ),
        ],
    );
}
