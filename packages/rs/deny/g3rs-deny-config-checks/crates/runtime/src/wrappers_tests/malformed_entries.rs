use g3rs_deny_config_checks_assertions::wrappers as assertions;

use test_support::run;

#[test]
fn errors_when_wrappers_entry_has_no_usable_name() {
    let results = run(
        r#"
[bans]
deny = [{ wrappers = ["adapter"] }]
"#,
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "ban wrappers entry malformed",
            "`deny.toml` has `[bans].deny` entry with wrappers but no usable `name` or `crate`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_wrappers_entry_name_is_blank() {
    let results = run(
        r#"
[bans]
deny = [{ name = "   ", wrappers = ["adapter"] }]
"#,
        Some(g3rs_toml_parser::types::RustProfile::Service),
        true,
        crate::wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "ban wrappers entry malformed",
            "`deny.toml` has `[bans].deny` entry with wrappers but no usable `name` or `crate`.",
            "deny.toml",
            false,
        )],
    );
}
