#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_08_weak_matches_assert::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

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

    assert_rule_quiet(&results);
}
