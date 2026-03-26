use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn test_without_assertion_macro_or_owned_assertions_call_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/proof.rs",
        "fn helper() {}\n#[test]\nfn touches_code_only() { helper(); }\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-07"),
        vec!["tests/proof.rs".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-07");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "test lacks real proof site");
    assert_eq!(finding.file.as_deref(), Some("tests/proof.rs"));
    assert_eq!(finding.line, Some(3));
}

#[test]
fn result_return_without_proof_is_reported() {
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
        "crates/runtime/tests/public_surface.rs",
        "#[test]\nfn returns_result_without_assertions() -> Result<(), ()> { Ok(()) }\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
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
    let finding = finding(&results, "RS-TEST-07");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "test lacks real proof site");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(2));
}

#[test]
fn shadowed_owned_assertions_call_is_reported() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn shadowed_call_does_not_count() {\n    let prove_runtime = || {};\n    prove_runtime();\n}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
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
    let finding = finding(&results, "RS-TEST-07");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "test lacks real proof site");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(3));
}

#[test]
fn name_heuristic_does_not_count_as_proof() {
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
        "crates/runtime/tests/public_surface.rs",
        "#[test]\nfn verify_runtime() {}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
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
    let finding = finding(&results, "RS-TEST-07");

    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "test lacks real proof site");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(2));
}
