use g3rs_release_repo_root_checks_assertions::rs_release_repo_root_02_publish_dry_run_workflow as assertions;

use super::helpers::run;

#[test]
fn info_when_publish_dry_run_workflow_present() {
    let results = run(true);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "Publish dry-run workflow present",
            "",
            ".github/workflows/release.yml",
            true,
        )],
    );
}
