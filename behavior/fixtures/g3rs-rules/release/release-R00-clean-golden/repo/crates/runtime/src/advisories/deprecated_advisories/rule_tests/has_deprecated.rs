use g3rs_deny_config_checks_assertions::advisories::deprecated_advisories::rule as assertions;

use super::helpers::run_check;

#[test]
fn vulnerability_present() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
vulnerability = "deny"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "deprecated advisory field `vulnerability`",
            "`deny.toml` uses deprecated `[advisories].vulnerability`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn notice_present() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
notice = "warn"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "deprecated advisory field `notice`",
            "`deny.toml` uses deprecated `[advisories].notice`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unsound_present() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
unsound = "workspace"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "deprecated advisory field `unsound`",
            "`deny.toml` uses deprecated `[advisories].unsound`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn all_deprecated_fields_present() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
vulnerability = "deny"
notice = "warn"
unsound = "workspace"
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "deprecated advisory field `notice`",
                "`deny.toml` uses deprecated `[advisories].notice`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "deprecated advisory field `unsound`",
                "`deny.toml` uses deprecated `[advisories].unsound`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "deprecated advisory field `vulnerability`",
                "`deny.toml` uses deprecated `[advisories].vulnerability`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
