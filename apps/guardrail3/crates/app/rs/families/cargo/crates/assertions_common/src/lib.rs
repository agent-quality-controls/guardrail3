use std::collections::BTreeSet;

use guardrail3_app_rs_family_cargo as runtime;
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsProjectSurface};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTreeDiscovery;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub file: Option<&'a str>,
    pub title: Option<&'a str>,
    pub inventory: Option<bool>,
}

pub fn check_results(tree: &dyn ProjectTreeDiscovery) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_cargo();
    let surface = RsProjectSurface::from_tree(tree);
    runtime::check(&surface, &route)
}

pub fn rule_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results.iter().filter(|result| result.id() == id).collect()
}

pub fn has_result<F>(results: &[CheckResult], id: &str, predicate: F) -> bool
where
    F: Fn(&CheckResult) -> bool,
{
    results
        .iter()
        .any(|result| result.id() == id && predicate(result))
}

pub fn assert_rule_results(
    results: &[CheckResult],
    rule_id: &str,
    expected: &[ExpectedRuleResult<'_>],
) {
    let actual = rule_results(results, rule_id);
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {rule_id} results: {results:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            expected_result
                .file
                .is_none_or(|file| result.file() == Some(file))
                && expected_result
                    .title
                    .is_none_or(|title| result.title() == title)
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory() == inventory)
        });
        assert!(
            matched,
            "missing expected {rule_id} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}
