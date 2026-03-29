use guardrail3_app_rs_family_test_assertions::rs_test_03_runtime_assertions_split::{
    assert_error_reported, assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn root_local_sidecar_harness_is_reported_instead_of_being_silently_skipped() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "pub fn value() -> u8 {1}\n");
    write_file(
        root,
        "src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() {assert_eq!(crate::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "assertions/Cargo.toml",
        None,
        "assertions crate missing",
    );
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
    write_file(root, "src/lib.rs", "pub fn value() -> u8 {1}\n");
    write_file(
        root,
        "tests/public_surface.rs",
        "#[test]\nfn public_surface() {assert_eq!(demo::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "assertions/Cargo.toml",
        None,
        "assertions crate missing",
    );
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
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "#[test]\nfn public_surface() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/assertions/Cargo.toml",
        None,
        "assertions crate missing",
    );
}

#[test]
fn family_container_without_parent_cargo_manifest_is_still_a_valid_runtime_assertions_split() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod tests;\n\npub fn value() -> u8 { 1 }\n",
    );
    write_file(root, "crates/runtime/src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use demo_assertions::lib::assert_runtime;\n\n#[test]\nfn owned_sidecar() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "test_support/src/lib.rs", "pub fn marker() {}\n");

    let results = run_family(root);
    assert_rule_files(&results, vec!["crates/runtime/Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn root_runtime_package_with_sibling_assertions_is_a_valid_runtime_assertions_split() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"assertions\"}\ntest_support = {path = \"test_support\"}\n",
    );
    write_file(
        root,
        "src/lib.rs",
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n\npub fn value() -> u8 { 1 }\n",
    );
    write_file(root, "src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "src/lib_tests/cases.rs",
        "use demo_assertions::assert_runtime;\n\n#[test]\nfn owned_sidecar() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "tests/public_surface.rs",
        "use demo_assertions::assert_runtime;\n\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"..\"}\ntest_support = {path = \"../test_support\"}\n",
    );
    write_file(
        root,
        "assertions/src/lib.rs",
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "test_support/src/lib.rs", "pub fn marker() {}\n");

    let results = run_family(root);
    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn runtime_depends_on_assertions_at_normal_scope_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"support/report\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_assertions = {path = \"../assertions\"}\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/Cargo.toml",
        None,
        "runtime depends on assertions at normal scope",
    );
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
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/Cargo.toml",
        None,
        "runtime missing assertions dev-dependency",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
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
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/assertions/Cargo.toml",
        None,
        "assertions missing runtime dependency",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() {assert!(true);}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/src/lib_tests/mod.rs",
        None,
        "sidecar missing owned assertions module",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/other.rs", "pub fn helper() {}\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/mod.rs",
        "use crate::other;\n#[test]\nfn owned_sidecar() {other::helper();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/src/lib_tests/mod.rs",
        Some(1),
        "sidecar imports sibling production module",
    );
}

#[test]
fn assertions_module_calling_runtime_check_test_tree_is_reported() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_project_tree = {path = \"../../domain/project-tree\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use demo_runtime as runtime;\nuse guardrail3_domain_project_tree::ProjectTree;\npub fn assert_runtime(tree: &ProjectTree) {let _ = runtime::check_test_tree(tree); assert!(true);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/assertions/src/lib.rs",
        None,
        "assertions module orchestrates family execution",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use crate::internal;\nfn internal() {}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/assertions/src/lib.rs",
        Some(1),
        "assertions module reaches local private code",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use guardrail3_app_rs_family_mapper::FamilyMapper;\npub fn assert_runtime() {let _ = FamilyMapper::new; assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/assertions/src/lib.rs",
        Some(1),
        "assertions module imports route construction infrastructure",
    );
}

#[test]
fn assertions_module_fully_qualified_family_mapper_call_is_reported() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn public_surface() {prove_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() { let _ = guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, scope, None, selected, None); assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    assert_error_reported(
        &results,
        "crates/assertions/src/lib.rs",
        None,
        "assertions module builds routed family input",
    );
}

#[test]
fn assertions_module_importing_shared_report_model_is_allowed() {
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_report = { package = \"guardrail3-domain-report\", path = \"../../support/report\" }\n",
    );
    write_file(
        root,
        "support/report/Cargo.toml",
        "[package]\nname = \"guardrail3-domain-report\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "support/report/src/lib.rs",
        "#[derive(Clone, Copy, PartialEq, Eq, Debug)]\npub enum Severity { Error }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "use guardrail3_domain_report::Severity;\npub fn assert_runtime() { assert_eq!(Severity::Error, Severity::Error); assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn sidecar_importing_shared_report_model_is_still_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"support/report\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use guardrail3_domain_report::Severity;\n#[test]\nfn owned_sidecar() { assert_eq!(Severity::Error, Severity::Error); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_report = { package = \"guardrail3-domain-report\", path = \"../../support/report\" }\n",
    );
    write_file(
        root,
        "support/report/Cargo.toml",
        "[package]\nname = \"guardrail3-domain-report\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "support/report/src/lib.rs",
        "#[derive(Clone, Copy, PartialEq, Eq, Debug)]\npub enum Severity { Error }\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/src/lib_tests/cases.rs",
        Some(1),
        "sidecar imports disallowed local crate",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
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
    write_file(
        root,
        "crates/runtime/src/foo_tests/mod.rs",
        "mod support;\n",
    );
    write_file(
        root,
        "crates/runtime/src/foo_tests/support.rs",
        "use super::super::super::forbidden_helper;\n#[test]\nfn calls_helper() {forbidden_helper();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(root, "crates/assertions/src/lib.rs", "pub mod foo;\n");
    write_file(
        root,
        "crates/assertions/src/foo.rs",
        "pub fn prove() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/src/foo_tests/support.rs",
        Some(1),
        "sidecar escapes owned module boundary",
    );
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "mod glue {pub fn helper() -> u8 {1}}\nuse self::glue::helper;\n#[test]\nfn public_surface() {assert_eq!(helper(), 1);}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
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
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/tests/public_surface.rs",
        "use super::glue;\n#[test]\nfn public_surface() {let _ = glue();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_error_reported(
        &results,
        "crates/runtime/tests/public_surface.rs",
        Some(1),
        "external harness reaches private runtime glue",
    );
}
