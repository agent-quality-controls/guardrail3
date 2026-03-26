use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn literal_vs_literal_assertion_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    assert_eq!(1, 1);\n}\n",
    );

    let results = run_family(root);
    assert_eq!(
        rule_files(&results, "RS-TEST-06"),
        vec!["tests/asserts.rs".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-06");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "tautological assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/asserts.rs"));
    assert_eq!(finding.line, Some(3));
}

#[test]
fn assert_ne_literal_vs_literal_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    assert_ne!(1, 1);\n}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-06");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "tautological assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/asserts.rs"));
    assert_eq!(finding.line, Some(3));
}

#[test]
fn debug_assert_eq_parenthesized_literals_are_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    debug_assert_eq!((1), (1));\n}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-06");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "tautological assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/asserts.rs"));
    assert_eq!(finding.line, Some(3));
}

#[test]
fn debug_assert_ne_parenthesized_literals_are_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    debug_assert_ne!((2), (2));\n}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-06");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "tautological assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/asserts.rs"));
    assert_eq!(finding.line, Some(3));
}
