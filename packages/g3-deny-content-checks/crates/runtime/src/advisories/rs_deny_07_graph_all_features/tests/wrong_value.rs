use g3_deny_content_checks_assertions::rs_deny_07_graph_all_features as assertions;

use super::helpers::run_check;

#[test]
fn all_features_false() {
    let results = run_check(
        r#"
[graph]
all-features = false
no-default-features = false
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph all-features must be true",
            "`deny.toml` must set `[graph].all-features = true`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn all_features_missing_from_graph() {
    let results = run_check(
        r#"
[graph]
no-default-features = false
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "graph all-features must be true",
            "`deny.toml` must set `[graph].all-features = true`.",
            "deny.toml",
            false,
        )],
    );
}
