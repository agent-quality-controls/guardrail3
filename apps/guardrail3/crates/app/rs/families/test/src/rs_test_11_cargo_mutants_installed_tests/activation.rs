use crate::test_support::{finding, run_family_with_tool, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn missing_tool_is_ignored_without_mutation_adoption_and_reported_when_adopted() {
    let dormant = tempdir();
    write_file(
        dormant.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        dormant.path(),
        "tests/basic.rs",
        "#[test]\nfn runs() { assert!(true); }\n",
    );

    let dormant_results = run_family_with_tool(dormant.path(), false);
    assert!(rule_files(&dormant_results, "RS-TEST-11").is_empty());

    let adopted = tempdir();
    write_file(
        adopted.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        adopted.path(),
        "tests/basic.rs",
        "#[test]\nfn runs() { assert!(true); }\n",
    );

    let adopted_results = run_family_with_tool(adopted.path(), false);
    assert_eq!(rule_files(&adopted_results, "RS-TEST-11"), vec!["Cargo.toml".to_owned()]);
    let finding = finding(&adopted_results, "RS-TEST-11");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "cargo-mutants missing");
    assert_eq!(finding.file.as_deref(), Some("Cargo.toml"));
    assert_eq!(finding.line, None);
}
