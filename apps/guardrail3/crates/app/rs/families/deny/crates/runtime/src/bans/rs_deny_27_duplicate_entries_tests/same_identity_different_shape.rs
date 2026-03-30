use guardrail3_app_rs_family_deny_assertions::rs_deny_27_duplicate_entries as assertions;

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
                "duplicate skip entry",
                "`deny.toml` has duplicate skip entry `demo`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
