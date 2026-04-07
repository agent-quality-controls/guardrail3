use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_03_extractor_type_bans as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;

#[test]
fn warns_when_extractor_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-types",
        "axum::extract::Multipart",
    ))
    .expect("valid clippy");
    let input = G3RsGardeConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse("[workspace]\nmembers = []\n")
            .expect("minimal cargo fixture should parse"),
        clippy_rel_path: Some("clippy.toml".to_owned()),
        clippy: Some(clippy),
    };

    let results = crate::run::check(&input);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing garde extractor bans",
            "Missing extractor type bans from `disallowed-types`: axum::extract::Multipart. Add these entries to `disallowed-types` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
