use super::{finding, run_family, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn missing_config_is_ignored_without_adoption_and_required_for_hook_only_adoption() {
    let dormant = tempdir();
    write_file(
        dormant.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let dormant_results = run_family(dormant.path());
    assert!(rule_files(&dormant_results, "RS-TEST-12").is_empty());

    let adopted = tempdir();
    write_file(
        adopted.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(adopted.path(), ".githooks/pre-commit", "#!/bin/sh\ncargo mutants --list\n");

    let adopted_results = run_family(adopted.path());
    assert_eq!(
        rule_files(&adopted_results, "RS-TEST-12"),
        vec![".cargo/mutants.toml".to_owned()]
    );
    let finding = finding(&adopted_results, "RS-TEST-12");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "mutants config missing");
    assert_eq!(finding.file.as_deref(), Some(".cargo/mutants.toml"));
    assert_eq!(finding.line, None);
    assert!(!finding.inventory);
}
