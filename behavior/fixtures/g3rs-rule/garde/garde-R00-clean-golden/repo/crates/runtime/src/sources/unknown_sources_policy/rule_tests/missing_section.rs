use g3rs_deny_config_checks_assertions::sources::unknown_sources_policy::rule as assertions;

use super::helpers::run_check;

#[test]
fn missing_sources_section_errors() {
    let results = run_check(
        r#"
[advisories]
yanked = "warn"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[sources] section missing",
            "`deny.toml` has no `[sources]` section.",
            "deny.toml",
            false,
        )],
    );
}
