use g3rs_release_config_checks_assertions::cliff_baseline::rule as assertions;

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

    assertions::assert_title_count(&results, "cliff: missing commit parser for prefix", 8);
}
