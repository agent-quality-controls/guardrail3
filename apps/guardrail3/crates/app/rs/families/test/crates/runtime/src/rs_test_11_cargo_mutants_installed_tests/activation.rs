#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_11_cargo_mutants_installed::{
    assert_inventory, assert_missing_cargo_mutants, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family_with_tool, tempdir, write_file};
#[test]
fn missing_tool_is_ignored_without_mutation_adoption_and_reported_when_adopted() {
    let dormant = tempdir();
    write_file(
        dormant.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        dormant.path(),
        "tests/basic.rs",
        "#[test]\nfn runs() {assert!(true);}\n",
    );

    let dormant_results = run_family_with_tool(dormant.path(), false);
    assert_rule_quiet(&dormant_results);

    let adopted = tempdir();
    write_file(
        adopted.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        adopted.path(),
        "tests/basic.rs",
        "#[test]\nfn runs() {assert!(true);}\n",
    );

    let adopted_results = run_family_with_tool(adopted.path(), false);
    assert_rule_files(&adopted_results, vec!["Cargo.toml".to_owned()]);
    assert_missing_cargo_mutants(&adopted_results);
    assert_inventory(&adopted_results, false);
}
