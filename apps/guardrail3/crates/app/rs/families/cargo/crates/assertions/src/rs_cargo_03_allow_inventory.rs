use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

const RULE_ID: &str = "RS-CARGO-03";
const EXPECTED_ALLOW_TITLES: &[&str] = &[
    "allow inventory: `missing_docs_in_private_items`",
    "allow inventory: `module_name_repetitions`",
    "allow inventory: `must_use_candidate`",
    "allow inventory: `option_if_let_else`",
    "allow inventory: `empty_line_after_doc_comments`",
    "allow inventory: `single_match_else`",
    "allow inventory: `ref_option_ref`",
    "allow inventory: `trivially_copy_pass_by_ref`",
    "allow inventory: `multiple_crate_versions`",
];

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    crate::common::check_results(tree)
}

pub fn rule_results<'a>(results: &'a [CheckResult], _rule_id: &str) -> Vec<&'a CheckResult> {
    crate::common::rule_results(results, RULE_ID)
}

pub fn assert_result_count(results: &[CheckResult], expected: usize) {
    assert_eq!(
        rule_results(results, RULE_ID).len(),
        expected,
        "unexpected {RULE_ID} results: {results:#?}"
    );
}

pub fn assert_expected_inventory(results: &[CheckResult]) {
    let actual_titles = rule_results(results, RULE_ID)
        .iter()
        .map(|result| result.title.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        actual_titles,
        EXPECTED_ALLOW_TITLES,
        "unexpected {RULE_ID} inventory titles: {actual_titles:#?}"
    );
}
