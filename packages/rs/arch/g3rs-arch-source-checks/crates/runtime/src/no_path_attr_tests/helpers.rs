use g3rs_arch_types::types::G3RsArchPathAttrSite;
use guardrail3_check_types::G3CheckResult;

pub(super) fn site(
    rel_path: &str,
    line: usize,
    module_name: &str,
    path_value: Option<&str>,
    cfg_test_only: bool,
) -> G3RsArchPathAttrSite {
    G3RsArchPathAttrSite {
        rel_path: rel_path.to_owned(),
        line,
        module_name: module_name.to_owned(),
        path_value: path_value.map(str::to_owned),
        cfg_test_only,
    }
}

pub(super) fn run_rule(site: &G3RsArchPathAttrSite) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::no_path_attr::check(site, &mut results);
    results
}
