use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn generic_test_support_passes() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/demo/runtime\", \"crates/demo/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/demo/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn public_surface() { prove_runtime(); }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/src/lib.rs",
        "pub fn prove_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn fixture_name(name: &str) -> String { name.to_owned() }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-18").is_empty());
}

#[test]
fn test_support_importing_runtime_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/demo/runtime\", \"crates/demo/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/demo/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn public_surface() { prove_runtime(); }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/src/lib.rs",
        "pub fn prove_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use demo_runtime::value;\npub fn fixture_value() -> u8 { value() }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-18");

    assert_eq!(
        rule_files(&results, "RS-TEST-18"),
        vec!["test_support/src/lib.rs".to_owned()]
    );
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test_support imports local component crate");
    assert_eq!(finding.file.as_deref(), Some("test_support/src/lib.rs"));
    assert_eq!(finding.line, Some(1));
}

#[test]
fn test_support_direct_runtime_call_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/demo/runtime\", \"crates/demo/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/demo/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn public_surface() { prove_runtime(); }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\ntest_support = { path = \"../../../test_support\" }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/src/lib.rs",
        "pub fn prove_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn fixture_value() -> u8 { demo_runtime::value() }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-18");

    assert_eq!(
        rule_files(&results, "RS-TEST-18"),
        vec!["test_support/src/lib.rs".to_owned()]
    );
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "test_support calls local component crate");
    assert_eq!(finding.file.as_deref(), Some("test_support/src/lib.rs"));
    assert_eq!(finding.line, None);
}
