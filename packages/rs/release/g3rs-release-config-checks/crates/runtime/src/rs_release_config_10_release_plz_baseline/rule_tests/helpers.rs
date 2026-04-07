use cargo_toml_parser::parse as parse_cargo;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;
use release_plz_toml_parser::parse as parse_release_plz;

const MINIMAL_CARGO: &str = "\
[package]
name = \"stub\"
version = \"1.0.0\"
edition = \"2024\"
";

pub(super) fn run_check(release_plz_toml: &str) -> Vec<G3CheckResult> {
    let cargo = parse_cargo(MINIMAL_CARGO).expect("cargo fixture should parse");
    let release_plz = parse_release_plz(release_plz_toml).expect("release-plz fixture should parse");
    let input = G3RsReleaseConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        release_plz_rel_path: Some("release-plz.toml".to_owned()),
        release_plz: Some(release_plz),
        cliff_rel_path: None,
        cliff: None,
    };
    let mut results = Vec::new();
    crate::rs_release_config_10_release_plz_baseline::check(&input, &mut results);
    results
}
