use guardrail3_app_rs_family_test_assertions::rs_test_01_inline_test_bodies::{
    assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn owned_sidecar_declaration_stays_quiet() {
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
        "#[test]\nfn uses_owned_sidecar() {assert!(true);}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["src/lib.rs".to_owned()]);
    assert_inventory(&results, true);
}
