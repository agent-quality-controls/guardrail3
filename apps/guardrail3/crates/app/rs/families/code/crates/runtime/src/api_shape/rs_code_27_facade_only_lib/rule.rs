use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;

const ID: &str = "RS-CODE-27";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    let _ = (input, results);
    let _ = (ID, Severity::Error);
    // Removed: redundant with RS-ARCH-02 which checks lib.rs facade-only for all crates.
}
