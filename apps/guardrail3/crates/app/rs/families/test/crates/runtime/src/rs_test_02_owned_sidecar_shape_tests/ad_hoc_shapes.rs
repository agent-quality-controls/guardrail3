use guardrail3_domain_report::Severity;

#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_02_owned_sidecar_shape::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn ad_hoc_cfg_test_module_declaration_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "#[cfg(test)]\nmod helper_tests;\n");
    write_file(
        root,
        "src/lib_tests/mod.rs",
        "#[test]\nfn helper() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/lib.rs",
        Some(1),
        Severity::Error,
        "ad hoc cfg(test) module declaration",
    );
}

#[test]
fn ad_hoc_src_tests_tree_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/tests/helper.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/tests",
        None,
        Severity::Error,
        "ad hoc src/tests tree",
    );
}

#[test]
fn missing_sidecar_mod_rs_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib_tests/helper.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/lib_tests",
        None,
        Severity::Error,
        "sidecar directory missing mod.rs",
    );
}

#[test]
fn flat_tests_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib_tests.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/lib_tests.rs",
        None,
        Severity::Error,
        "flat sidecar test file",
    );
}

#[test]
fn flat_test_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib_test.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/lib_test.rs",
        None,
        Severity::Error,
        "flat sidecar test file",
    );
}

#[test]
fn flat_tests_rs_file_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/tests.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/tests.rs",
        None,
        Severity::Error,
        "flat sidecar test file",
    );
}

#[test]
fn orphaned_sidecar_harness_is_reported() {
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
        "crates/runtime/src/lib_tests/mod.rs",
        "#[test]\nfn stray() {assert!(true);}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "crates/runtime/src/lib_tests/mod.rs",
        None,
        Severity::Error,
        "orphaned sidecar harness",
    );
}
