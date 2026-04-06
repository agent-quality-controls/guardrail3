use cargo_toml_parser::parse as parse_cargo;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(cargo_toml: &str) -> Vec<G3CheckResult> {
    let cargo = parse_cargo(cargo_toml).expect("cargo fixture should parse");
    let input = G3RsReleaseConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        release_plz_rel_path: None,
        release_plz: None,
        cliff_rel_path: None,
        cliff: None,
    };
    let mut results = Vec::new();
    crate::rs_release_config_09_accidentally_publishable::check(&input, &mut results);
    results
}
