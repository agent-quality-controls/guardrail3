use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::additional_method_bans::rule as assertions;

#[test]
fn warns_when_additional_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "serde_qs::from_bytes",
    ))
    .expect("clippy fixture missing one additional garde method ban should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing additional garde method bans",
            "Missing additional deserialization bans from `disallowed-methods`: serde_qs::from_bytes. Add these entries to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
