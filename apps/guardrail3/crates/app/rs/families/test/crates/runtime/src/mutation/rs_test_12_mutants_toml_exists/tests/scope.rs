use guardrail3_app_rs_family_test_assertions::rs_test_12_mutants_toml_exists::{
    assert_rule_files, assert_rule_quiet,
};

use super::{run_family_scoped, tempdir, write_file};

#[test]
fn scoped_member_root_does_not_wake_sibling_mutation_roots() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/adopted\"]\n\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");
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

    let results = run_family_scoped(root, "crates/adopted/Cargo.toml");

    assert_rule_files(
        &results,
        vec!["crates/adopted/.cargo/mutants.toml".to_owned()],
    );
}

#[test]
fn scoped_generated_target_file_does_not_activate_mutation_rules() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");
    write_file(
        root,
        "target/debug/build/demo/out/private.rs",
        "#[test]\nfn generated() { assert!(true); }\n",
    );

    let results = run_family_scoped(root, "target/debug/build/demo/out/private.rs");

    assert_rule_quiet(&results);
}
