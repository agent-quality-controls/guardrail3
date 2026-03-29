use guardrail3_app_rs_family_test_assertions::rs_test_11_cargo_mutants_installed::{
    Severity, assert_inventory, assert_reported, assert_rule_files,
};

use super::{run_family_with_tool, tempdir, write_file};

#[test]
fn installed_tool_reports_info_for_an_adopted_root() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
    );

    let results = run_family_with_tool(root, true);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
    assert_reported(
        &results,
        "Cargo.toml",
        None,
        Severity::Info,
        "cargo-mutants installed",
    );
    assert_inventory(&results, true);
}
