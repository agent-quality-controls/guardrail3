use super::{finding, rule_files, run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn adopted_workspace_root_does_not_require_profile_for_idle_standalone_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/adopted\"]\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");
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
}
