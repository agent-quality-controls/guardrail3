use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, remove_deny_ban_reason};

#[test]
fn local_missing_ban_reason_only_errors_for_the_owned_local_root() {
    let results = super::helpers::run_check(&remove_deny_ban_reason(
        &build_fixture_deny_toml("service"),
        "json5",
    ));
    assert!(!results.is_empty());
    assert!(
        assertions::findings(&results).contains(&assertions::error(
            "ban entry missing reason",
            "`deny.toml` ban entry `json5` has no `reason`.",
            "deny.toml",
            false,
        )),
        "expected local missing-reason error: {results:#?}"
    );
    assert!(
        assertions::findings(&results)
            .iter()
            .any(|finding| finding.title == "ban entry count" && finding.file.is_none()),
        "expected count summaries to remain visible: {results:#?}"
    );
}
