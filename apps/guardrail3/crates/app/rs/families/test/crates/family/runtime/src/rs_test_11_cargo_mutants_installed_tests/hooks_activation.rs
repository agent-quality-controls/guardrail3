use super::{finding, run_family_with_tool, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn hook_only_mutation_adoption_activates_the_rule() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants --check\n");

    let results = run_family_with_tool(root, false);

    assert_eq!(rule_files(&results, "RS-TEST-11"), vec!["Cargo.toml".to_owned()]);
    let finding = finding(&results, "RS-TEST-11");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "cargo-mutants missing");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
    assert!(!finding.inventory);
}
