use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_04_reqwest_json_ban as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;

#[test]
fn warns_when_reqwest_json_ban_missing() {
    let clippy = parse(&super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "reqwest::Response::json",
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
            "missing reqwest garde ban",
            "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
            "clippy.toml",
        ),
    );
}
