use g3rs_deny_config_checks_assertions::bans::allow_wildcard_paths::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_allow_wildcard_paths_is_false() {
    let results = run_check(
        r"
[bans]
allow-wildcard-paths = false
",
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
