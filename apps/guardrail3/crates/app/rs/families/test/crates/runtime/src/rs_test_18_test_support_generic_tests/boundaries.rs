use guardrail3_domain_report::Severity;

#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_18_test_support_generic::{assert_reported, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn generic_test_support_passes() {let fixture = tempdir();
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

    assert_rule_quiet(&results);}

#[test]
fn test_support_importing_runtime_is_reported() {let fixture = tempdir();
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
    assert_rule_files(&results, vec!["test_support/src/lib.rs".to_owned()]
    );
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(1),
        Severity::Error,
        "test_support imports local component crate",
    );}

#[test]
fn test_support_direct_runtime_call_is_reported() {let fixture = tempdir();
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
    assert_rule_files(&results, vec!["test_support/src/lib.rs".to_owned()]
    );
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        None,
        Severity::Error,
        "test_support calls local component crate",
    );}

#[test]
fn test_support_route_construction_imports_are_reported() {let fixture = tempdir();
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
    write_file(root, "crates/runtime/src/lib.rs", "pub fn value() -> u8 {1}\n");
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
        "use guardrail3_app_rs_family_mapper::FamilyMapper;\nuse guardrail3_app_rs_placement;\npub fn cargo_route(tree: &ProjectTree, scope: &Scope, selected: &Selected) {let _ = guardrail3_app_rs_placement::collect(tree); FamilyMapper::new(tree, scope, None, selected, None);}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "test_support/src/lib.rs",
        Some(1),
        Severity::Error,
        "test_support imports route construction infrastructure",
    );}
