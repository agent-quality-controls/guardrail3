use g3_deny_content_checks_assertions::rs_deny_08_graph_no_default_features as assertions;

use super::helpers::run_check;

#[test]
fn no_graph_section() {
    let results = run_check("");
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[graph] section missing",
            "`deny.toml` must contain `[graph]` coverage settings.",
            "deny.toml",
            false,
        )],
    );
}
