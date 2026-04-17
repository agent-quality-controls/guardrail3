use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_04_reqwest_json_ban::rule as assertions;
use super::super::check;

#[test]
fn inventories_when_reqwest_json_ban_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml())
        .expect("clippy fixture with reqwest json ban should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        assertions::info(
            "reqwest garde ban present",
            "`reqwest::Response::json` is banned in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
