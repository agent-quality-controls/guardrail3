use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
const ID: &str = "RS-CODE-25";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    let _ = (input, results);
    let _ = (ID, Severity::Warn);
    // `RS-CODE-33` now owns weak public error-form findings to avoid double-firing.
}





// reason: test-only sidecar module wiring
