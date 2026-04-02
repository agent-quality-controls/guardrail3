use guardrail3_app_rs_family_test_assertions::rs_test_10_input_failures::{
    assert_inventory, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn malformed_nextest_config_is_ignored_without_async_activation() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/basic.rs",
        "#[test]\nfn runs() {assert_eq!(1, 1);}\n",
    );
    write_file(
        root,
        ".config/nextest.toml",
        "[profile.default]\nslow-timeout = ",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_inventory(&results, true);
}
