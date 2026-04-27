use g3rs_release_repo_root_checks_assertions::registry_token::rule as assertions;

use super::helpers::run;

#[test]
fn info_when_registry_token_present() {
    let results = run(true);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "CARGO_REGISTRY_TOKEN wired in workflow",
            "",
            ".github/workflows/release.yml",
            true,
        )],
    );
}
