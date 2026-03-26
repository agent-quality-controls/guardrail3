use super::{finding, rule_files, run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn complete_nextest_timeouts_inventory_clean_async_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt\"] }\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );
    write_file(
        root,
        ".config/nextest.toml",
        "[profile.default]\nslow-timeout = { period = \"60s\" }\nleak-timeout = \"100ms\"\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-09"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-09");
    assert_eq!(finding.severity, Severity::Info);
    assert_eq!(finding.title, "nextest timeouts configured");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
}
