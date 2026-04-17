use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::rs_deny_config_10_license_allow_baseline::check(deny_rel_path, deny, results);
    super::rs_deny_config_11_confidence_threshold::check(deny_rel_path, deny, results);
    super::rs_deny_config_12_copyleft_allowlist::check(deny_rel_path, deny, results);
}
