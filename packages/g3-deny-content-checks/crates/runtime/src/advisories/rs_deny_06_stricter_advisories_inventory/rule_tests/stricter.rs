use g3_deny_content_checks_assertions::rs_deny_06_stricter_advisories_inventory as assertions;

use super::helpers::run_check;

#[test]
fn yanked_deny() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "deny"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "advisories `yanked` stricter than baseline",
            "`deny.toml` sets `[advisories].yanked = \"deny\"`.",
            "deny.toml",
            true,
        )],
    );
}

#[test]
fn non_deny_non_baseline_not_flagged() {
    // "all" is not the baseline ("workspace") but it's also not "deny", so no inventory.
    // "allow" is not the baseline ("warn") but it's also not "deny", so no inventory.
    let results = run_check(
        r#"
[advisories]
unmaintained = "all"
yanked = "allow"
"#,
    );
    assertions::assert_no_findings(&results);
}
