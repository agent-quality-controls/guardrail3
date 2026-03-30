use guardrail3_app_rs_family_test_assertions::rs_test_17_external_harnesses_use_assertions::{
    Severity, assert_external_harness_direct_assertion, assert_inventory, assert_reported,
    assert_rule_files,
};

use super::{run_family, tempdir, write_file};

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
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);

    assert_rule_files(
        &results,
        vec!["crates/runtime/tests/public_surface.rs".to_owned()],
    );
    assert_inventory(&results, true);
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
        "#[test]\nfn public_surface() {assert_eq!(demo_runtime::value(), 1);}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_external_harness_direct_assertion(&results);
}

#[test]
fn external_harness_local_assertion_helper_is_reported() {
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
        "fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "crates/runtime/tests/public_surface.rs",
        Some(3),
        Severity::Error,
        "external harness asserts directly",
    );
}

#[test]
fn external_harness_transitive_owned_assertion_wrapper_is_reported() {
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
        "use demo_assertions::prove_runtime;\nfn assert_expected() { prove_runtime(); }\nfn assert_runtime() { assert_expected(); }\n#[test]\nfn public_surface() { assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "crates/runtime/tests/public_surface.rs",
        Some(5),
        Severity::Error,
        "external harness asserts directly",
    );
}

#[test]
fn external_harness_function_pointer_alias_wrapper_is_reported() {
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
        "use demo_assertions::prove_runtime;\nfn assert_runtime() { prove_runtime(); }\n#[test]\nfn public_surface() { let prove = assert_runtime; prove(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "crates/runtime/tests/public_surface.rs",
        Some(4),
        Severity::Error,
        "external harness asserts directly",
    );
}

#[test]
fn external_harness_associated_function_wrapper_is_reported() {
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
        "use demo_assertions::prove_runtime;\nstruct Probe;\nimpl Probe { fn assert_runtime() { prove_runtime(); } }\n#[test]\nfn public_surface() { Probe::assert_runtime(); }\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub fn prove_runtime() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "crates/runtime/tests/public_surface.rs",
        Some(5),
        Severity::Error,
        "external harness asserts directly",
    );
}
