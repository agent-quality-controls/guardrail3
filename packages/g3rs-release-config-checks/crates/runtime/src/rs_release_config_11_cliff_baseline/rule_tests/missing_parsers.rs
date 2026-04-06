use g3rs_release_config_checks_assertions::rs_release_config_11_cliff_baseline as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_commit_parser_prefixes_missing() {
    // Has git section with conventional_commits, but no commit parsers.
    let toml = "\
[git]
conventional_commits = true
filter_unconventional = true
";
    let results = run_check(toml);

    let findings = assertions::findings(&results);
    // Should warn for each of the 8 required prefixes.
    let missing_parser_warnings: Vec<_> = findings
        .iter()
        .filter(|f| f.title.starts_with("cliff: missing commit parser for prefix"))
        .collect();
    assert_eq!(
        missing_parser_warnings.len(),
        8,
        "expected 8 missing commit parser warnings, got: {missing_parser_warnings:?}",
    );
}
