use guardrail3_app_rs_family_test_assertions::rs_test_10_input_failures::{
    Severity, assert_message_starts_with, assert_reported, assert_rule_files,
};

use super::{run_family, run_family_with_tool, tempdir, write_file};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[test]
fn malformed_source_fails_closed_as_input_failure() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "#[test]\nfn broken( {\n");

    let results = run_family(root);

    assert_rule_files(&results, vec!["src/lib.rs".to_owned()]);
    assert_reported(
        &results,
        "src/lib.rs",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(
        &results,
        "Failed to parse Rust source file for test-family analysis:",
    );
}

#[test]
fn malformed_cargo_manifest_fails_closed() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(root, "Cargo.toml", "[package]\nname = \"demo\"\nversion = ");

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_reported(
        &results,
        "Cargo.toml",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(
        &results,
        "Failed to parse Cargo.toml for test-family root discovery:",
    );
}

#[test]
fn malformed_nextest_config_is_reported_when_async_active() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = {version = \"1\", features = [\"macros\", \"rt\"]}\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() {assert!(true);}\n",
    );
    write_file(
        root,
        ".config/nextest.toml",
        "[profile.default]\nslow-timeout = ",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec![".config/nextest.toml".to_owned()]);
    assert_reported(
        &results,
        ".config/nextest.toml",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(&results, "Failed to parse nextest config:");
}

#[test]
fn malformed_mutants_config_is_reported_when_mutation_is_adopted() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = ");

    let results = run_family_with_tool(root, false);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]);
    assert_reported(
        &results,
        ".cargo/mutants.toml",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(&results, "Failed to parse mutants config:");
}

#[cfg(unix)]
#[test]
fn unreadable_pre_commit_hook_fails_closed_for_hook_only_adoption() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");

    let hook_path = root.join(".githooks/pre-commit");
    let mut permissions = std::fs::metadata(&hook_path).expect("metadata").permissions();
    permissions.set_mode(0o000);
    std::fs::set_permissions(&hook_path, permissions.clone()).expect("chmod 000");

    let results = run_family(root);

    permissions.set_mode(0o644);
    std::fs::set_permissions(&hook_path, permissions).expect("chmod restore");

    assert_rule_files(&results, vec![".githooks/pre-commit".to_owned()]);
    assert_reported(
        &results,
        ".githooks/pre-commit",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(
        &results,
        "Failed to read active hook surface for test-family mutation detection:",
    );
}

#[cfg(unix)]
#[test]
fn unreadable_pre_commit_d_hook_fails_closed_for_hook_only_adoption() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        ".githooks/pre-commit.d/run-mutants",
        "#!/bin/sh\ncargo mutants\n",
    );

    let hook_path = root.join(".githooks/pre-commit.d/run-mutants");
    let mut permissions = std::fs::metadata(&hook_path).expect("metadata").permissions();
    permissions.set_mode(0o000);
    std::fs::set_permissions(&hook_path, permissions.clone()).expect("chmod 000");

    let results = run_family(root);

    permissions.set_mode(0o644);
    std::fs::set_permissions(&hook_path, permissions).expect("chmod restore");

    assert_rule_files(&results, vec![".githooks/pre-commit.d/run-mutants".to_owned()]);
    assert_reported(
        &results,
        ".githooks/pre-commit.d/run-mutants",
        None,
        Severity::Error,
        "test-family input failure",
    );
    assert_message_starts_with(
        &results,
        "Failed to read active hook surface for test-family mutation detection:",
    );
}
