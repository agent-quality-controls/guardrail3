use crate::test_support::{run_family, rule_files, tempdir, write_file};

#[test]
fn clean_root_surfaces_no_input_failures() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "pub fn value() -> u8 { 1 }\n");

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-10").is_empty());
    assert!(results.iter().all(|result| result.id != "RS-TEST-10"));
}
