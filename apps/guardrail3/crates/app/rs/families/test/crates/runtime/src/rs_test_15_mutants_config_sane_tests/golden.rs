#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_15_mutants_config_sane::{
    assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn sane_mutants_config_keeps_the_root_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(
        root,
        ".cargo/mutants.toml",
        "timeout_multiplier = 2.0\nexclude_re = [\"^src/legacy/\"]\n",
    );

    let results = run_family(root);

    assert_rule_quiet(&results);
}
