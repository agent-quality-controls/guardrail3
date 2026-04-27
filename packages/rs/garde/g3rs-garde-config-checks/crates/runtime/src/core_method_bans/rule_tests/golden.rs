use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::core_method_bans::rule as assertions;

#[test]
fn inventories_when_all_core_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml())
        .expect("clippy fixture with all core garde method bans should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        assertions::info(
            "core garde method bans present",
            "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
