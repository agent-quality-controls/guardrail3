use cargo_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_01_dependency_present as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;

#[test]
fn inventories_when_garde_dependency_present() {
    let cargo = parse(
        "[workspace]\nmembers = []\n[dependencies]\ngarde = \"0.1\"\n",
    )
    .expect("valid cargo");
    let input = G3RsGardeConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        clippy_rel_path: None,
        clippy: None,
    };

    let results = crate::run::check(&input);

    assertions::assert_contains(
        &results,
        assertions::info(
            "garde dependency found",
            "garde is present in `Cargo.toml` for this workspace root. Garde-specific boundary checks are active.",
            "Cargo.toml",
        ),
    );
}
