use guardrail3_app_rs_family_test_assertions::rs_test_02_owned_sidecar_shape::{
    assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn sidecar_fixture_rust_is_ignored() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\npub fn value() -> u8 { 1 }\n",
    );
    write_file(root, "src/lib_tests/mod.rs", "mod cases;\n");
    write_file(
        root,
        "src/lib_tests/cases.rs",
        "#[test]\nfn proves() { assert_eq!(super::helpers::value(), 1); }\n",
    );
    write_file(
        root,
        "src/lib_tests/fixtures/sample.rs",
        "#[cfg(test)]\nmod garbage { #[test] fn broken() {} }\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}
