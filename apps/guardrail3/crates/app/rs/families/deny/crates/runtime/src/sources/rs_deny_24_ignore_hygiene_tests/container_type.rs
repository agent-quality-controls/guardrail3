use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_24_ignore_hygiene as assertions;

#[test]
fn errors_when_ignore_container_is_not_an_array() {
    let results = super::super::run_check("[advisories]\nignore = \"RUSTSEC-2026-0001\"\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "malformed advisory ignore container",
            "`deny.toml` must use an array for `[advisories].ignore` entries.",
            "deny.toml",
            false,
        )],
    );
}
