use super::super::check;
use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::extractor_type_bans::rule as assertions;

#[test]
fn warns_when_extractor_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-types",
        "axum::extract::Multipart",
    ))
    .expect("clippy fixture missing one garde extractor ban should parse");
    let mut results = Vec::new();
    check("clippy.toml", &clippy, &mut results);

    assertions::assert_contains(
        &results,
        &assertions::warn(
            "missing garde extractor bans",
            "Missing extractor type bans from `disallowed-types`: axum::extract::Multipart. Add these entries to `disallowed-types` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
