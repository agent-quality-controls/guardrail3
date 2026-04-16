use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use test_support::run;

use super::helpers;

#[test]
fn errors_when_canonical_non_empty_wrapper_policy_changes() {
    let deny_toml = helpers::service_canonical_bans_toml().replace(
        r#""regex""#,
        r#"{ name = "regex", wrappers = ["tree-sitter"] }"#,
    );
    let results = run(
        &deny_toml,
        Some(guardrail3_rs_toml_parser::RustProfile::Service),
        true,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "managed ban wrappers changed",
            "`deny.toml` ban `regex` adds local wrappers `tree-sitter`.",
            "deny.toml",
            false,
        )],
    );
}
