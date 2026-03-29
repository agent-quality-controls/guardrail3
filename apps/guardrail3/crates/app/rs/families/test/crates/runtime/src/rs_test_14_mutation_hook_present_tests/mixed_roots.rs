#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_14_mutation_hook_present::{
    Severity, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn workspace_root_hook_does_not_duplicate_on_idle_standalone_root() {
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
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");

    let results = run_family(root);

    assert_rule_files(&results, vec![".githooks/pre-commit".to_owned()]);
    assert_reported(
        &results,
        ".githooks/pre-commit",
        None,
        Severity::Info,
        "mutation hook step present",
    );
}
