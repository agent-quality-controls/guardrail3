use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::multiple_versions_floor::check(deny_rel_path, deny, results);
    super::highlight_inventory::check(deny_rel_path, deny, results);
    super::allow_wildcard_paths::check(deny_rel_path, deny, results);
    super::wildcards_inventory::check(deny_rel_path, deny, results);
    super::tokio_full_ban::check(deny_rel_path, deny, results);
    super::extra_feature_bans_inventory::check(deny_rel_path, deny, results);
    super::duplicate_entries::check(deny_rel_path, deny, results);
}
