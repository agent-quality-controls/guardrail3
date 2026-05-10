use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

/// Result identifier for the diff-filter ACM routing rule.
const ID: &str = "g3ts-hooks/routing-staged-files-diff-filter-acm";

/// Records a finding when the hook collects staged files without `--diff-filter=ACM`.
pub(crate) fn check(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let parsed = input.parsed();
    let has_correct = any_resolved_command(parsed, is_staged_files_with_acm);
    let has_diff = any_resolved_command(parsed, is_git_diff_cached_name_only);

    if has_correct {
        return;
    }

    let detail = if has_diff {
        "`.githooks/pre-commit` reads staged files with `git diff --cached --name-only` but does not pass `--diff-filter=ACM`. Staged-file collection must restrict the set to added/copied/modified entries."
    } else {
        "`.githooks/pre-commit` does not read staged files with `git diff --cached --name-only --diff-filter=ACM`."
    };

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "staged-file collection missing --diff-filter=ACM".to_owned(),
        detail.to_owned(),
        Some(input.rel_path().to_owned()),
        None,
    ));
}

/// Returns true when `command` is `git diff --cached --name-only ...`.
fn is_git_diff_cached_name_only(command: &ResolvedCommand) -> bool {
    command.command_name() == "git"
        && command.args().first().map(String::as_str) == Some("diff")
        && command.args().iter().any(|arg| arg == "--cached")
        && command.args().iter().any(|arg| arg == "--name-only")
}

/// Returns true when `command` collects staged files restricted to ACM entries.
fn is_staged_files_with_acm(command: &ResolvedCommand) -> bool {
    if !is_git_diff_cached_name_only(command) {
        return false;
    }
    command.args().iter().any(|arg| arg == "--diff-filter=ACM")
        || command
            .args()
            .windows(2)
            .any(|window| matches!(window, [a, b] if a == "--diff-filter" && b == "ACM"))
}
