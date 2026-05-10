#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

/// `ID` constant.
const ID: &str = "g3rs-hooks/routing-staged-files-diff-filter-acm";

/// `check` function.
pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let has_correct = any_resolved_command(input.parsed, is_staged_files_with_acm);
    let has_diff = any_resolved_command(input.parsed, is_git_diff_cached_name_only);

    if has_correct {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "staged-file collection uses --diff-filter=ACM".to_owned(),
                ".githooks/pre-commit collects staged files with `git diff --cached --name-only --diff-filter=ACM`."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    let detail = if has_diff {
        "`.githooks/pre-commit` reads staged files with `git diff --cached --name-only` but does not pass `--diff-filter=ACM`. Staged-file collection must restrict the set to added/copied/modified entries."
    } else {
        "`.githooks/pre-commit` does not read staged files with `git diff --cached --name-only --diff-filter=ACM`."
    };

    results.push(G3CheckResult::from_parts(
        ID.to_owned(),
        G3Severity::Error,
        "staged-file collection missing --diff-filter=ACM".to_owned(),
        detail.to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `is_git_diff_cached_name_only` function.
fn is_git_diff_cached_name_only(command: &ResolvedCommand) -> bool {
    command.command_name() == "git"
        && command.args().first().map(String::as_str) == Some("diff")
        && command.args().iter().any(|arg| arg == "--cached")
        && command.args().iter().any(|arg| arg == "--name-only")
}

/// `is_staged_files_with_acm` function.
fn is_staged_files_with_acm(command: &ResolvedCommand) -> bool {
    if !is_git_diff_cached_name_only(command) {
        return false;
    }
    command.args().iter().any(|arg| arg == "--diff-filter=ACM")
        || command
            .args()
            .windows(2)
            .any(|window| window[0] == "--diff-filter" && window[1] == "ACM")
}
