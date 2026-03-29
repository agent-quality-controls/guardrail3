use guardrail3_app_rs_family_test_assertions::rs_test_16_assertions_modules_prove::{
    Severity, assert_error_results_are_error, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn proof_bearing_export_in_assertions_module_passes() {
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
        "use demo_assertions::foo::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
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
        "pub fn assert_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["crates/assertions/src/foo.rs".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn thin_wrapper_assertions_module_is_reported() {
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
        "use demo_assertions::foo::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
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
        "pub fn assert_runtime() {let _ = demo_runtime::value();}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["crates/assertions/src/foo.rs".to_owned()]);
    assert_reported(
        &results,
        "crates/assertions/src/foo.rs",
        Some(1),
        Severity::Error,
        "assertions module lacks proof-bearing export",
    );
}

#[test]
fn sidecar_result_shape_assertion_is_reported() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use demo_assertions::error_results;\nuse guardrail3_app_rs_family_test::Severity;\n#[test]\nfn semantic_sidecar() {let results = vec![]; let errors = error_results(&results, \"RS-DEMO-01\"); assert!(errors.iter().all(|result| result.severity == Severity::Error));}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn error_results<'a>(_results: &'a [guardrail3_domain_report::CheckResult], _rule_id: &str) -> Vec<&'a guardrail3_domain_report::CheckResult> {Vec::new()}\npub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_error_results_are_error(&results, "RS-DEMO-01");
}

#[test]
fn sidecar_result_shape_panic_is_reported() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use demo_assertions::error_results;\nuse guardrail3_app_rs_family_test::Severity;\n#[test]\nfn semantic_sidecar() {let results = vec![]; let errors = error_results(&results, \"RS-DEMO-01\"); if errors.iter().any(|result| result.severity != Severity::Error) { panic!(\"wrong severity\"); }}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn error_results<'a>(_results: &'a [guardrail3_domain_report::CheckResult], _rule_id: &str) -> Vec<&'a guardrail3_domain_report::CheckResult> {Vec::new()}\npub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_error_results_are_error(&results, "RS-DEMO-01");
}

#[test]
fn sidecar_indexed_result_id_assertion_is_reported() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/lib_tests/cases.rs",
        "use demo_assertions::error_results;\n#[test]\nfn semantic_sidecar() {let results = vec![]; let errors = error_results(&results, \"RS-DEMO-01\"); assert_eq!(errors[0].id, \"RS-DEMO-01\");}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\nguardrail3_domain_report = {path = \"../../../../domain/report\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn error_results<'a>(_results: &'a [guardrail3_domain_report::CheckResult], _rule_id: &str) -> Vec<&'a guardrail3_domain_report::CheckResult> {Vec::new()}\npub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_error_results_are_error(&results, "RS-DEMO-01");
}
