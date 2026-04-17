use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use test_support::run;

#[test]
fn errors_when_library_only_ban_is_allow_listed() {
    let results = run(
        r#"
[bans]
deny = ["axum"]
allow = ["axum"]
"#,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Library),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "allow-list overrides deny-list",
                "`deny.toml` allows `axum` even though it is banned.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "bans allow-list present",
                "`deny.toml` has non-empty `[bans].allow`: axum.",
                "deny.toml",
                false,
            ),
        ],
    );
}
