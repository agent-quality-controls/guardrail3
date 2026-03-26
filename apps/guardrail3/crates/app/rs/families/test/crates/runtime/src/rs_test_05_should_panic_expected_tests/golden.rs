#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_05_should_panic_expected::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn expected_message_keeps_should_panic_quiet() {
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
        "#[test]\n#[should_panic(expected = \"boom\")]\nfn panics_with_expected_message() {panic!(\"boom\");}\n",
    );

    let results = run_family(root);

    assert_rule_quiet(&results);
}
