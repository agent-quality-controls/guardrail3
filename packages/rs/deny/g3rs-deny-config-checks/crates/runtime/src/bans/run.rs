use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::rs_deny_config_06_multiple_versions_floor::check(deny_rel_path, deny, results);
    super::rs_deny_config_07_highlight_inventory::check(deny_rel_path, deny, results);
    super::rs_deny_config_08_allow_wildcard_paths::check(deny_rel_path, deny, results);
    super::rs_deny_config_09_wildcards_inventory::check(deny_rel_path, deny, results);
    super::rs_deny_config_16_tokio_full_ban::check(deny_rel_path, deny, results);
    super::rs_deny_config_17_extra_feature_bans_inventory::check(deny_rel_path, deny, results);
    super::rs_deny_config_20_duplicate_entries::check(deny_rel_path, deny, results);
}
