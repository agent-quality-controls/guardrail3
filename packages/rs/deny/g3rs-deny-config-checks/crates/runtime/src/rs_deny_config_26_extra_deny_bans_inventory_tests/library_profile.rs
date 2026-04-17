use g3rs_deny_config_checks_assertions::rs_deny_config_26_extra_deny_bans_inventory as assertions;

use test_support::run;

#[test]
fn does_not_inventory_library_only_bans_as_extra_in_library_profile() {
    let results = run(
        r#"
[bans]
deny = ["axum"]
"#,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Library),
        true,
        crate::rs_deny_config_26_extra_deny_bans_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no extra deny bans",
            "`deny.toml` has 0 deny bans beyond the managed baseline.",
            "deny.toml",
            true,
        )],
    );
}
