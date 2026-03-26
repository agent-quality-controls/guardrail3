use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn proof_bearing_export_in_assertions_module_passes() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/demo/runtime\", \"crates/demo/assertions\"]\n",
    );
    write_file(
        root,
        "crates/demo/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/tests/public_surface.rs",
        "use demo_assertions::foo::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(root, "crates/demo/assertions/src/lib.rs", "pub mod foo;\n");
    write_file(
        root,
        "crates/demo/assertions/src/foo.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-16").is_empty());
}

#[test]
fn thin_wrapper_assertions_module_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/demo/runtime\", \"crates/demo/assertions\"]\n",
    );
    write_file(
        root,
        "crates/demo/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/demo/runtime/tests/public_surface.rs",
        "use demo_assertions::foo::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(root, "crates/demo/assertions/src/lib.rs", "pub mod foo;\n");
    write_file(
        root,
        "crates/demo/assertions/src/foo.rs",
        "pub fn assert_runtime() { let _ = demo_runtime::value(); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-16");

    assert_eq!(
        rule_files(&results, "RS-TEST-16"),
        vec!["crates/demo/assertions/src/foo.rs".to_owned()]
    );
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "assertions module lacks proof-bearing export"
    );
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/demo/assertions/src/foo.rs")
    );
    assert_eq!(finding.line, Some(1));
}
