use g3rs_deny_config_checks_assertions::rs_deny_config_04_graph_all_features as assertions;

use super::helpers::run_check;

#[test]
fn all_features_true() {
    let results = run_check(
        r#"
[graph]
all-features = true
no-default-features = false
"#,
    );
    assertions::assert_no_findings(&results);
}
