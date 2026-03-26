use std::collections::BTreeSet;

use guardrail3_app_rs_family_arch as runtime;
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = route(tree);
    runtime::check(tree, &route)
}

pub fn error_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Error)
        .collect()
}

pub fn info_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Info)
        .collect()
}

pub fn assert_error_files(results: &[CheckResult], id: &str, expected: &[&str]) {
    let actual = error_results(results, id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "unexpected {id} hit set: {results:#?}");
}

pub fn assert_info_files(results: &[CheckResult], id: &str, expected: &[&str]) {
    let actual = info_results(results, id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "unexpected {id} hit set: {results:#?}");
}

fn route(tree: &ProjectTree) -> guardrail3_app_rs_family_mapper::RsArchRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Arch]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selection, None).map_rs_arch()
}
