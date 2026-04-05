use clippy_toml_parser::parse;
use g3_garde_content_checks_assertions::rs_garde_03_extractor_type_bans as assertions;
use g3_garde_content_checks_types::G3GardeClippyBanChecksInput;

#[test]
fn warns_when_extractor_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-types",
        "axum::extract::Multipart",
    ))
    .expect("valid clippy");
    let input = G3GardeClippyBanChecksInput {
        clippy_rel_path: "clippy.toml".to_owned(),
        clippy,
    };

    let results = crate::run::check_clippy_bans(&input);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing garde extractor bans",
            "Missing extractor type bans from `disallowed-types`: axum::extract::Multipart. Add these entries to `disallowed-types` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
