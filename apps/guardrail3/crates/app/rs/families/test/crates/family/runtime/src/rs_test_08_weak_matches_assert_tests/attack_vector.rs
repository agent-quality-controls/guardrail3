use guardrail3_domain_report::Severity;

use super::{finding, run_family, rule_files, tempdir, write_file};

#[test]
fn wildcard_payload_match_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/matches.rs",
        "#[test]\nfn hides_the_payload() {\n    let value = Some(1);\n    assert!(matches!(value, Some(_)));\n}\n",
    );

    let results = run_family(root);
    assert_eq!(rule_files(&results, "RS-TEST-08"), vec!["tests/matches.rs".to_owned()]);
    let finding = finding(&results, "RS-TEST-08");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "weak matches assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/matches.rs"));
    assert_eq!(finding.line, Some(4));
}

#[test]
fn debug_assert_wildcard_match_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/matches.rs",
        "#[test]\nfn hides_the_payload() {\n    let value = Some((1, 2));\n    debug_assert!((matches!(value, Some((_, 2)))));\n}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-08");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "weak matches assertion");
    assert_eq!(finding.file.as_deref(), Some("tests/matches.rs"));
    assert_eq!(finding.line, Some(4));
}
