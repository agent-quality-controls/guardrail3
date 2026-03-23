use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn warns_when_cliff_file_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-04");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("cliff.toml"));
}

#[test]
fn stays_quiet_when_cliff_exists_but_parse_failure_is_owned_by_release_12() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = None;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn warns_when_git_section_is_missing() {
    let mut facts = repo_facts();
    facts.cliff_exists = true;
    facts.cliff_parsed = Some(toml::Value::Table(Default::default()));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].title.contains("[git]"));
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].title.contains("conventional_commits"));
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].title.contains("filter_unconventional"));
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

    assert_eq!(results.len(), 6);
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Warn)
    );
    assert!(
        results
            .iter()
            .all(|result| result.title.contains("commit parser"))
    );
}
