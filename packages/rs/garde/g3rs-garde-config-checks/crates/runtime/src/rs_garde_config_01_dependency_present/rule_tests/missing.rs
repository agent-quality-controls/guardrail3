use cargo_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_01_dependency_present::rule as assertions;
use super::super::check;

#[test]
fn errors_when_garde_dependency_missing() {
    let cargo = parse("[workspace]\nmembers = []\n")
        .expect("cargo fixture without garde dependency should parse");
    let mut results = Vec::new();
    check("Cargo.toml", &cargo, &mut results);

    assertions::assert_contains(
        &results,
        assertions::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
}
