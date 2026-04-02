use guardrail3_app_rs_family_test_assertions::rs_test_11_cargo_mutants_installed::assert_rule_files;

use super::{run_family_with_tool, tempdir, write_file};

#[test]
fn workspace_root_hook_adoption_reaches_workspace_member_but_not_idle_standalone_root() {
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

    let results = run_family_with_tool(root, false);

    assert_rule_files(
        &results,
        vec![
            "Cargo.toml".to_owned(),
            "crates/adopted/Cargo.toml".to_owned(),
        ],
    );
}
