use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::GrdzCheckResult;

pub fn check(input: &G3FmtContentChecksInput) -> Vec<GrdzCheckResult> {
    let mut results = Vec::new();
    crate::rs_fmt_02_settings::check(input, &mut results);
    crate::rs_fmt_03_extra_settings::check(input, &mut results);
    crate::rs_fmt_04_nightly_keys_on_stable::check(input, &mut results);
    crate::rs_fmt_06_edition_mismatch::check(input, &mut results);
    results
}
