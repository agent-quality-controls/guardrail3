use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, message_covers_prefix, warn};

const ID: &str = "RS-RELEASE-CONFIG-11";
const REQUIRED_PREFIXES: &[&str] = &[
    "^feat", "^fix", "^doc", "^perf", "^refactor", "^style", "^test", "^chore",
];

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    if !repo.cliff_exists {
        return;
    }

    let Some(cliff) = repo.cliff.as_ref() else {
        return;
    };
    let file = &repo.cliff_rel_path;
    let Some(git) = cliff.git.as_ref() else {
        results.push(warn(
            ID,
            "cliff: missing [git] section".to_owned(),
            "cliff.toml should have a [git] section.".to_owned(),
            file,
        ));
        return;
    };

    let mut issues = 0usize;

    if !git.conventional_commits.is_some_and(|value| value) {
        issues += 1;
        results.push(warn(
            ID,
            "cliff: conventional_commits should be true".to_owned(),
            "Set conventional_commits = true in [git].".to_owned(),
            file,
        ));
    }

    if !git.filter_unconventional.is_some_and(|value| value) {
        issues += 1;
        results.push(warn(
            ID,
            "cliff: filter_unconventional should be true".to_owned(),
            "Set filter_unconventional = true in [git].".to_owned(),
            file,
        ));
    }

    let messages = git
        .commit_parsers
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .filter_map(|parser| parser.message.as_deref())
        .collect::<Vec<_>>();

    for prefix in REQUIRED_PREFIXES {
        let covered = messages
            .iter()
            .any(|message| message_covers_prefix(message, prefix));
        if !covered {
            issues += 1;
            results.push(warn(
                ID,
                format!("cliff: missing commit parser for prefix \"{prefix}\""),
                format!(
                    "Add a [[git.commit_parsers]] entry with a message starting with \"{prefix}\"."
                ),
                file,
            ));
        }
    }

    if issues == 0 {
        results.push(info(
            ID,
            "cliff: baseline configuration correct".to_owned(),
            String::new(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
