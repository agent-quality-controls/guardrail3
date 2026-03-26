use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_app_rs_family_hexarch as runtime;
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsHexarchRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

const RULE_ID: &str = "RS-HEXARCH-04";

pub fn run_family(root: &Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    runtime::check(&tree, &route(&tree))
}

pub fn check_results(root: &Path) -> Vec<CheckResult> {
    run_family(root)
}

pub fn error_results<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    results
        .iter()
        .filter(|result| result.id == rule_id && result.severity == Severity::Error)
        .collect()
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], rule_id: &str) -> Vec<&'a CheckResult> {
    error_results(results, rule_id)
}

pub fn assert_no_error(results: &[CheckResult], rule_id: &str) {
    let rule_id = if rule_id.is_empty() { RULE_ID } else { rule_id };
    let errors = error_results(results, rule_id);
    assert!(
        errors.is_empty(),
        "expected no {rule_id} errors, got: {errors:#?}"
    );
}

fn route(tree: &ProjectTree) -> RsHexarchRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Hexarch]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selection, None).map_rs_hexarch()
}
