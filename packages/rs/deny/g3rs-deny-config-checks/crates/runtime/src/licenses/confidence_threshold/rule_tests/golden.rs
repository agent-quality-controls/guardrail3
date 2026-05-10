use g3rs_deny_config_checks_assertions::licenses::confidence_threshold::rule as assertions;

use super::helpers::run_check;

#[test]
fn exact_threshold_produces_no_findings() {
    let results = run_check(
        r"
[licenses]
confidence-threshold = 0.8
",
    );
    assertions::assert_no_findings(&results);
}
