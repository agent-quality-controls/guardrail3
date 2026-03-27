use guardrail3_domain_report::Severity;

use super::super::{add_skip_entry, build_fixture_deny_toml, set_advisory_ignores};

fn skip_table(name: &str) -> toml::Value {
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

#[test]
fn warns_when_same_identity_is_duplicated_across_supported_shapes() {
    let with_skip = add_skip_entry(
        &build_fixture_deny_toml("service"),
        toml::Value::String("demo".to_owned()),
    );
    let with_skip = add_skip_entry(&with_skip, skip_table("demo"));
    let deny = set_advisory_ignores(
        &with_skip,
        vec![
            toml::Value::String("RUSTSEC-2020-0001".to_owned()),
            toml::Value::Table(toml::map::Map::from_iter([
                (
                    "id".to_owned(),
                    toml::Value::String("RUSTSEC-2020-0001".to_owned()),
                ),
                (
                    "reason".to_owned(),
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ])),
        ],
    );
    let results = super::super::run_check(&deny);

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-27"
            && result.severity == Severity::Warn
            && result.title == "duplicate skip entry"
            && result.message == "`deny.toml` has duplicate skip entry `demo`."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-27"
            && result.severity == Severity::Warn
            && result.title == "duplicate advisory ignore entry"
            && result.message == "`deny.toml` has duplicate advisory ignore `RUSTSEC-2020-0001`."
    }));
}
