use super::{run_family, rule_files, tempdir, write_file};

#[test]
fn concrete_payload_match_is_allowed() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/matches.rs",
        "#[test]\nfn checks_specific_payload() {\n    let value = Some(1);\n    assert!(matches!(value, Some(1)));\n}\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-08").is_empty());
}
