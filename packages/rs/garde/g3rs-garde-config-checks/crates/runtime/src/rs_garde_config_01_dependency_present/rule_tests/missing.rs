use cargo_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_01_dependency_present as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;

#[test]
fn errors_when_garde_dependency_missing() {
    let cargo = parse("[workspace]\nmembers = []\n").expect("valid cargo");
    let input = G3RsGardeConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        clippy_rel_path: None,
        clippy: None,
    };

    let results = crate::run::check(&input);

    assertions::assert_contains(
        &results,
        assertions::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
}
