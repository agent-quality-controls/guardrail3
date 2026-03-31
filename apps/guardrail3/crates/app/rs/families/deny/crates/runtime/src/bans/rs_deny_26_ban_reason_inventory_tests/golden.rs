use guardrail3_app_rs_family_deny_assertions::rs_deny_26_ban_reason_inventory as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn emits_documented_ban_warnings_for_reasoned_canonical_ban_entries() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(
        !results.is_empty(),
        "expected canonical deny entries to stay visible"
    );
    assert!(
        assertions::findings(&results)
            .iter()
            .any(|finding| finding.title == "ban entry count" && finding.file.is_none()),
        "expected a count summary: {results:#?}"
    );
    assert!(
        assertions::findings(&results).iter().any(|finding| {
            finding.title == "ban entry"
                && finding.file == Some("deny.toml")
                && finding.message.contains("`json5`")
        }),
        "expected canonical documented ban warnings: {results:#?}"
    );
}
