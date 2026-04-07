use g3rs_deny_config_checks_assertions::rs_deny_config_13_unknown_sources_policy as assertions;

use super::helpers::run_check;

#[test]
fn correct_values_produce_no_findings() {
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
