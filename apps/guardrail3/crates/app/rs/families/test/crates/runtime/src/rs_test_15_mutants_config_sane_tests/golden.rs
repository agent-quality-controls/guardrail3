#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_15_mutants_config_sane::{
    Severity, assert_inventory, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn sane_mutants_config_is_reported_as_inventory() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        ".cargo/mutants.toml",
        "timeout_multiplier = 2.0\nexclude_re = [\"^src/legacy/\"]\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &results,
        ".cargo/mutants.toml",
        None,
        Severity::Info,
        "mutants config looks sane",
    );
    assert_inventory(&results, true);
}
