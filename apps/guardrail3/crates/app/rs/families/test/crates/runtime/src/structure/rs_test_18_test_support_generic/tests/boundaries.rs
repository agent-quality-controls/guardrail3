use guardrail3_app_rs_family_test_assertions::rs_test_18_test_support_generic::{
    Severity, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn generic_test_support_passes() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn fixture_name(name: &str) -> String {name.to_owned()}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["test_support/src/lib.rs".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn crates_test_support_layout_is_checked_too() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"crates/test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "crates/test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "crates/test_support/src/lib.rs",
        "pub fn fixture_name(name: &str) -> String {name.to_owned()}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["crates/test_support/src/lib.rs".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn test_support_importing_runtime_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use demo_runtime::value;\npub fn fixture_value() -> u8 {value()}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["test_support/src/lib.rs".to_owned()]);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(1),
        Severity::Error,
        "test_support imports local component crate",
    );
}

#[test]
fn test_support_direct_runtime_call_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn fixture_value() -> u8 {demo_runtime::value()}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["test_support/src/lib.rs".to_owned()]);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        None,
        Severity::Error,
        "test_support calls local component crate",
    );
}

#[test]
fn test_support_route_construction_imports_are_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use guardrail3_app_rs_family_mapper::FamilyMapper;\nuse guardrail3_app_rs_placement;\npub fn cargo_route(tree: &ProjectTree, scope: &Scope, selected: &Selected) {let _ = guardrail3_app_rs_structure::collect(tree); FamilyMapper::new(tree, scope, None, selected, None);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(1),
        Severity::Error,
        "test_support imports route construction infrastructure",
    );
}

#[test]
fn test_support_importing_placement_route_infra_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use guardrail3_app_rs_placement as placement;\npub fn cargo_route() { let _ = placement::collect; }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(1),
        Severity::Error,
        "test_support imports route construction infrastructure",
    );
}

#[test]
fn test_support_fully_qualified_family_mapper_call_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn cargo_route() { let _ = guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, scope, None, selected, None); }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        None,
        Severity::Error,
        "test_support builds routed family input",
    );
}

#[test]
fn test_support_fully_qualified_placement_call_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn cargo_route() { let _ = guardrail3_app_rs_placement::collect(todo!()); }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        None,
        Severity::Error,
        "test_support builds routed family input",
    );
}

#[test]
fn test_support_family_mapper_function_pointer_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "pub fn cargo_route() { let make = guardrail3_app_rs_family_mapper::FamilyMapper::new; let _ = make; }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        None,
        Severity::Error,
        "test_support builds routed family input",
    );
}

#[test]
fn test_support_transitive_semantic_helper_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "fn select_rule(results: &[guardrail3_domain_report::CheckResult]) -> usize { results.iter().filter(|result| result.id()()()() == \"RS-DEMO-01\").count() }\npub fn error_count(results: &[guardrail3_domain_report::CheckResult]) -> usize { select_rule(results) }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(2),
        Severity::Error,
        "test_support exports semantic finding helper",
    );
}

#[test]
fn test_support_check_result_method_selector_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use guardrail3_domain_report::{CheckResult, Severity};\npub fn all_errors(results: &[CheckResult]) -> bool { results.iter().all(|result| result.severity() == Severity::Error) }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(2),
        Severity::Error,
        "test_support exports semantic finding helper",
    );
}

#[test]
fn test_support_check_result_type_alias_selector_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "type Finding = guardrail3_domain_report::CheckResult;\nuse guardrail3_domain_report::Severity;\npub fn all_errors(results: &[Finding]) -> bool { results.iter().all(|result| result.severity() == Severity::Error) }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(3),
        Severity::Error,
        "test_support exports semantic finding helper",
    );
}

#[test]
fn test_support_transitive_method_selector_helper_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\n",
    );
    write_file(
        root,
        "crates/runtime/Cargo.toml",
        "[package]\nname = \"demo_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = {path = \"../assertions\"}\ntest_support = {path = \"../../test_support\"}\n",
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
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\ntest_support = {path = \"../../test_support\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "test_support/Cargo.toml",
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "test_support/src/lib.rs",
        "use guardrail3_domain_report::{CheckResult, Severity};\nfn select_all_errors(results: &[CheckResult]) -> bool { results.iter().all(|result| result.severity() == Severity::Error) }\npub fn all_errors(results: &[CheckResult]) -> bool { select_all_errors(results) }\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(3),
        Severity::Error,
        "test_support exports semantic finding helper",
    );
}
