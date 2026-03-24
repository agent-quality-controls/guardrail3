use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    add_skip_entry, canonical_deny_toml_service, config_facts, set_advisory_ignores,
    set_feature_entries, set_license_exceptions,
};
use super::super::check;

#[test]
fn warns_on_unknown_nested_skip_ignore_exception_and_feature_keys() {
    let deny = set_feature_entries(
        &set_license_exceptions(
            &set_advisory_ignores(
                &add_skip_entry(
                    &canonical_deny_toml_service(),
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
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual = results
        .iter()
        .map(|result| (result.title.clone(), result.message.clone()))
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "unknown advisories.ignore key".to_owned(),
            "`deny.toml` uses unknown `[[advisories.ignore]].extra` at index 0.".to_owned(),
        ),
        (
            "unknown bans.skip key".to_owned(),
            "`deny.toml` uses unknown `[[bans.skip]].extra` at index 0.".to_owned(),
        ),
        (
            "unknown feature-ban key".to_owned(),
            "`deny.toml` uses unknown `[[bans.features]].extra`.".to_owned(),
        ),
        (
            "unknown licenses.exceptions key".to_owned(),
            "`deny.toml` uses unknown `[[licenses.exceptions]].extra` at index 0.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.file.as_deref() == Some("deny.toml")
    }));
}
