use g3rs_deny_config_checks_assertions::rs_deny_config_02_advisories_baseline as assertions;

use super::helpers::run_check;

#[test]
fn unmaintained_missing() {
    let results = run_check(
        r#"
[advisories]
yanked = "deny"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `unmaintained` missing",
            "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn yanked_missing() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `yanked` missing",
            "`deny.toml` must set `[advisories].yanked = \"deny\"`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn both_missing() {
    let results = run_check(
        r#"
[advisories]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisories `unmaintained` missing",
                "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "advisories `yanked` missing",
                "`deny.toml` must set `[advisories].yanked = \"deny\"`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
