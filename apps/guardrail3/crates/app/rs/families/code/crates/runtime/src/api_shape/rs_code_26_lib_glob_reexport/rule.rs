use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;

const ID: &str = "RS-CODE-26";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    let _ = (input, results);
    let _ = (ID, Severity::Warn);
    // Removed: redundant with RS-ARCH-02 which checks broad re-exports in lib.rs for all crates.
}
