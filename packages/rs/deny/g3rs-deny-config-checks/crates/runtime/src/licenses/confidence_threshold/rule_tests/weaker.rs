use g3rs_deny_config_checks_assertions::licenses::confidence_threshold::rule as assertions;

use super::helpers::run_check;

#[test]
fn weaker_threshold_produces_warn() {
    let results = run_check(
        r#"
[licenses]
confidence-threshold = 0.6
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "confidence-threshold weaker than baseline",
            "`deny.toml` sets `confidence-threshold = 0.6`.",
            "deny.toml",
            false,
        )],
    );
}
