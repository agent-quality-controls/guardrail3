mod rs_deny_config_10_license_allow_baseline;
mod rs_deny_config_11_confidence_threshold;
mod rs_deny_config_12_copyleft_allowlist;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    rs_deny_config_10_license_allow_baseline::check(deny_rel_path, deny, results);
    rs_deny_config_11_confidence_threshold::check(deny_rel_path, deny, results);
    rs_deny_config_12_copyleft_allowlist::check(deny_rel_path, deny, results);
}
