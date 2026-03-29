use guardrail3_app_rs_family_test_assertions::rs_test_07_real_proof_site::{
    assert_reported_file, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn same_named_function_in_other_assertions_module_does_not_count_as_proof() {
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
        "pub fn value() -> u8 {1}\n#[cfg(test)]\n#[path = \"foo_tests/mod.rs\"]\nmod foo_tests;\n",
    );
    write_file(
        root,
        "crates/runtime/src/foo.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(root, "crates/runtime/src/foo_tests/mod.rs", "mod proof;\n");
    write_file(
        root,
        "crates/runtime/src/foo_tests/proof.rs",
        "use demo_assertions::foo::prove;\n#[test]\nfn reuses_wrong_same_name() {prove();}\n",
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
        vec!["crates/runtime/src/foo_tests/proof.rs".to_owned()],
    );
    assert_reported_file(&results, "crates/runtime/src/foo_tests/proof.rs");
}
