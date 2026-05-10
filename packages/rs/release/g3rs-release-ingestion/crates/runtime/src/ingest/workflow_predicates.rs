//! Predicates over `WorkflowFacts` that detect release-relevant workflow shapes.

use super::collect::WorkflowFacts;

/// True when the workflow uses the `release-plz` action.
pub(super) fn workflow_has_release_plz(workflow: &WorkflowFacts) -> bool {
    workflow.analysis.jobs.iter().any(|job| {
        job.steps.iter().any(|step| {
            step.uses
                .as_deref()
                .is_some_and(|uses| uses.contains("release-plz"))
        })
    })
}

/// True when the workflow runs `cargo publish --dry-run` in any step.
pub(super) fn workflow_has_publish_dry_run(workflow: &WorkflowFacts) -> bool {
    workflow.analysis.jobs.iter().any(|job| {
        job.steps.iter().any(|step| {
            step.run_lines
                .iter()
                .any(|line| line_has_cargo_publish_dry_run(line))
        })
    })
}

/// True when any env binding (workflow, job, or step) defines `CARGO_REGISTRY_TOKEN`.
pub(super) fn workflow_has_registry_token(workflow: &WorkflowFacts) -> bool {
    workflow
        .analysis
        .env_keys
        .iter()
        .any(|key| key == "CARGO_REGISTRY_TOKEN")
        || workflow.analysis.jobs.iter().any(|job| {
            job.env_keys.iter().any(|key| key == "CARGO_REGISTRY_TOKEN")
                || job.steps.iter().any(|step| {
                    step.env_keys
                        .iter()
                        .any(|key| key == "CARGO_REGISTRY_TOKEN")
                })
        })
}

/// True when `line` (after `&&` splitting and shell-wrapper stripping) runs `cargo publish --dry-run`.
fn line_has_cargo_publish_dry_run(line: &str) -> bool {
    line.split("&&").any(|segment| {
        let segment = strip_shell_wrapper(segment.trim());
        let tokens = segment
            .split_whitespace()
            .map(normalize_run_token)
            .collect::<Vec<_>>();
        let Some(command_index) = first_command_token_index(&tokens) else {
            return false;
        };
        let Some(rest) = tokens.get(command_index..) else {
            return false;
        };
        if rest.first().is_none_or(|command| command != "cargo") {
            return false;
        }
        rest.windows(3).any(|window| {
            matches!(
                (window.first(), window.get(1), window.get(2)),
                (Some(a), Some(b), Some(c))
                    if a == "cargo" && b == "publish" && c == "--dry-run"
            )
        })
    })
}

/// Trim wrapping quotes from a run-line token.
fn normalize_run_token(token: &str) -> String {
    token.trim_matches(['"', '\'']).to_owned()
}

/// Strip a leading `sh -c "..."` / `bash -c '...'` etc. wrapper, returning the inner command.
fn strip_shell_wrapper(segment: &str) -> &str {
    for prefix in ["sh -c ", "bash -c ", "dash -c ", "zsh -c "] {
        if let Some(command) = segment.strip_prefix(prefix) {
            return command.trim().trim_matches(['"', '\'']);
        }
    }
    segment
}

/// Find the index of the first non-`env`-prefix command token.
fn first_command_token_index(tokens: &[String]) -> Option<usize> {
    let first = tokens.first()?;
    if first != "env" {
        return Some(0);
    }

    let mut index: usize = 1;
    while let Some(token) = tokens.get(index) {
        if token.starts_with('-') || token.contains('=') {
            index = index.saturating_add(1);
            continue;
        }
        return Some(index);
    }
    None
}
