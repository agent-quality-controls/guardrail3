use guardrail3_app_rs_family_test_assertions::rs_test_08_weak_matches_assert::{
    Severity, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn wildcard_payload_match_is_reported() {
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
        "#[test]\nfn hides_the_payload() {\n    let value = Some(1);\n    assert!(matches!(value, Some(_)));\n}\n",
    );

    let results = run_family(root);
    assert_rule_files(&results, vec!["tests/matches.rs".to_owned()]);
    assert_reported(
        &results,
        "tests/matches.rs",
        Some(4),
        Severity::Warn,
        "weak matches assertion",
    );
}

#[test]
fn debug_assert_wildcard_match_is_reported() {
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
        "#[test]\nfn hides_the_payload() {\n    let value = Some((1, 2));\n    debug_assert!((matches!(value, Some((_, 2)))));\n}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "tests/matches.rs",
        Some(4),
        Severity::Warn,
        "weak matches assertion",
    );
}
