use g3rs_deny_config_checks_assertions::rs_deny_config_02_advisories_baseline as assertions;

use super::helpers::run_check;

#[test]
fn unmaintained_wrong() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "all"
yanked = "warn"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `unmaintained` has wrong value",
            "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`, found `all`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn yanked_wrong() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "deny"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `yanked` has wrong value",
            "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `deny`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn both_wrong() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "all"
yanked = "allow"
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisories `unmaintained` has wrong value",
                "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`, found `all`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "advisories `yanked` has wrong value",
                "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
