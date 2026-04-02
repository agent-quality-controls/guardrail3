use guardrail3_app_rs_family_test_assertions::rs_test_12_mutants_toml_exists::{
    Severity, assert_inventory, assert_reported, assert_rule_files, assert_rule_quiet,
};

use super::{run_family, tempdir, write_file};

#[test]
fn missing_config_is_ignored_without_adoption_and_required_for_hook_only_adoption() {
    let dormant = tempdir();
    write_file(
        dormant.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let dormant_results = run_family(dormant.path());
    assert_rule_quiet(&dormant_results);

    let adopted = tempdir();
    write_file(
        adopted.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        adopted.path(),
        ".githooks/pre-commit",
        "#!/bin/sh\ncargo mutants --list\n",
    );

    let adopted_results = run_family(adopted.path());
    assert_rule_files(&adopted_results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &adopted_results,
        ".cargo/mutants.toml",
        None,
        Severity::Error,
        "mutants config missing",
    );
    assert_inventory(&adopted_results, false);
}
