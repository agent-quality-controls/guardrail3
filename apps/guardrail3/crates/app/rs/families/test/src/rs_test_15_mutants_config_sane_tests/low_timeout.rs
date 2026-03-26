use crate::test_support::{finding, run_family, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn low_timeout_multiplier_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 0.5\n");

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-15"),
        vec![".cargo/mutants.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-15");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "mutants timeout multiplier too low");
    assert_eq!(finding.file.as_deref(), Some(".cargo/mutants.toml"));
    assert_eq!(finding.line, None);
    assert!(!finding.inventory);
}
