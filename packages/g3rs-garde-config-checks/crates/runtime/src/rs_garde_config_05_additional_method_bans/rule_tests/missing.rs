use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_05_additional_method_bans as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigClippyBanChecksInput;

#[test]
fn warns_when_additional_bans_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "serde_qs::from_bytes",
    ))
    .expect("valid clippy");
    let input = G3RsGardeConfigClippyBanChecksInput {
        clippy_rel_path: "clippy.toml".to_owned(),
        clippy,
    };

    let results = crate::run::check_clippy_bans(&input);

    assertions::assert_contains(
        &results,
        assertions::warn(
            "missing additional garde method bans",
            "Missing additional deserialization bans from `disallowed-methods`: serde_qs::from_bytes. Add these entries to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
