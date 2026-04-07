use g3rs_deny_config_checks_assertions::rs_deny_config_05_graph_no_default_features as assertions;

use super::helpers::run_check;

#[test]
fn no_default_features_false() {
    let results = run_check(
        r#"
[graph]
all-features = true
no-default-features = false
"#,
    );
    assertions::assert_no_findings(&results);
}
