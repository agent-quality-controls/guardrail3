use guardrail3_app_rs_family_test_assertions::rs_test_09_nextest_timeouts::{
    Severity, assert_reported,
};

use super::{run_family, tempdir, write_file};

#[test]
fn scoped_non_test_source_does_not_activate_async_timeout_checks() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = {version = \"1\", features = [\"macros\", \"rt\"]}\n",
    );
    write_file(root, "src/lib.rs", "pub fn value() -> u8 { 1 }\n");
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );

    let results = run_family(root);

    assert_reported(
        &results,
        ".config/nextest.toml",
        None,
        Severity::Error,
        "nextest config missing",
    );
}
