use cargo_toml_parser::parse;
use g3_garde_content_checks_assertions::rs_garde_01_dependency_present as assertions;
use g3_garde_content_checks_types::G3GardeDependencyCheckInput;

#[test]
fn errors_when_garde_dependency_missing() {
    let cargo = parse("[workspace]\nmembers = []\n").expect("valid cargo");
    let input = G3GardeDependencyCheckInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
    };

    let results = crate::run::check_dependency_present(&input);

    assertions::assert_contains(
        &results,
        assertions::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
}
