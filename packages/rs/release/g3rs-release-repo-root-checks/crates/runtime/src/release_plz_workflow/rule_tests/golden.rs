use g3rs_release_repo_root_checks_assertions::release_plz_workflow::rule as assertions;

use super::helpers::run;

#[test]
fn info_when_release_plz_workflow_present() {
    let results = run(true);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "Release-plz workflow present",
            "",
            ".github/workflows/release.yml",
            true,
        )],
    );
}
