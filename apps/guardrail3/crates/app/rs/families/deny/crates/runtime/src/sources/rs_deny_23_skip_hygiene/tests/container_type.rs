use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_23_skip_hygiene as assertions;

#[test]
fn errors_when_skip_container_is_not_an_array() {
    let results = super::super::run_check("[bans]\nskip = \"serde\"\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "malformed skip container",
            "`deny.toml` must use an array for `[bans].skip` entries.",
            "deny.toml",
            false,
        )],
    );
}
