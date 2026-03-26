#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_04_ignore_reason::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

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

    assert_rule_quiet(&results);
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

    assert_rule_quiet(&results);
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

    assert_rule_quiet(&results);
}
