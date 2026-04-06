use g3rs_deny_config_checks_assertions::rs_deny_config_15_allow_git_inventory as assertions;

use super::helpers::run_check;

#[test]
fn no_sources_section_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
