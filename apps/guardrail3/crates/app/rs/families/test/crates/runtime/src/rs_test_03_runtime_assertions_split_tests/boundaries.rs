use guardrail3_domain_report::Severity;

use super::{finding, rule_files, run_family, tempdir, write_file};

#[test]
fn root_local_sidecar_harness_is_reported_instead_of_being_silently_skipped() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "pub fn value() -> u8 { 1 }\n");
    write_file(
        root,
        "src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() { assert_eq!(crate::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "test harness outside runtime/assertions split"
    );
    assert_eq!(finding.file.as_deref(), Some("src/lib_tests/mod.rs"));
    assert_eq!(finding.line, None);
}

#[test]
fn root_local_external_harness_is_reported_instead_of_being_silently_skipped() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "pub fn value() -> u8 { 1 }\n");
    write_file(
        root,
        "tests/public_surface.rs",
        "#[test]\nfn public_surface() { assert_eq!(demo::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "test harness outside runtime/assertions split"
    );
    assert_eq!(finding.file.as_deref(), Some("tests/public_surface.rs"));
    assert_eq!(finding.line, None);
}

#[test]
fn missing_assertions_crate_for_external_harness_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
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

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "assertions crate missing");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/assertions/Cargo.toml")
    );
    assert_eq!(finding.line, None);
}

#[test]
fn runtime_depends_on_assertions_at_normal_scope_is_reported() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_assertions = { path = \"../assertions\" }\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "runtime depends on assertions at normal scope"
    );
    assert_eq!(finding.file.as_deref(), Some("crates/runtime/Cargo.toml"));
    assert_eq!(finding.line, None);
}

#[test]
fn runtime_missing_assertions_dev_dependency_is_reported() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "runtime missing assertions dev-dependency");
    assert_eq!(finding.file.as_deref(), Some("crates/runtime/Cargo.toml"));
    assert_eq!(finding.line, None);
}

#[test]
fn assertions_missing_runtime_dependency_is_reported() {
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
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "assertions missing runtime dependency");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/assertions/Cargo.toml")
    );
    assert_eq!(finding.line, None);
}

#[test]
fn sidecar_missing_owned_assertions_module_is_reported() {
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
        "crates/runtime/src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() { assert!(true); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "sidecar missing owned assertions module");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/src/lib_tests/mod.rs")
    );
    assert_eq!(finding.line, None);
}

#[test]
fn sidecar_imports_sibling_production_module_is_reported() {
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
    write_file(root, "crates/runtime/src/other.rs", "pub fn helper() {}\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/mod.rs",
        "use crate::other;\n#[test]\nfn owned_sidecar() { other::helper(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "sidecar imports sibling production module");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/src/lib_tests/mod.rs")
    );
    assert_eq!(finding.line, Some(1));
}

#[test]
fn sidecar_super_imports_sibling_production_module_is_reported() {
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
        "crates/runtime/src/lint_support.rs",
        "pub const EXPECTED: usize = 1;\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib_tests/mod.rs",
        "#[path = \"cases.rs\"]\nmod cases;\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use super::super::super::lint_support::EXPECTED;\n#[test]\nfn owned_sidecar() { assert_eq!(EXPECTED, 1); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.title, "sidecar escapes owned module boundary");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/src/lib_tests/cases.rs")
    );
    assert_eq!(finding.line, Some(1));
}

#[test]
fn assertions_module_reaches_local_private_code_is_reported() {
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
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use crate::internal;\nfn internal() {}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "assertions module reaches local private code"
    );
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/assertions/src/lib.rs")
    );
    assert_eq!(finding.line, Some(1));
}

#[test]
fn assertions_module_importing_route_infra_is_reported() {
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
        "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use guardrail3_app_rs_family_mapper::FamilyMapper;\npub fn assert_runtime() { let _ = FamilyMapper::new; assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "assertions module imports route construction infrastructure"
    );
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/assertions/src/lib.rs")
    );
    assert_eq!(finding.line, Some(1));
}

#[test]
fn sidecar_calling_crate_root_helper_is_reported() {
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
        "pub fn forbidden_helper() {}\npub mod foo;\n",
    );
    write_file(
        root,
        "crates/runtime/src/foo.rs",
        "#[cfg(test)]\n#[path = \"foo_tests/mod.rs\"]\nmod foo_tests;\n",
    );
    write_file(root, "crates/runtime/src/foo_tests/mod.rs", "mod support;\n");
    write_file(
        root,
        "crates/runtime/src/foo_tests/support.rs",
        "use super::super::super::forbidden_helper;\n#[test]\nfn calls_helper() { forbidden_helper(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(root, "crates/assertions/src/lib.rs", "pub mod foo;\n");
    write_file(
        root,
        "crates/assertions/src/foo.rs",
        "pub fn prove() { assert!(true); }\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(
        rule_files(&results, "RS-TEST-03"),
        vec!["crates/runtime/src/foo_tests/support.rs".to_owned()]
    );
    assert_eq!(finding.title, "sidecar escapes owned module boundary");
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/src/foo_tests/support.rs")
    );
}

#[test]
fn external_harness_crate_boundary_is_reported() {
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
        "use crate::glue;\n#[test]\nfn public_surface() { let _ = glue(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "external harness reaches private runtime glue"
    );
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(1));
}

#[test]
fn external_harness_self_boundary_stays_quiet() {
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
        "mod glue { pub fn helper() -> u8 { 1 } }\nuse self::glue::helper;\n#[test]\nfn public_surface() { assert_eq!(helper(), 1); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);

    assert!(
        results.iter().all(|result| result.id != "RS-TEST-03"),
        "external harness-local `self::` helpers should stay quiet"
    );
}

#[test]
fn external_harness_super_boundary_is_reported() {
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
        "use super::glue;\n#[test]\nfn public_surface() { let _ = glue(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);
    let finding = finding(&results, "RS-TEST-03");

    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(
        finding.title,
        "external harness reaches private runtime glue"
    );
    assert_eq!(
        finding.file.as_deref(),
        Some("crates/runtime/tests/public_surface.rs")
    );
    assert_eq!(finding.line, Some(1));
}
