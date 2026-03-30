use guardrail3_app_rs_family_test_assertions::rs_test_04_ignore_reason::{
    Severity, assert_count_summary, assert_inventory, assert_reported, assert_rule_files,
};

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

    assert_rule_files(&results, vec!["tests/slow.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/slow.rs",
        Some(3),
        Severity::Warn,
        "ignored test has documented reason",
    );
    assert_count_summary(
        &results,
        "`tests/slow.rs` has 1 ignored tests (1 documented, 0 missing reasons, 0 weak reasons).",
    );
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

    assert_rule_files(&results, vec!["tests/slow.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/slow.rs",
        Some(2),
        Severity::Warn,
        "ignored test has documented reason",
    );
    assert_count_summary(
        &results,
        "`tests/slow.rs` has 1 ignored tests (1 documented, 0 missing reasons, 0 weak reasons).",
    );
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

    assert_rule_files(&results, vec!["tests/slow.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/slow.rs",
        Some(2),
        Severity::Warn,
        "ignored test has documented reason",
    );
    assert_count_summary(
        &results,
        "`tests/slow.rs` has 1 ignored tests (1 documented, 0 missing reasons, 0 weak reasons).",
    );
}

#[test]
fn file_without_ignored_tests_keeps_inventory_proof() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "tests/slow.rs", "#[test]\nfn waits_for_service() {}\n");

    let results = run_family(root);

    assert_rule_files(&results, vec!["tests/slow.rs".to_owned()]);
    assert_inventory(&results, true);
}
