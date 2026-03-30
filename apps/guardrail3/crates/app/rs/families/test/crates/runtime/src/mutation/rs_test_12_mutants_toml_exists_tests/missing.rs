use guardrail3_app_rs_family_test_assertions::rs_test_12_mutants_toml_exists::{
    Severity, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn missing_mutants_config_is_reported_when_mutation_is_adopted() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &results,
        ".cargo/mutants.toml",
        None,
        Severity::Error,
        "mutants config missing",
    );
    assert_inventory(&results, false);
}
