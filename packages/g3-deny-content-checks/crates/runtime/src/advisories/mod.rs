mod rs_deny_04_deprecated_advisories;
mod rs_deny_05_advisories_baseline;
mod rs_deny_06_stricter_advisories_inventory;
mod rs_deny_07_graph_all_features;
mod rs_deny_08_graph_no_default_features;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    rs_deny_04_deprecated_advisories::check(deny_rel_path, deny, results);
    rs_deny_05_advisories_baseline::check(deny_rel_path, deny, results);
    rs_deny_06_stricter_advisories_inventory::check(deny_rel_path, deny, results);
    rs_deny_07_graph_all_features::check(deny_rel_path, deny, results);
    rs_deny_08_graph_no_default_features::check(deny_rel_path, deny, results);
}
