use guardrail3_domain_report::CheckResult;
use guardrail3_reason_policy as _;
use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-15";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let _ = (ID, input, results);
}
