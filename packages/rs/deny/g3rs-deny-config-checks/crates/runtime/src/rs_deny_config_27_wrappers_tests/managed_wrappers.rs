use g3rs_deny_config_checks_assertions::rs_deny_config_27_wrappers as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn errors_when_canonical_non_empty_wrapper_policy_changes() {
    let deny_toml = canonical_bans_toml("service").replace(
        r#"{ name = "regex", wrappers = ["globset", "ignore", "tree-sitter"] }"#,
        r#"{ name = "regex", wrappers = ["tree-sitter"] }"#,
    );
    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_27_wrappers::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "managed ban wrappers changed",
            "`deny.toml` ban `regex` must keep wrappers `globset, ignore, tree-sitter`.",
            "deny.toml",
            false,
        )],
    );
}
