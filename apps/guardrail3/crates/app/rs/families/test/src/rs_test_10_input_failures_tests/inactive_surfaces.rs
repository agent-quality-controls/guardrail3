use crate::test_support::{run_family, rule_files, tempdir, write_file};

#[test]
fn malformed_nextest_config_is_ignored_without_async_activation() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/basic.rs",
        "#[test]\nfn runs() { assert_eq!(1, 1); }\n",
    );
    write_file(root, ".config/nextest.toml", "[profile.default]\nslow-timeout = ");

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-10").is_empty());
}
