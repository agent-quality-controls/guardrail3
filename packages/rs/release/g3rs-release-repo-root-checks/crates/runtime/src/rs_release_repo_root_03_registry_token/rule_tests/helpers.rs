use guardrail3_check_types::G3CheckResult;

pub(super) fn run(has_workflow: bool) -> Vec<G3CheckResult> {
    let mut input = crate::test_support::input();
    input.has_registry_token_workflow = has_workflow;
    input.registry_token_workflow_rel_path = has_workflow.then(|| ".github/workflows/release.yml".to_owned());
    let mut results = Vec::new();
    crate::rs_release_repo_root_03_registry_token::check(&input, &mut results);
    results
}
