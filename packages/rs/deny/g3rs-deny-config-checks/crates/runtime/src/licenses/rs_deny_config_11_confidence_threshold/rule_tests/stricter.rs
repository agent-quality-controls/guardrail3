use g3rs_deny_config_checks_assertions::licenses::rs_deny_config_11_confidence_threshold::rule as assertions;

use super::helpers::run_check;

#[test]
fn stricter_threshold_produces_inventory() {
    let results = run_check(
        r#"
[licenses]
confidence-threshold = 0.9
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "confidence-threshold stricter than baseline",
            "`deny.toml` sets `confidence-threshold = 0.9`.",
            "deny.toml",
            true,
        )],
    );
}
