#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_11_cargo_mutants_installed::{
    assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family_with_tool, tempdir, write_file};

#[test]
fn installed_tool_keeps_an_adopted_root_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );

    let results = run_family_with_tool(root, true);

    assert_rule_quiet(&results);
}
