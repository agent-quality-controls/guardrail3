#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_10_input_failures::{assert_reported, assert_message_starts_with, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, run_family_with_tool, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn malformed_source_fails_closed_as_input_failure() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "#[test]\nfn broken( {\n");

    let results = run_family(root);

    assert_rule_files(&results, vec!["src/lib.rs".to_owned()]
    );    assert_reported(&results, "src/lib.rs", None, Severity::Error, "test-family input failure");
    assert_message_starts_with(&results, "Failed to parse Rust source file for test-family analysis:");}

#[test]
fn malformed_cargo_manifest_fails_closed() {let fixture = tempdir();
    let root = fixture.path();

    write_file(root, "Cargo.toml", "[package]\nname = \"demo\"\nversion = ");

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]
    );    assert_reported(&results, "Cargo.toml", None, Severity::Error, "test-family input failure");
    assert_message_starts_with(&results, "Failed to parse Cargo.toml for test-family root discovery:");}

#[test]
fn malformed_nextest_config_is_reported_when_async_active() {let fixture = tempdir();
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

    assert_rule_files(&results, vec![".config/nextest.toml".to_owned()]
    );    assert_reported(&results, ".config/nextest.toml", None, Severity::Error, "test-family input failure");
    assert_message_starts_with(&results, "Failed to parse nextest config:");}

#[test]
fn malformed_mutants_config_is_reported_when_mutation_is_adopted() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = ");

    let results = run_family_with_tool(root, false);

    assert_rule_files(&results, vec![".cargo/mutants.toml".to_owned()]
    );    assert_reported(&results, ".cargo/mutants.toml", None, Severity::Error, "test-family input failure");
    assert_message_starts_with(&results, "Failed to parse mutants config:");}
