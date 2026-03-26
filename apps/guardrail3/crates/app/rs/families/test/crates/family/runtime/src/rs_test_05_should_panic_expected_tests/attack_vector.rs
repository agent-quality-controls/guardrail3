use guardrail3_domain_report::Severity;

use super::{finding, run_family, rule_files, tempdir, write_file};

#[test]
fn bare_should_panic_is_reported_on_the_test_file() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic]\nfn panics_without_expected_message() { panic!(\"boom\"); }\n",
    );

    let results = run_family(root);
    assert_eq!(rule_files(&results, "RS-TEST-05"), vec!["tests/panic.rs".to_owned()]);
    let finding = finding(&results, "RS-TEST-05");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "should_panic missing expected string");
    assert_eq!(finding.file.as_deref(), Some("tests/panic.rs"));
    assert_eq!(finding.line, Some(2));
}

#[test]
fn empty_expected_string_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic(expected = \"\")]\nfn panics_with_empty_expected_message() { panic!(\"boom\"); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-05");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "should_panic missing expected string");
    assert_eq!(finding.file.as_deref(), Some("tests/panic.rs"));
    assert_eq!(finding.line, Some(2));
}

#[test]
fn non_string_expected_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic(expected = 1)]\nfn panics_with_non_string_expected() { panic!(\"boom\"); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-05");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "should_panic missing expected string");
    assert_eq!(finding.file.as_deref(), Some("tests/panic.rs"));
    assert_eq!(finding.line, Some(2));
}
