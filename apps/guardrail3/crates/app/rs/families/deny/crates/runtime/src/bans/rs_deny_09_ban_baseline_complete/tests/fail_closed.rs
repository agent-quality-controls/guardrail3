use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_09_ban_baseline_complete as assertions;

#[test]
fn errors_when_bans_section_is_missing() {
    let results = super::helpers::run_check("[graph]\nall-features = true\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[bans] section missing",
            "`deny.toml` has no `[bans]` section.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_bans_deny_is_missing() {
    let results = super::helpers::run_check("[bans]\nmultiple-versions = \"deny\"\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[bans].deny missing",
            "`deny.toml` must contain `[bans].deny`.",
            "deny.toml",
            false,
        )],
    );
}
