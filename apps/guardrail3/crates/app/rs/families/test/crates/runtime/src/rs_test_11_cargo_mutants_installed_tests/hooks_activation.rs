#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_11_cargo_mutants_installed::{
    Severity, assert_inventory, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family_with_tool, tempdir, write_file};

#[test]
fn hook_only_mutation_adoption_activates_the_rule() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        ".githooks/pre-commit",
        "#!/bin/sh\ncargo mutants --check\n",
    );

    let results = run_family_with_tool(root, false);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_reported(
        &results,
        "Cargo.toml",
        None,
        Severity::Warn,
        "cargo-mutants missing",
    );
    assert_inventory(&results, false);
}
