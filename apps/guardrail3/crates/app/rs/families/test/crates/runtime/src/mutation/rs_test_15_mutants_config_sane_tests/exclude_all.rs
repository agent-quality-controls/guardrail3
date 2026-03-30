use guardrail3_app_rs_family_test_assertions::rs_test_15_mutants_config_sane::{
    Severity, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn exclude_all_pattern_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "exclude_re = [\".*\"]\n");

    let results = run_family(root);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &results,
        ".cargo/mutants.toml",
        None,
        Severity::Warn,
        "mutants config excludes everything",
    );
    assert_inventory(&results, false);
}
