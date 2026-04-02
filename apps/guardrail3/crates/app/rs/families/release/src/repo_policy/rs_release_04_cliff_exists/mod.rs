use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

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
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cliff.toml missing".to_owned(),
            "Repo root is missing `cliff.toml`.".to_owned(),
            Some(repo.cliff_rel_path.clone()),
            None,
            false,
        ));
        return;
    }
    let Some(parsed) = repo.cliff_parsed.as_ref() else {
        return;
    };
    let Some(git) = parsed.get("git").and_then(toml::Value::as_table) else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cliff.toml missing [git]".to_owned(),
            "`cliff.toml` should include a `[git]` section.".to_owned(),
            Some(repo.cliff_rel_path.clone()),
            None,
            false,
        ));
        return;
    };
    if git
        .get("conventional_commits")
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cliff.toml missing conventional_commits = true".to_owned(),
            "`cliff.toml` should set `[git].conventional_commits = true`.".to_owned(),
            Some(repo.cliff_rel_path.clone()),
            None,
            false,
        ));
    }
    if git
        .get("filter_unconventional")
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cliff.toml missing filter_unconventional = true".to_owned(),
            "`cliff.toml` should set `[git].filter_unconventional = true`.".to_owned(),
            Some(repo.cliff_rel_path.clone()),
            None,
            false,
        ));
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
        .any(|result| result.id() == ID && !result.inventory())
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
pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
pub(crate) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]

mod tests;

fn message_covers_prefix(message: &str, prefix: &str) -> bool {
    if message == prefix {
        return true;
    }

    let Some(prefix_body) = prefix.strip_prefix('^') else {
        return false;
    };
    let Some(message_body) = message.strip_prefix('^') else {
        return false;
    };

    if let Some(stripped) = message_body.strip_prefix(prefix_body) {
        return has_valid_commit_suffix(stripped);
    }

    let Some(grouped) = message_body.strip_prefix('(') else {
        return false;
    };
    let Some(group_end) = grouped.find(')') else {
        return false;
    };
    let heads = &grouped[..group_end];
    let suffix = &grouped[(group_end + 1)..];

    heads.split('|').any(|head| head == prefix_body) && has_valid_commit_suffix(suffix)
}

fn has_valid_commit_suffix(suffix: &str) -> bool {
    suffix.starts_with(':') || (suffix.starts_with('(') && suffix.ends_with(':'))
}
