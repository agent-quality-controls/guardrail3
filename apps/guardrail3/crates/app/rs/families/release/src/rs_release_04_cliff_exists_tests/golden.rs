use guardrail3_domain_report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

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
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-04");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("cliff.toml"));
    assert!(results[0].title.contains("baseline"));
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
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-04");
    assert!(results[0].inventory);
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
        .expect("valid cliff"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-04");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("cliff.toml"));
}
