#![allow(
    clippy::missing_docs_in_private_items,
    reason = "private managed-chain helpers are named by the specific result they produce"
)]

use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;
use hook_shell_parser::command_query::any_resolved_command;

use crate::compat::{G3CheckResult as CompatResult, G3Severity};

const MANAGED_G3RS_HOOK_CHAIN_ID: &str = "g3rs-hooks/managed-g3rs-hook-chain";
pub(crate) const MANAGED_G3RS_HOOK_PATH: &str = ".githooks/pre-commit.d/g3rs";

pub(crate) fn check(inputs: &[G3RsHooksSourceChecksInput], results: &mut Vec<G3CheckResult>) {
    let Some(pre_commit) = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::PreCommit)
    else {
        return;
    };

    let Some(managed) = inputs
        .iter()
        .find(|input| input.rel_path == MANAGED_G3RS_HOOK_PATH)
    else {
        results.push(missing_managed_hook_result());
        return;
    };

    let pre_commit_text = source_text(&pre_commit.parsed);
    if !pre_commit_text.contains(MANAGED_G3RS_HOOK_PATH) {
        results.push(pre_commit_does_not_run_managed_result(&pre_commit.rel_path));
        return;
    }

    if managed_has_required_commands(managed) {
        results.push(managed_chain_present_result(&managed.rel_path));
        return;
    }

    results.push(managed_commands_missing_result(&managed.rel_path));
}

fn missing_managed_hook_result() -> G3CheckResult {
    CompatResult::from_parts(
        MANAGED_G3RS_HOOK_CHAIN_ID.to_owned(),
        G3Severity::Error,
        "managed g3rs hook missing".to_owned(),
        "`.githooks/pre-commit` must run `.githooks/pre-commit.d/g3rs`, and that managed G3RS hook file must exist."
            .to_owned(),
        Some(MANAGED_G3RS_HOOK_PATH.to_owned()),
        None,
        false,
    )
    .into_inner()
}

fn pre_commit_does_not_run_managed_result(rel_path: &str) -> G3CheckResult {
    CompatResult::from_parts(
        MANAGED_G3RS_HOOK_CHAIN_ID.to_owned(),
        G3Severity::Error,
        "pre-commit does not run managed g3rs hook".to_owned(),
        "`.githooks/pre-commit` must run `.githooks/pre-commit.d/g3rs`.".to_owned(),
        Some(rel_path.to_owned()),
        None,
        false,
    )
    .into_inner()
}

fn managed_chain_present_result(rel_path: &str) -> G3CheckResult {
    CompatResult::from_parts(
        MANAGED_G3RS_HOOK_CHAIN_ID.to_owned(),
        G3Severity::Warn,
        "managed g3rs hook chain present".to_owned(),
        "`.githooks/pre-commit` runs `.githooks/pre-commit.d/g3rs`, and the managed G3RS hook contains repo and workspace validation commands."
            .to_owned(),
        Some(rel_path.to_owned()),
        None,
        false,
    )
    .into_inventory()
    .into_inner()
}

fn managed_commands_missing_result(rel_path: &str) -> G3CheckResult {
    CompatResult::from_parts(
        MANAGED_G3RS_HOOK_CHAIN_ID.to_owned(),
        G3Severity::Error,
        "managed g3rs hook commands missing".to_owned(),
        "`.githooks/pre-commit.d/g3rs` must call `g3rs validate repo` and `g3rs validate workspace --path <unit> --staged`."
            .to_owned(),
        Some(rel_path.to_owned()),
        None,
        false,
    )
    .into_inner()
}

fn managed_has_required_commands(managed: &G3RsHooksSourceChecksInput) -> bool {
    let has_repo_validate = any_resolved_command(&managed.parsed, |command| {
        command.command_name() == "g3rs"
            && command.args().first().map(String::as_str) == Some("validate")
            && command.args().get(1).map(String::as_str) == Some("repo")
    });
    let has_workspace_validate = managed
        .parsed
        .source_lines
        .iter()
        .map(|line| line.raw.trim_start())
        .any(line_is_g3rs_validate_workspace_path_staged);

    has_repo_validate && has_workspace_validate
}

fn line_is_g3rs_validate_workspace_path_staged(line: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(line);
    for (index, window) in words.windows(3).enumerate() {
        let [command, validate, scope] = window else {
            continue;
        };
        if command == "g3rs" && validate == "validate" && scope == "workspace" {
            let tail = index
                .checked_add(3)
                .and_then(|start| words.get(start..))
                .unwrap_or(&[]);
            let has_staged = tail.iter().any(|w| w == "--staged");
            let has_path = tail
                .iter()
                .any(|w| w == "--path" || w.starts_with("--path="));
            return has_staged && has_path;
        }
    }
    false
}

fn source_text(parsed: &hook_shell_parser::types::ParsedShellScript) -> String {
    parsed
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
