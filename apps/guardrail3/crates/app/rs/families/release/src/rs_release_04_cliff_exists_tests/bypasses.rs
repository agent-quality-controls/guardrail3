use guardrail3_app_rs_family_release_assertions::rs_release_04_cliff_exists as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn warns_when_cliff_file_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn stays_quiet_when_cliff_exists_but_parse_failure_is_owned_by_release_12() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = None;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[test]
fn warns_when_git_section_is_missing() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(toml::Value::Table(Default::default()));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("[git]"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_git_key_exists_but_is_not_a_table() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
git = "oops"
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("[git]"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_conventional_commits_is_missing() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("conventional_commits"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_conventional_commits_is_false() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = false
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("conventional_commits"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_filter_unconventional_is_missing() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("filter_unconventional"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_filter_unconventional_is_false() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = false
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("filter_unconventional"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_once_per_missing_commit_parser_prefix() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(assertions::findings(&results).len(), 6);
    assertions::assert_rule_count(&results, 6);
    assertions::assert_rule_results(
        &results,
        &["^doc", "^perf", "^refactor", "^style", "^test", "^chore"].map(|prefix| {
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some(prefix),
                file: Some("cliff.toml"),
                inventory: Some(false),
                ..Default::default()
            }
        }),
    );
}

#[test]
fn does_not_count_mid_pattern_grouped_substrings_as_prefix_coverage() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^release(feat):", group = "Releases" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("^feat"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn does_not_count_grouped_heads_with_invalid_suffixes_as_prefix_coverage() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^(feat|fix)zzz", group = "Bogus" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("^feat"),
            file: Some("cliff.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
