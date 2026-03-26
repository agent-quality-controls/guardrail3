use super::{finding, run_family, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn missing_mutants_profile_is_reported_when_mutation_is_adopted() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-13"),
        vec!["Cargo.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-13");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "profile.mutants missing");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
    assert!(!finding.inventory);
}
