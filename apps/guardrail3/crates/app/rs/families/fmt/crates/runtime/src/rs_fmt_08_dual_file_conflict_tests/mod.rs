use guardrail3_app_rs_family_fmt_assertions::rs_fmt_08_dual_file_conflict as assertions;

use super::run_check;

#[test]
fn reports_dual_root_config_conflicts() {
    let results = run_check("");

    assertions::assert_conflict(&results, "rustfmt.toml");
}

#[test]
fn reports_nested_dual_config_conflicts_at_nested_path() {
    let results = run_check("nested");

    assertions::assert_conflict(&results, "nested/rustfmt.toml");
}
