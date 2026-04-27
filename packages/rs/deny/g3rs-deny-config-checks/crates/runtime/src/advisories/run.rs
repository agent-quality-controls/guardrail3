use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    super::deprecated_advisories::check(deny_rel_path, deny, results);
    super::advisories_baseline::check(deny_rel_path, deny, results);
    super::stricter_advisories_inventory::check(deny_rel_path, deny, results);
    super::graph_all_features::check(deny_rel_path, deny, results);
    super::graph_no_default_features::check(deny_rel_path, deny, results);
}
