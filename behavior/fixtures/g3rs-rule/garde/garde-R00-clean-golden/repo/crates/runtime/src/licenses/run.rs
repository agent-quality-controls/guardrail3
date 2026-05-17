use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::license_allow_baseline::check(deny_rel_path, deny, results);
    super::confidence_threshold::check(deny_rel_path, deny, results);
    super::copyleft_allowlist::check(deny_rel_path, deny, results);
}
