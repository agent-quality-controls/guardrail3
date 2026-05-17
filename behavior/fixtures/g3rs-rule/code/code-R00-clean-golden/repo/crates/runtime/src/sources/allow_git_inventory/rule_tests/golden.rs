use g3rs_deny_config_checks_assertions::sources::allow_git_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn empty_allow_git_produces_no_findings() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );
    assertions::assert_no_findings(&results);
}
