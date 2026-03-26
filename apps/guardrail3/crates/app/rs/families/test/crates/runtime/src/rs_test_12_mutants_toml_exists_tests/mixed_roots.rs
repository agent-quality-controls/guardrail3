#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_12_mutants_toml_exists::{
    assert_inventory, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn workspace_root_adoption_does_not_require_mutants_config_for_idle_standalone_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/adopted\"]\n\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        "crates/adopted/Cargo.toml",
        "[package]\nname = \"adopted\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "standalone/idle/Cargo.toml",
        "[package]\nname = \"idle\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &results,
        ".cargo/mutants.toml",
        None,
        Severity::Warn,
        "mutants config missing",
    );
    assert_inventory(&results, false);
}
