use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn warns_on_unknown_top_level_and_core_section_keys() {
    let results = super::super::run_check(&build_fixture_deny_toml("service").replace(
        "[graph]\n",
        "extra-root = true\n[graph]\nextra-flag = true\n",
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unknown graph key",
                "`deny.toml` uses unknown `[graph].extra-flag`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown top-level deny key",
                "`deny.toml` uses unknown top-level key `extra-root`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
