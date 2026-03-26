use guardrail3_domain_report::Severity;

#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_05_should_panic_expected::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn bare_should_panic_is_reported_on_the_test_file() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic]\nfn panics_without_expected_message() {panic!(\"boom\");}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["tests/panic.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/panic.rs",
        Some(2),
        Severity::Warn,
        "should_panic missing expected string",
    );
}

#[test]
fn empty_expected_string_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic(expected = \"\")]\nfn panics_with_empty_expected_message() {panic!(\"boom\");}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/panic.rs",
        Some(2),
        Severity::Warn,
        "should_panic missing expected string",
    );
}

#[test]
fn non_string_expected_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/panic.rs",
        "#[test]\n#[should_panic(expected = 1)]\nfn panics_with_non_string_expected() {panic!(\"boom\");}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/panic.rs",
        Some(2),
        Severity::Warn,
        "should_panic missing expected string",
    );
}
