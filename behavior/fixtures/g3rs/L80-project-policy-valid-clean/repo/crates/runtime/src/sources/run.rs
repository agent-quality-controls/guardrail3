use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::unknown_sources_policy::check(deny_rel_path, deny, results);
    super::allow_registry_baseline::check(deny_rel_path, deny, results);
    super::allow_git_inventory::check(deny_rel_path, deny, results);
    super::skip_hygiene::check(deny_rel_path, deny, results);
    super::ignore_hygiene::check(deny_rel_path, deny, results);
    super::unknown_keys::check(deny_rel_path, deny, results);
    super::ignore_accumulation::check(deny_rel_path, deny, results);
}
