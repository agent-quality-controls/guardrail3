use guardrail3_app_rs_family_test_assertions::rs_test_13_mutants_profile_present::{
    Severity, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn adopted_workspace_root_does_not_require_profile_for_idle_standalone_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/adopted\"]\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");
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

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_reported(
        &results,
        "Cargo.toml",
        None,
        Severity::Warn,
        "profile.mutants missing",
    );
}
