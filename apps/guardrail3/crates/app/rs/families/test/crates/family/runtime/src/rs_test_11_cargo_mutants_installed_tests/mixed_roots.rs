use super::{finding, rule_files, run_family_with_tool, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn workspace_root_adoption_does_not_activate_idle_standalone_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/adopted\"]\n\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        "crates/adopted/Cargo.toml",
        "[package]\nname = \"adopted\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "standalone/idle/Cargo.toml",
        "[package]\nname = \"idle\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");

    let results = run_family_with_tool(root, false);

    assert_eq!(
        rule_files(&results, "RS-TEST-11"),
        vec!["Cargo.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-11");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "cargo-mutants missing");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
}
