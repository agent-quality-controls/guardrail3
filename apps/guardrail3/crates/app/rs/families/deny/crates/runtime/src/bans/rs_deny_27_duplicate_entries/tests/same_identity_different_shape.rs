use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_20_duplicate_entries as assertions;

use super::helpers::{add_skip_entry, build_fixture_deny_toml, set_advisory_ignores};

fn skip_name_table(name: &str) -> toml::Value {
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

fn skip_crate_table(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        (
            "crate".to_owned(),
            toml::Value::String(format!("{name}@1.0.0")),
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
        skip_crate_table("demo"),
    );
    let with_skip = add_skip_entry(&with_skip, skip_name_table("demo"));
    let deny = set_advisory_ignores(
        &with_skip,
        vec![
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
    let results = super::helpers::run_check(&deny);

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
                "`deny.toml` has duplicate skip entry `demo@1.0.0`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
