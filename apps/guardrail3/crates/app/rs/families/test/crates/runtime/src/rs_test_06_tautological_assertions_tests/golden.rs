#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_06_tautological_assertions::{assert_reported, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn variable_vs_literal_assertion_is_not_tautological() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/asserts.rs",
        "#[test]\nfn checks_a_real_value() {\n    let expected = 2;\n    assert_eq!(1, expected);\n}\n",
    );

    let results = run_family(root);

    assert_rule_quiet(&results);}
