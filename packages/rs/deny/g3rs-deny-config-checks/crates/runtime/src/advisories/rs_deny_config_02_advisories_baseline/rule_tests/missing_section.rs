use g3rs_deny_config_checks_assertions::advisories::rs_deny_config_02_advisories_baseline::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_advisories_section() {
    let results = run_check("");
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[advisories] section missing",
            "`deny.toml` has no `[advisories]` section.",
            "deny.toml",
            false,
        )],
    );
}
