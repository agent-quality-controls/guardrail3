use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::core_method_bans::rule as assertions;

#[test]
fn warns_when_core_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "serde_json::from_reader",
    ))
    .expect("clippy fixture missing one core garde method ban should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing core garde method bans",
            "Missing core deserialization bans from `disallowed-methods`: serde_json::from_reader. Add these entries to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
