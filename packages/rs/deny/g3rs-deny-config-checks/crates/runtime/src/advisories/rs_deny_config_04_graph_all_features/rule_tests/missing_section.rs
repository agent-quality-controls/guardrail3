use g3rs_deny_config_checks_assertions::advisories::rs_deny_config_04_graph_all_features::rule as assertions;

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
