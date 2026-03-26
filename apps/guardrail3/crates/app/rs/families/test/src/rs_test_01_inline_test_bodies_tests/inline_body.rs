use guardrail3_domain_report::Severity;

use crate::test_support::{finding, run_family, tempdir, write_file};

#[test]
fn inline_cfg_test_body_hits_owned_source_file() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib.rs",
        "#[cfg(test)]\nmod lib_tests { #[test] fn proves_nothing() { assert!(true); } }\n",
    );

    let results = run_family(root);

    let finding = finding(&results, "RS-TEST-01");
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "inline cfg(test) body in src");
    assert_eq!(finding.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(finding.line, Some(1));
}
