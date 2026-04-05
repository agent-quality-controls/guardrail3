use g3_deny_content_checks_assertions::rs_deny_12_allow_wildcard_paths as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_allow_wildcard_paths_is_false() {
    let results = run_check(
        r#"
[bans]
allow-wildcard-paths = false
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "allow-wildcard-paths must be true",
            "`deny.toml` must set `[bans].allow-wildcard-paths = true`.",
            "deny.toml",
            false,
        )],
    );
}
