#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_13_mutants_profile_present::{
    assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn present_mutants_profile_keeps_the_root_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");

    let results = run_family(root);

    assert_rule_quiet(&results);
}
