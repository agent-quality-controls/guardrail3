use cargo_toml_parser::parse as parse_cargo;
use cliff_toml_parser::parse as parse_cliff;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const MINIMAL_CARGO: &str = "\
[package]
name = \"stub\"
version = \"1.0.0\"
edition = \"2024\"
";

pub(super) fn run_check(cliff_toml: &str) -> Vec<G3CheckResult> {
    let cargo = parse_cargo(MINIMAL_CARGO).expect("cargo fixture should parse");
    let cliff = parse_cliff(cliff_toml).expect("cliff fixture should parse");
    let input = G3RsReleaseConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        release_plz_rel_path: None,
        release_plz: None,
        cliff_rel_path: Some("cliff.toml".to_owned()),
        cliff: Some(cliff),
    };
    let mut results = Vec::new();
    crate::rs_release_config_11_cliff_baseline::check(&input, &mut results);
    results
}
