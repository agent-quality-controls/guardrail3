use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use test_support::run;

#[test]
fn errors_when_allow_entries_have_no_matchable_name() {
    let results = run(
        r#"
[bans]
deny = ["json5"]
allow = [{ reason = "Temporary local carveout while removal lands." }]
"#,
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list malformed",
            "`deny.toml` has malformed `[bans].allow` entries that cannot be matched to crate names.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_allow_entry_name_is_blank() {
    let results = run(
        r#"
[bans]
deny = ["json5"]
allow = ["   "]
"#,
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list malformed",
            "`deny.toml` has malformed `[bans].allow` entries that cannot be matched to crate names.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_detailed_allow_entry_name_is_blank() {
    let results = run(
        r#"
[bans]
deny = ["json5"]
allow = [{ name = "   ", reason = "Temporary local carveout while removal lands." }]
"#,
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list malformed",
            "`deny.toml` has malformed `[bans].allow` entries that cannot be matched to crate names.",
            "deny.toml",
            false,
        )],
    );
}
