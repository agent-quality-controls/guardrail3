use guardrail3_app_rs_family_deny_assertions::advisories::rs_deny_08_graph_no_default_features as assertions;

use super::super::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_graph_section_is_missing() {
    let results = super::super::run_check(&remove_section(
        &build_fixture_deny_toml("service"),
        "graph",
    ));

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
