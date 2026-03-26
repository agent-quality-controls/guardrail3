use guardrail3_domain_report::Severity;

#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_02_owned_sidecar_shape::{assert_reported, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn path_included_source_file_without_backing_sidecar_is_reported() {let fixture = tempdir();
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
        "tests/public_surface.rs",
        "#[test]\nfn public_surface() {assert!(true);}\n",
    );

    let results = run_family(root);
    assert_reported(&results, "src/lib.rs", Some(1), Severity::Error, "ad hoc cfg(test) module declaration");}