use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::extractor_type_bans::rule as assertions;

#[test]
fn inventories_when_all_extractor_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml())
        .expect("clippy fixture with all garde extractor bans should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        &assertions::info(
            "garde extractor bans present",
            "All required Axum extractor bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
