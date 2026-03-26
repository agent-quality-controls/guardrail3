use super::{finding, rule_files, run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn sane_mutants_config_is_reported_as_inventory() {
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
        "timeout_multiplier = 2.0\nexclude_re = [\"^src/legacy/\"]\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-15"),
        vec![".cargo/mutants.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-15");
    assert_eq!(finding.severity, Severity::Info);
    assert_eq!(finding.title, "mutants config looks sane");
    assert_eq!(finding.file.as_deref(), Some(".cargo/mutants.toml"));
    assert_eq!(finding.line, None);
    assert!(finding.inventory);
}
