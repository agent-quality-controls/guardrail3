#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_06_tautological_assertions::{
    Severity, assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn literal_vs_literal_assertion_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    assert_eq!(1, 1);\n}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["tests/asserts.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/asserts.rs",
        Some(3),
        Severity::Warn,
        "tautological assertion",
    );
}

#[test]
fn assert_ne_literal_vs_literal_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    assert_ne!(1, 1);\n}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/asserts.rs",
        Some(3),
        Severity::Warn,
        "tautological assertion",
    );
}

#[test]
fn debug_assert_eq_parenthesized_literals_are_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    debug_assert_eq!((1), (1));\n}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/asserts.rs",
        Some(3),
        Severity::Warn,
        "tautological assertion",
    );
}

#[test]
fn debug_assert_ne_parenthesized_literals_are_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn proves_nothing() {\n    debug_assert_ne!((2), (2));\n}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/asserts.rs",
        Some(3),
        Severity::Warn,
        "tautological assertion",
    );
}
