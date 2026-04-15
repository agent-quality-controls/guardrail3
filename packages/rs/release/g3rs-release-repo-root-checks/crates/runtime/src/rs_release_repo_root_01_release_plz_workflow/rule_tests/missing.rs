use g3rs_release_repo_root_checks_assertions::rs_release_repo_root_01_release_plz_workflow as assertions;

use super::helpers::run;

#[test]
fn warns_when_release_plz_workflow_missing() {
    let results = run(false);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "Release-plz workflow missing",
            "No workflow contains an actual release-plz execution step. Add a release-plz step to a GitHub Actions workflow.",
            "Cargo.toml",
            false,
        )],
    );
}
