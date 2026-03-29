use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-04";
const REQUIRED_COMMIT_PREFIXES: &[&str] = &[
    "^feat",
    "^fix",
    "^doc",
    "^perf",
    "^refactor",
    "^style",
    "^test",
    "^chore",
];

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if !repo.cliff_exists {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cliff.toml missing".to_owned(),
            message: "Repo root is missing `cliff.toml`.".to_owned(),
            file: Some(repo.cliff_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }
    let Some(parsed) = repo.cliff_parsed.as_ref() else {
        return;
    };
    let Some(git) = parsed.get("git").and_then(toml::Value::as_table) else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cliff.toml missing [git]".to_owned(),
            message: "`cliff.toml` should include a `[git]` section.".to_owned(),
            file: Some(repo.cliff_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    };
    if git
        .get("conventional_commits")
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cliff.toml missing conventional_commits = true".to_owned(),
            message: "`cliff.toml` should set `[git].conventional_commits = true`.".to_owned(),
            file: Some(repo.cliff_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    if git
        .get("filter_unconventional")
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cliff.toml missing filter_unconventional = true".to_owned(),
            message: "`cliff.toml` should set `[git].filter_unconventional = true`.".to_owned(),
            file: Some(repo.cliff_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    let commit_parsers = git
        .get("commit_parsers")
        .and_then(toml::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for prefix in REQUIRED_COMMIT_PREFIXES {
        let covered = commit_parsers.iter().any(|entry| {
            entry
                .get("message")
                .and_then(toml::Value::as_str)
                .is_some_and(|message| message_covers_prefix(message, prefix))
        });
        if !covered {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: format!("cliff.toml missing commit parser `{prefix}`"),
                message: format!(
                    "`cliff.toml` should include a `[git].commit_parsers` entry for `{prefix}`."
                ),
                file: Some(repo.cliff_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
    if !results
        .iter()
        .any(|result| result.id == ID && !result.inventory)
    {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cliff.toml baseline looks good".to_owned(),
                message: "`cliff.toml` includes the canonical git-cliff release baseline."
                    .to_owned(),
                file: Some(repo.cliff_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}
#[cfg(test)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]
#[path = "rs_release_04_cliff_exists_tests/mod.rs"]
mod rs_release_04_cliff_exists_tests;

fn message_covers_prefix(message: &str, prefix: &str) -> bool {
    if message == prefix {
        return true;
    }
    let bare = prefix.trim_start_matches('^');
    let exact_head = format!("^{bare}");
    if message.starts_with(&format!("{exact_head}("))
        || message.starts_with(&format!("{exact_head}:"))
        || message.starts_with(&format!("{exact_head}\\"))
    {
        return true;
    }
    let Some(grouped) = message.strip_prefix("^(") else {
        return false;
    };
    let Some(close_paren) = grouped.find(')') else {
        return false;
    };
    let suffix = &grouped[close_paren + 1..];
    let valid_continuation = suffix.is_empty()
        || suffix.starts_with('(')
        || suffix.starts_with(':')
        || suffix.starts_with('\\');
    valid_continuation && grouped[..close_paren].split('|').any(|entry| entry == bare)
}
