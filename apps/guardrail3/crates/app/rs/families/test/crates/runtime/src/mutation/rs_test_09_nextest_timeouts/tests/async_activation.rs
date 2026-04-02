use guardrail3_app_rs_family_test_assertions::rs_test_09_nextest_timeouts::{
    Severity, assert_reported, assert_rule_files,
};

use super::{run_family, tempdir, write_file};

#[test]
fn tokio_root_without_nextest_timeouts_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = {version = \"1\", features = [\"macros\", \"rt\"]}\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() {assert!(true);}\n",
    );

    let results = run_family(root);

    assert_rule_files(&results, vec![".config/nextest.toml".to_owned()]);
    assert_reported(
        &results,
        ".config/nextest.toml",
        None,
        Severity::Error,
        "nextest config missing",
    );
}
