use g3_deny_content_checks_assertions::rs_deny_05_advisories_baseline as assertions;

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
