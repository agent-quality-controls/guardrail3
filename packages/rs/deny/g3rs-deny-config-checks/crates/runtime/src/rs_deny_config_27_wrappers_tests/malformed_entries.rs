use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use crate::test_support::run;

#[test]
fn errors_when_wrappers_entry_has_no_usable_name() {
    let results = run(
        r#"
[bans]
deny = [{ wrappers = ["adapter"] }]
"#,
        Some("service"),
        true,
        crate::rs_deny_config_27_wrappers::check,
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
        Some("service"),
        true,
        crate::rs_deny_config_27_wrappers::check,
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
