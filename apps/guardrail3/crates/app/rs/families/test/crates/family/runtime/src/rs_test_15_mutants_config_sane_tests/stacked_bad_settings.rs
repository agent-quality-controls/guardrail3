use super::{run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn exclude_all_and_low_timeout_each_emit_a_warning() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        ".cargo/mutants.toml",
        "exclude_re = [\".*\"]\ntimeout_multiplier = 0.5\n",
    );

    let results = run_family(root);
    let findings = results
        .iter()
        .filter(|result| result.id == "RS-TEST-15")
        .collect::<Vec<_>>();

    assert_eq!(findings.len(), 2);
    assert!(findings.iter().all(|finding| finding.severity == Severity::Warn));
    assert!(findings.iter().all(|finding| !finding.inventory));
    assert!(findings
        .iter()
        .any(|finding| finding.title == "mutants config excludes everything"));
    assert!(findings
        .iter()
        .any(|finding| finding.title == "mutants timeout multiplier too low"));
    assert!(findings.iter().all(|finding| finding.file.as_deref() == Some(".cargo/mutants.toml")));
    assert!(findings.iter().all(|finding| finding.line.is_none()));
}
