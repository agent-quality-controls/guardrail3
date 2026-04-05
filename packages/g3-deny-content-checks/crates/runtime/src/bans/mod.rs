mod rs_deny_10_multiple_versions_floor;
mod rs_deny_11_highlight_inventory;
mod rs_deny_12_allow_wildcard_paths;
mod rs_deny_13_wildcards_inventory;
mod rs_deny_21_tokio_full_ban;
mod rs_deny_22_extra_feature_bans_inventory;
mod rs_deny_27_duplicate_entries;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    rs_deny_10_multiple_versions_floor::check(deny_rel_path, deny, results);
    rs_deny_11_highlight_inventory::check(deny_rel_path, deny, results);
    rs_deny_12_allow_wildcard_paths::check(deny_rel_path, deny, results);
    rs_deny_13_wildcards_inventory::check(deny_rel_path, deny, results);
    rs_deny_21_tokio_full_ban::check(deny_rel_path, deny, results);
    rs_deny_22_extra_feature_bans_inventory::check(deny_rel_path, deny, results);
    rs_deny_27_duplicate_entries::check(deny_rel_path, deny, results);
}
