use g3rs_deny_config_checks_assertions::licenses::confidence_threshold::rule as assertions;

use super::helpers::run_check;

#[test]
fn missing_threshold_produces_warn() {
    let results = run_check(
        r#"
[licenses]
allow = ["MIT"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "confidence-threshold missing or invalid",
            "`deny.toml` must set `confidence-threshold >= 0.8`.",
            "deny.toml",
            false,
        )],
    );
}
