use crate::test_support::{finding, run_family, run_family_with_tool, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

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

    assert_eq!(
        rule_files(&results, "RS-TEST-10"),
        vec!["src/lib.rs".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-10");
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test-family input failure");
    assert_eq!(finding.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(finding.line, None);
    assert!(
        finding
            .message
            .starts_with("Failed to parse Rust source file for test-family analysis:")
    );
}

#[test]
fn malformed_cargo_manifest_fails_closed() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(root, "Cargo.toml", "[package]\nname = \"demo\"\nversion = ");

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-10"),
        vec!["Cargo.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-10");
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test-family input failure");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
    assert!(
        finding
            .message
            .starts_with("Failed to parse Cargo.toml for test-family root discovery:")
    );
}

#[test]
fn malformed_nextest_config_is_reported_when_async_active() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt\"] }\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );
    write_file(root, ".config/nextest.toml", "[profile.default]\nslow-timeout = ");

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-10"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-10");
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test-family input failure");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
    assert!(
        finding
            .message
            .starts_with("Failed to parse nextest config:")
    );
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

    assert_eq!(
        rule_files(&results, "RS-TEST-10"),
        vec![".cargo/mutants.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-10");
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test-family input failure");
    assert_eq!(finding.file.as_deref(), Some(".cargo/mutants.toml"));
    assert_eq!(finding.line, None);
    assert!(
        finding
            .message
            .starts_with("Failed to parse mutants config:")
    );
}
