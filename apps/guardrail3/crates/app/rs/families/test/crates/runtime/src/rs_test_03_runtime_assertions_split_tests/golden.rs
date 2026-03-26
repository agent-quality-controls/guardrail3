#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_03_runtime_assertions_split::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn runtime_assertions_split_with_black_box_harness_stays_clean() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\npub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib_tests/mod.rs",
        "#[test]\nfn internal() {assert!(true);}\n",
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

    assert_rule_quiet(&results);
}

#[test]
fn nested_package_root_with_runtime_assertions_split_stays_clean() {
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
        "#[test]\nfn internal() {assert!(true);}\n",
    );
    write_file(
        root,
        "apps/backend/crates/domain/crates/runtime/tests/public_surface.rs",
        "use domain_assertions::lib::assert_runtime;\n#[test]\nfn public_surface() {assert_runtime();}\n",
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

    assert_rule_quiet(&results);
}
