use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_04_cliff_exists as assertions;

use super::helpers::check;
use super::helpers::{repo_facts, repo_input};

#[test]
fn inventories_cliff_file_when_present() {
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
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
"#,
        )
        .expect("failed to parse cliff fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("cliff.toml"),
            inventory: Some(true),
            title_contains: Some("baseline"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_scope_aware_commit_parser_regexes() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^(feat|fix)(\\(.+\\))?:", group = "Core" },
    { message = "^doc(\\(.+\\))?:", group = "Documentation" },
    { message = "^perf(\\(.+\\))?:", group = "Performance" },
    { message = "^refactor(\\(.+\\))?:", group = "Refactoring" },
    { message = "^style(\\(.+\\))?:", group = "Styling" },
    { message = "^test(\\(.+\\))?:", group = "Testing" },
    { message = "^chore(\\(.+\\))?:", group = "Miscellaneous" },
]
"#,
        )
        .expect("failed to parse cliff fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_singleton_group_commit_parser_regexes() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(
        toml::from_str(
            r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^(feat)(\\(.+\\))?:", group = "Features" },
    { message = "^(fix)(\\(.+\\))?:", group = "Bug Fixes" },
    { message = "^(doc)(\\(.+\\))?:", group = "Documentation" },
    { message = "^(perf)(\\(.+\\))?:", group = "Performance" },
    { message = "^(refactor)(\\(.+\\))?:", group = "Refactoring" },
    { message = "^(style)(\\(.+\\))?:", group = "Styling" },
    { message = "^(test)(\\(.+\\))?:", group = "Testing" },
    { message = "^(chore)(\\(.+\\))?:", group = "Miscellaneous" },
]
"#,
        )
        .expect("failed to parse cliff fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("cliff.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
