use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::additional_method_bans::rule as assertions;

#[test]
fn inventories_when_additional_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml())
        .expect("clippy fixture with all additional garde method bans should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        &assertions::info(
            "additional garde method bans present",
            "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
