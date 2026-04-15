use guardrail3_check_types::G3CheckResult;

pub(super) fn run(has_workflow: bool) -> Vec<G3CheckResult> {
    let mut input = crate::test_support::input();
    input.has_release_plz_workflow = has_workflow;
    input.release_plz_workflow_rel_path = has_workflow.then(|| ".github/workflows/release.yml".to_owned());
    let mut results = Vec::new();
    crate::rs_release_repo_root_01_release_plz_workflow::check(&input, &mut results);
    results
}
