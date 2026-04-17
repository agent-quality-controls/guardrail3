use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::rs_deny_config_13_unknown_sources_policy::check(deny_rel_path, deny, results);
    super::rs_deny_config_14_allow_registry_baseline::check(deny_rel_path, deny, results);
    super::rs_deny_config_15_allow_git_inventory::check(deny_rel_path, deny, results);
    super::rs_deny_config_18_skip_hygiene::check(deny_rel_path, deny, results);
    super::rs_deny_config_19_ignore_hygiene::check(deny_rel_path, deny, results);
    super::rs_deny_config_21_unknown_keys::check(deny_rel_path, deny, results);
    super::rs_deny_config_22_ignore_accumulation::check(deny_rel_path, deny, results);
}
