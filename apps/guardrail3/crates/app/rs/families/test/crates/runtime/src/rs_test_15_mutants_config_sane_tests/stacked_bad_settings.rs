#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_15_mutants_config_sane::{
    assert_exclude_all_and_low_timeout, assert_reported, assert_rule_files, assert_rule_findings,
    assert_rule_quiet, ExpectedRuleFinding,
};

use super::{run_family, tempdir, write_file};
#[test]
fn exclude_all_and_low_timeout_each_emit_a_warning() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        ".cargo/mutants.toml",
        "exclude_re = [\".*\"]\ntimeout_multiplier = 0.5\n",
    );

    let results = run_family(root);
    assert!(!results.is_empty());
    assert_exclude_all_and_low_timeout(&results);
}
