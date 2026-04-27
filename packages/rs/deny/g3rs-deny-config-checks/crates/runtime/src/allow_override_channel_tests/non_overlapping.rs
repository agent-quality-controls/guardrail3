use g3rs_deny_config_checks_assertions::allow_override_channel as assertions;

use test_support::run;

#[test]
fn reports_non_overlapping_allow_entries_without_override_findings() {
    let results = run(
        r#"
[bans]
allow = ["totally-custom-crate"]
"#,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Service),
        true,
        crate::allow_override_channel::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list present",
            "`deny.toml` has non-empty `[bans].allow`: totally-custom-crate.",
            "deny.toml",
            false,
        )],
    );
}
