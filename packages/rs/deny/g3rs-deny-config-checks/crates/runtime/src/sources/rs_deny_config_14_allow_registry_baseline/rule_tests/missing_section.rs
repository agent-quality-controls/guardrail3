use g3rs_deny_config_checks_assertions::rs_deny_config_14_allow_registry_baseline as assertions;

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
            "[sources] allow-registry missing",
            "`deny.toml` has no valid crates.io registry allow-list.",
            "deny.toml",
            false,
        )],
    );
}
