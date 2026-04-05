mod rs_deny_18_unknown_sources_policy;
mod rs_deny_19_allow_registry_baseline;
mod rs_deny_20_allow_git_inventory;
mod rs_deny_23_skip_hygiene;
mod rs_deny_24_ignore_hygiene;
mod rs_deny_28_unknown_keys;
mod rs_deny_29_ignore_accumulation;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    rs_deny_18_unknown_sources_policy::check(deny_rel_path, deny, results);
    rs_deny_19_allow_registry_baseline::check(deny_rel_path, deny, results);
    rs_deny_20_allow_git_inventory::check(deny_rel_path, deny, results);
    rs_deny_23_skip_hygiene::check(deny_rel_path, deny, results);
    rs_deny_24_ignore_hygiene::check(deny_rel_path, deny, results);
    rs_deny_28_unknown_keys::check(deny_rel_path, deny, results);
    rs_deny_29_ignore_accumulation::check(deny_rel_path, deny, results);
}
