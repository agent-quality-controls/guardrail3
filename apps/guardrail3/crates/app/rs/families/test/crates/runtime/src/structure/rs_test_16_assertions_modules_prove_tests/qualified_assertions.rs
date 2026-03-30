use guardrail3_app_rs_family_test_assertions::rs_test_16_assertions_modules_prove::{
    Severity, assert_has_finding, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn same_named_proof_in_other_module_does_not_make_wrapper_pass() {
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
        "use demo_assertions::foo::prove;\n#[test]\nfn public_surface() {prove();}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        "pub mod foo;\npub mod bar;\n",
    );
    write_file(
        root,
        "crates/assertions/src/foo.rs",
        "pub fn prove() {let _ = demo_runtime::value();}\n",
    );
    write_file(
        root,
        "crates/assertions/src/bar.rs",
        "pub fn prove() {assert_eq!(demo_runtime::value(), 1);}\n",
    );

    let results = run_family(root);
    assert_rule_files(
        &results,
        vec![
            "crates/assertions/src/bar.rs".to_owned(),
            "crates/assertions/src/foo.rs".to_owned(),
        ],
    );
    assert_has_finding(
        &results,
        "crates/assertions/src/bar.rs",
        Severity::Info,
        true,
    );
    assert_has_finding(
        &results,
        "crates/assertions/src/foo.rs",
        Severity::Error,
        false,
    );
}
