use g3rs_release_config_checks_assertions::rs_release_config_10_release_plz_baseline as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_workspace_section_missing() {
    let toml = "\
[[package]]
name = \"some-crate\"
";
    let results = run_check(toml);

    // Missing workspace triggers: missing [workspace], wrong changelog_config,
    // wrong git_release_enable, wrong release_always.
    let findings = assertions::findings(&results);
    assert!(
        findings.iter().any(|f| f.title == "release-plz: missing [workspace] section"),
        "expected missing [workspace] warning, got: {findings:?}",
    );
}
