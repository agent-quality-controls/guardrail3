use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_04_reqwest_json_ban::rule as assertions;
use super::super::check;

#[test]
fn warns_when_reqwest_json_ban_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "reqwest::Response::json",
    ))
    .expect("clippy fixture missing reqwest json ban should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing reqwest garde ban",
            "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
