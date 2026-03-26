use super::{rule_files, run_family, tempdir, write_file};

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
        "#[test]\nfn owned_sidecar() { assert!(true); }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-02").is_empty());
}
