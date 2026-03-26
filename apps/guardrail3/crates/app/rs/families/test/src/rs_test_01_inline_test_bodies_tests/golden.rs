use crate::test_support::{run_family, rule_files, tempdir, write_file};

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
    write_file(root, "src/lib_tests/mod.rs", "#[test]\nfn uses_owned_sidecar() { assert!(true); }\n");

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-01").is_empty());
}
