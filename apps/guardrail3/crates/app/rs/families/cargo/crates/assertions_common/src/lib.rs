use std::collections::BTreeSet;

use guardrail3_app_rs_family_cargo as runtime;
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub file: Option<&'a str>,
    pub title: Option<&'a str>,
    pub inventory: Option<bool>,
}

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let structure = guardrail3_app_rs_structure::collect(tree.clone(), &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let mapper = FamilyMapper::from_legality(&legality, config.as_ref(), &selected, None);
    let route = mapper.map_rs_cargo();
    // Build a FamilyView with full project scope for tests.
    let surface = FamilyView::build(
        tree.root().clone(),
        tree.structure(),
        tree.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
    );
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
