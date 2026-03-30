use guardrail3_app_rs_family_test_assertions::rs_test_03_runtime_assertions_split::{
    assert_inventory, assert_rule_files,
};

use super::{run_family_scoped, tempdir, write_file};

#[test]
fn scoped_sidecar_does_not_require_in_scope_assertions_file() {
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
        "#[cfg(test)]\n#[path = \"foo_tests/mod.rs\"]\nmod foo_tests;\npub fn value() -> u8 { 1 }\n",
    );
    write_file(
        root,
        "crates/runtime/src/foo.rs",
        "pub fn value() -> u8 { 1 }\n",
    );
    write_file(root, "crates/runtime/src/foo_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "crates/runtime/src/foo_tests/cases.rs",
        "use demo_assertions::foo::assert_runtime;\n#[test]\nfn uses_owned_assertions() { assert_runtime(); }\n",
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
        "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
    );

    let results = run_family_scoped(root, "crates/runtime/src/foo_tests/mod.rs");

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}
