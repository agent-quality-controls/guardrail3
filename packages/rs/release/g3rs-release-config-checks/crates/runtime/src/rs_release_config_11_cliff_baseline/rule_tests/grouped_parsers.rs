use g3rs_release_config_checks_assertions::rs_release_config_11_cliff_baseline as assertions;

use super::helpers::run_check;

#[test]
fn accepts_grouped_commit_parser_patterns() {
    let cliff = "\
[git]
conventional_commits = true
filter_unconventional = true

[[git.commit_parsers]]
message = \"^(feat|fix|doc|perf|refactor|style|test|chore):\"
group = \"Changes\"
";

    let results = run_check(cliff);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "cliff: baseline configuration correct",
            "",
            "cliff.toml",
            true,
        )],
    );
}
