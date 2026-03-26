use guardrail3_domain_report::Severity;

use super::{finding, run_family, tempdir, write_file};

#[test]
fn ad_hoc_cfg_test_module_declaration_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "#[cfg(test)]\nmod helper_tests;\n");
    write_file(root, "src/lib_tests/mod.rs", "#[test]\nfn helper() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "ad hoc cfg(test) module declaration");
    assert_eq!(finding.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(finding.line, Some(1));
}

#[test]
fn ad_hoc_src_tests_tree_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/tests/helper.rs", "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "ad hoc src/tests tree");
    assert_eq!(finding.file.as_deref(), Some("src/tests"));
    assert_eq!(finding.line, None);
}

#[test]
fn missing_sidecar_mod_rs_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib_tests/helper.rs", "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "sidecar directory missing mod.rs");
    assert_eq!(finding.file.as_deref(), Some("src/lib_tests"));
    assert_eq!(finding.line, None);
}

#[test]
fn flat_tests_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib_tests.rs", "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "flat sidecar test file");
    assert_eq!(finding.file.as_deref(), Some("src/lib_tests.rs"));
    assert_eq!(finding.line, None);
}

#[test]
fn flat_test_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib_test.rs", "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "flat sidecar test file");
    assert_eq!(finding.file.as_deref(), Some("src/lib_test.rs"));
    assert_eq!(finding.line, None);
}

#[test]
fn flat_tests_rs_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/tests.rs", "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "flat sidecar test file");
    assert_eq!(finding.file.as_deref(), Some("src/tests.rs"));
    assert_eq!(finding.line, None);
}

#[test]
fn orphaned_sidecar_harness_is_reported() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "crates/demo/runtime/src/lib_tests/mod.rs", "#[test]\nfn stray() { assert!(true); }\n");
    write_file(
        root,
        "crates/demo/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-02");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "orphaned sidecar harness");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/demo/runtime/src/lib_tests/mod.rs")
    );
    assert_eq!(finding.line, None);
}
