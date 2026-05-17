use g3rs_deny_config_checks_assertions::advisories::stricter_advisories_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn matching_baseline() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn no_advisories_section() {
    let results = run_check("");
    assertions::assert_no_findings(&results);
}
