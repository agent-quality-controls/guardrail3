use guardrail3_domain_report::Severity;

#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_01_inline_test_bodies::{
    assert_reported, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn inline_cfg_test_body_hits_owned_source_file() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib.rs",
        "#[cfg(test)]\nmod lib_tests {#[test] fn proves_nothing() {assert!(true);}}\n",
    );

    let results = run_family(root);
    assert_reported(
        &results,
        "src/lib.rs",
        Some(1),
        Severity::Error,
        "inline cfg(test) body in src",
    );
}
