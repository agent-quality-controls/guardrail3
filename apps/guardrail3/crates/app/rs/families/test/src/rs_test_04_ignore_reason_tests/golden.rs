use crate::test_support::{run_family, rule_files, tempdir, write_file};

#[test]
fn reason_comment_keeps_ignore_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/slow.rs",
        "#[test]\n// reason: external service unavailable\n#[ignore]\nfn waits_for_service() {}\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-04").is_empty());
}

#[test]
fn same_line_reason_keeps_ignore_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/slow.rs",
        "#[test]\n#[ignore] // reason: external service unavailable\nfn waits_for_service() {}\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-04").is_empty());
}

#[test]
fn ignore_attribute_reason_keeps_ignore_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/slow.rs",
        "#[test]\n#[ignore = \"external service unavailable\"]\nfn waits_for_service() {}\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-04").is_empty());
}
