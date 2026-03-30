use guardrail3_app_rs_family_test_assertions::rs_test_13_mutants_profile_present::{
    Severity, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn missing_mutants_profile_is_reported_when_mutation_is_adopted() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_reported(
        &results,
        "Cargo.toml",
        None,
        Severity::Error,
        "profile.mutants missing",
    );
    assert_inventory(&results, false);
}
