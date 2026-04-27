use super::super::check;
use cargo_toml_parser::parse;
use g3rs_garde_config_checks_assertions::dependency_present::rule as assertions;

#[test]
fn inventories_when_garde_dependency_present() {
    let cargo = parse("[workspace]\nmembers = []\n[dependencies]\ngarde = \"0.1\"\n")
        .expect("cargo fixture with garde dependency should parse");
    let mut results = Vec::new();
    check("Cargo.toml", &cargo, &mut results);

    assertions::assert_contains(
        &results,
        assertions::info(
            "garde dependency found",
            "garde is present in `Cargo.toml` for this workspace root. Garde-specific boundary checks are active.",
            "Cargo.toml",
        ),
    );
}
