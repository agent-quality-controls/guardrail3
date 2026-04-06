use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_04_reqwest_json_ban as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigClippyBanChecksInput;

#[test]
fn warns_when_reqwest_json_ban_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "reqwest::Response::json",
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
            "missing reqwest garde ban",
            "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
