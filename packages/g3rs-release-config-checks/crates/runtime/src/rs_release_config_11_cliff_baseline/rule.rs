use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// Check ID for cliff.toml baseline configuration.
const ID: &str = "RS-RELEASE-CONFIG-11";

/// Required commit message prefixes that should each have at least one parser.
const REQUIRED_PREFIXES: &[&str] = &[
    "^feat", "^fix", "^doc", "^perf", "^refactor", "^style", "^test", "^chore",
];

/// Verify baseline cliff.toml settings.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let cliff = match input.cliff.as_ref() {
        Some(c) => c,
        None => return,
    };

    let file = input.cliff_rel_path.as_deref().unwrap_or("cliff.toml");

    // Check git section exists.
    let git = match cliff.git.as_ref() {
        Some(g) => g,
        None => {
            results.push(warn(
                ID,
                "cliff: missing [git] section".to_owned(),
                "cliff.toml should have a [git] section.".to_owned(),
                file,
            ));
            // Without git section, remaining checks cannot pass.
            return;
        }
    };

    let mut issues = 0usize;

    // Check conventional_commits == true.
    if !git.conventional_commits.is_some_and(|v| v) {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "cliff: conventional_commits should be true".to_owned(),
            "Set conventional_commits = true in [git].".to_owned(),
            file,
        ));
    }

    // Check filter_unconventional == true.
    if !git.filter_unconventional.is_some_and(|v| v) {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "cliff: filter_unconventional should be true".to_owned(),
            "Set filter_unconventional = true in [git].".to_owned(),
            file,
        ));
    }

    // Check commit_parsers cover required prefixes.
    let parsers = git.commit_parsers.as_deref().unwrap_or(&[]);
    let messages: Vec<&str> = parsers
        .iter()
        .filter_map(|p| p.message.as_deref())
        .collect();

    for prefix in REQUIRED_PREFIXES {
        let covered = messages.iter().any(|m| m.starts_with(prefix));
        if !covered {
            issues = issues.saturating_add(1);
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
