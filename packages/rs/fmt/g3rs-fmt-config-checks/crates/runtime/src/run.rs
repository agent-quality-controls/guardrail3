use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsFmtConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::settings::check(input, &mut results);
    crate::extra_settings::check(input, &mut results);
    crate::nightly_keys_on_stable::check(input, &mut results);
    crate::edition_mismatch::check(input, &mut results);
    crate::ignore_escape_hatch::check(input, &mut results);
    results
}
