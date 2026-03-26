use super::{finding, run_family, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn workspace_root_hook_does_not_duplicate_on_idle_standalone_root() {
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

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-14"),
        vec![".githooks/pre-commit".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-14");
    assert_eq!(finding.severity, Severity::Info);
    assert_eq!(finding.title, "mutation hook step present");
    assert_eq!(finding.file.as_deref(), Some(".githooks/pre-commit"));
    assert_eq!(finding.line, None);
}
