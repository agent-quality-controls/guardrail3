use g3rs_deny_config_checks_assertions::extra_deny_bans_inventory as assertions;

use test_support::run;

#[test]
fn errors_when_deny_entry_has_no_usable_name() {
    let results = run(
        r#"
[bans]
deny = [{ version = "1.0.0", reason = "temporary" }]
"#,
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::extra_deny_bans_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "deny ban entry malformed",
                "`deny.toml` has `[bans].deny` entry without a usable `name` or `crate`.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "no extra deny bans",
                "`deny.toml` has 0 deny bans beyond the managed baseline.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn errors_when_deny_entry_name_is_blank() {
    let deny_toml = r#"
[bans]
deny = [{ name = "   ", reason = "temporary" }]
"#;
    let results = run(
        deny_toml,
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::extra_deny_bans_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "deny ban entry malformed",
                "`deny.toml` has `[bans].deny` entry without a usable `name` or `crate`.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "no extra deny bans",
                "`deny.toml` has 0 deny bans beyond the managed baseline.",
                "deny.toml",
                true,
            ),
        ],
    );
}
