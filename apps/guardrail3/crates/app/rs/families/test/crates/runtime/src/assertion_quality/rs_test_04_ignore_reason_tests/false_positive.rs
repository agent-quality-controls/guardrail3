use guardrail3_app_rs_family_test_assertions::rs_test_04_ignore_reason::assert_rule_quiet;

use super::{run_family, tempdir, write_file};

#[test]
fn generated_target_rust_does_not_activate_ignore_reason_checks() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "target/debug/build/demo/out/private.rs",
        "#[test]\n#[ignore]\nfn generated_private_test() {}\n",
    );

    let results = run_family(root);

    assert_rule_quiet(&results);
}
