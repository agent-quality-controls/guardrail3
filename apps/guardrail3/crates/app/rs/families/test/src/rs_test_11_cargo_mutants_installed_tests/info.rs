use crate::test_support::{finding, run_family_with_tool, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn installed_tool_reports_info_for_an_adopted_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );

    let results = run_family_with_tool(root, true);

    assert_eq!(rule_files(&results, "RS-TEST-11"), vec!["Cargo.toml".to_owned()]);
    let finding = finding(&results, "RS-TEST-11");
    assert_eq!(finding.severity, Severity::Info);
    assert_eq!(finding.title, "cargo-mutants installed");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
    assert!(finding.inventory);
}
