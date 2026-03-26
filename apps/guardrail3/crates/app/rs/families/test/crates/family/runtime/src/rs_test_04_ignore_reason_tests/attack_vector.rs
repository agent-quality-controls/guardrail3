use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn bare_ignore_is_reported_on_the_test_file() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/slow.rs",
        "#[test]\n#[ignore]\nfn waits_for_service() {}\n",
    );

    let results = run_family(root);
    assert_eq!(
        rule_files(&results, "RS-TEST-04"),
        vec!["tests/slow.rs".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-04");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "ignored test lacks reason");
    assert_eq!(finding.file.as_deref(), Some("tests/slow.rs"));
    assert_eq!(finding.line, Some(2));
}
