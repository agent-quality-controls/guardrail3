#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_04_ignore_reason::{
    Severity, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn bare_ignore_is_reported_on_the_test_file() {
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
        "#[test]\n#[ignore]\nfn waits_for_service() {}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["tests/slow.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/slow.rs",
        Some(2),
        Severity::Warn,
        "ignored test lacks reason",
    );
}
