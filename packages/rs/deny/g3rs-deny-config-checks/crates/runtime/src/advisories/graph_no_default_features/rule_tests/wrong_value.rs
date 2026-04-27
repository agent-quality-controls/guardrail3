use g3rs_deny_config_checks_assertions::advisories::graph_no_default_features::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_default_features_true() {
    let results = run_check(
        r#"
[graph]
all-features = true
no-default-features = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph no-default-features must be false",
            "`deny.toml` must set `[graph].no-default-features = false`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn no_default_features_missing_from_graph() {
    let results = run_check(
        r#"
[graph]
all-features = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph no-default-features must be false",
            "`deny.toml` must set `[graph].no-default-features = false`.",
            "deny.toml",
            false,
        )],
    );
}
