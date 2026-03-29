use guardrail3_app_rs_family_test_assertions::rs_test_02_owned_sidecar_shape::{
    assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn owned_sidecar_directory_shape_passes_cleanly() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib.rs",
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n",
    );
    write_file(
        root,
        "src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() {assert!(true);}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}

#[test]
fn nested_package_root_sidecar_shape_passes_cleanly() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/domain\"]\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/runtime/Cargo.toml",
        "[package]\nname = \"domain_runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndomain_assertions = {path = \"../assertions\"}\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/runtime/src/lib.rs",
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/runtime/src/lib_tests/mod.rs",
        "#[test]\nfn owned_sidecar() {assert!(true);}\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/assertions/Cargo.toml",
        "[package]\nname = \"domain_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndomain_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/assertions/src/lib.rs",
        "pub fn assert_runtime() {assert_eq!(domain_runtime::value(), 1);}\n",
    );

    let results = run_family(root);

    assert_rule_files(
        &results,
        vec!["apps/backend/crates/domain/Cargo.toml".to_owned()],
    );
    assert_inventory(&results, true);
}
