use g3rs_release_repo_root_checks_assertions::rs_release_repo_root_02_publish_dry_run_workflow::rule as assertions;

use super::helpers::run;

#[test]
fn warns_when_publish_dry_run_workflow_missing() {
    let results = run(false);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "Publish dry-run workflow missing",
            "No workflow contains an actual `cargo publish --dry-run` step. Add a `cargo publish --dry-run` step to a CI workflow.",
            "Cargo.toml",
            false,
        )],
    );
}
