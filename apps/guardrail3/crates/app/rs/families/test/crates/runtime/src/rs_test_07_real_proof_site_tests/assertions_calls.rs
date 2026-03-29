use guardrail3_app_rs_family_test_assertions::rs_test_07_real_proof_site::{
    assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn owned_assertions_crate_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::prove_runtime;\n#[test]\nfn reuses_owned_assertions() {prove_runtime();}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
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
fn owned_assertions_alias_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions as assertions;\n#[test]\nfn reuses_owned_assertions() {assertions::prove_runtime();}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
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
fn owned_assertions_glob_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::*;\n#[test]\nfn reuses_owned_assertions() {prove_runtime();}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
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
fn owned_assertions_grouped_import_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions::proofs::{prove_runtime};\n#[test]\nfn reuses_owned_assertions() {prove_runtime();}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(root, "crates/assertions/src/lib.rs", "pub mod proofs;\n");
    write_file(
        root,
        "crates/assertions/src/proofs.rs",
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
fn macro_defined_owned_assertions_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions as assertions;\n#[test]\nfn reuses_macro_defined_owned_assertions() {assertions::proofs::assert_rule_quiet::<u8>(&[]);}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        r#"
#[macro_export]
macro_rules! define_rule_assertions {
    ($rule_id:literal) => {
        pub fn assert_rule_quiet<T>(_results: &[T]) {
            assert!(true, "{}", $rule_id);
        }
    };
}

pub mod proofs;
"#,
    );
    write_file(
        root,
        "crates/assertions/src/proofs.rs",
        "crate::define_rule_assertions!(\"RS-DEMO-01\");\n",
    );

    let results = run_family(root);

    assert_rule_files(
        &results,
        vec!["crates/runtime/tests/public_surface.rs".to_owned()],
    );
    assert_inventory(&results, true);
}

#[test]
fn macro_defined_result_assertions_call_counts_as_real_proof_site() {
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
        "crates/runtime/tests/public_surface.rs",
        "use demo_assertions as assertions;\n#[test]\nfn reuses_macro_defined_owned_assertions() {assertions::proofs::assert_findings::<u8>(&[], &[]);}\n",
    );
    write_file(
        root,
        "crates/runtime/src/lib.rs",
        "pub fn value() -> u8 {1}\n",
    );
    write_file(
        root,
        "crates/assertions/Cargo.toml",
        "[package]\nname = \"demo_assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = {path = \"../runtime\"}\n",
    );
    write_file(
        root,
        "crates/assertions/src/lib.rs",
        r#"
#[macro_export]
macro_rules! define_result_assertions {
    ($rule_id:literal) => {
        pub fn assert_findings<T>(_results: &[T], _expected: &[T]) {
            assert!(true, "{}", $rule_id);
        }
    };
}

pub mod proofs;
"#,
    );
    write_file(
        root,
        "crates/assertions/src/proofs.rs",
        "crate::define_result_assertions!(\"RS-DEMO-01\");\n",
    );

    let results = run_family(root);

    assert_rule_files(
        &results,
        vec!["crates/runtime/tests/public_surface.rs".to_owned()],
    );
    assert_inventory(&results, true);
}
