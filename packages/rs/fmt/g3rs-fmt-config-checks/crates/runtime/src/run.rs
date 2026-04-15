use g3rs_fmt_types::G3RsFmtConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsFmtConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_fmt_config_01_settings::check(input, &mut results);
    crate::rs_fmt_config_02_extra_settings::check(input, &mut results);
    crate::rs_fmt_config_03_nightly_keys_on_stable::check(input, &mut results);
    crate::rs_fmt_config_04_edition_mismatch::check(input, &mut results);
    crate::rs_fmt_config_07_ignore_escape_hatch::check(input, &mut results);
    results
}
