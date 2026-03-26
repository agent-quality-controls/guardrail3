use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn external_harness_reusing_owned_assertions_passes() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn public_surface() { prove_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-17").is_empty());
}

#[test]
fn external_harness_direct_assertion_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "#[test]\nfn public_surface() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-17");

    assert_eq!(
        rule_files(&results, "RS-TEST-17"),
        vec!["crates/runtime/tests/public_surface.rs".to_owned()]
    );
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "external harness asserts directly");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(2));
}
