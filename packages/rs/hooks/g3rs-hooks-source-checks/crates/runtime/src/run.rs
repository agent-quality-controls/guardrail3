#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::match_same_arms,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::type_complexity,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::compat::{G3CheckResult as CompatResult, G3Severity};

#[must_use]
pub fn check(input: &G3RsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_single(input, true)
}

#[must_use]
pub fn check_all(inputs: &[G3RsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = inputs
        .iter()
        .flat_map(|input| check_single(input, false))
        .collect::<Vec<_>>();

    check_required_contracts_across_selected_surface(inputs, &mut results);

    results
}

/// `check_single` function.
fn check_single(
    input: &G3RsHooksSourceChecksInput,
    include_required_contracts: bool,
) -> Vec<G3CheckResult> {
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
        // Reason: the bash verifier was deleted; the verifier is now in-binary
        // (`g3rs validate --path` / `g3rs validate-repo`). Any leftover ingestion
        // input in the verifier slot is treated as a modular script for shape checks.
        G3RsHookScriptKind::G3RsVerifier => crate::facts::HookScriptKind::Modular,
    };
    let rust_input = crate::inputs::RustHookCommandInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        is_workspace_project: input.is_workspace_project,
        requirements: &input.requirements,
    };
    let executable_input = crate::inputs::ExecutableCommandContextInput {
        rel_path: &input.rel_path,
        kind,
        parsed: &input.parsed,
    };
    let dispatcher_input = crate::inputs::DispatcherSyntaxInput {
        rel_path: &input.rel_path,
        has_modular_dir: input.has_modular_dir,
        parsed: &input.parsed,
    };
    let fail_open_input = crate::inputs::FailOpenWrapperInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        requirements: &input.requirements,
    };
    let _ = rust_input.is_workspace_project;
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::real_dispatcher_syntax_only::check(&dispatcher_input, &mut results);

        crate::gitleaks_step_present::check(&rust_input, &mut results);
        crate::routing::scope_not_hardcoded_literal::check(&rust_input, &mut results);
        crate::routing::no_env_override_routing::check(&rust_input, &mut results);
        crate::routing::no_upward_walk_from_units::check(&rust_input, &mut results);
        crate::routing::discovers_marker_pair::check(&rust_input, &mut results);
        crate::routing::staged_files_diff_filter_acm::check(&rust_input, &mut results);
        check_calls_validate_repo(&rust_input, &mut results);
        check_dispatches_per_unit_validate_staged(&rust_input, &mut results);
        check_dedups_owning_units(&rust_input, &mut results);
        check_skips_when_no_owning_unit(&rust_input, &mut results);
        check_no_toolchain_invocation(&rust_input, &mut results);
        crate::contract_trigger_coverage::rule::check(&rust_input, &mut results);
        if include_required_contracts {
            crate::required_contract_command_present::rule::check(&rust_input, &mut results);
        }
    }

    if !input.exists {
        return crate::compat::finish(results);
    }

    crate::shell_safety::shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::valid_shebang::check(&executable_input, &mut results);
    if input.exists {
        crate::shell_safety::no_unconditional_exit_zero::check(&executable_input, &mut results);
        crate::shell_safety::no_bypass_instructions::check(&executable_input, &mut results);
        crate::workflow::merge_conflict_step_present::check(&executable_input, &mut results);
        crate::workflow::file_size_step_present::check(&executable_input, &mut results);
        crate::shell_safety::executable_command_context_only::check(
            &executable_input,
            &mut results,
        );
        crate::shell_safety::concrete_lockfile_command::check(&executable_input, &mut results);
    }
    if input.exists {
        crate::shell_safety::no_fail_open_wrappers::check(&fail_open_input, &mut results);
        crate::contract_critical_command_not_fail_open::rule::check(&fail_open_input, &mut results);
    }

    crate::compat::finish(results)
}

/// `CALLS_VALIDATE_REPO_ID` constant.
const CALLS_VALIDATE_REPO_ID: &str = "g3rs-hooks/calls-validate-repo";
/// `DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID` constant.
const DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID: &str =
    "g3rs-hooks/dispatches-per-unit-validate-staged";
/// `DEDUPS_OWNING_UNITS_ID` constant.
const DEDUPS_OWNING_UNITS_ID: &str = "g3rs-hooks/dedups-owning-units";
/// `SKIPS_WHEN_NO_OWNING_UNIT_ID` constant.
const SKIPS_WHEN_NO_OWNING_UNIT_ID: &str = "g3rs-hooks/skips-when-no-owning-unit";
/// `NO_TOOLCHAIN_INVOCATION_ID` constant.
const NO_TOOLCHAIN_INVOCATION_ID: &str = "g3rs-hooks/no-toolchain-invocation";

/// `check_calls_validate_repo` function.
fn check_calls_validate_repo(
    input: &crate::inputs::RustHookCommandInput<'_>,
    results: &mut Vec<CompatResult>,
) {
    let found = any_resolved_command(input.parsed, |command| {
        command.command_name() == "g3rs"
            && command.args().first().map(String::as_str) == Some("validate-repo")
    });
    if found {
        results.push(
            CompatResult::from_parts(
                CALLS_VALIDATE_REPO_ID.to_owned(),
                G3Severity::Warn,
                "hook calls g3rs validate-repo".to_owned(),
                ".githooks/pre-commit invokes `g3rs validate-repo` for repo-level invariants."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }
    results.push(CompatResult::from_parts(
        CALLS_VALIDATE_REPO_ID.to_owned(),
        G3Severity::Error,
        "hook does not call g3rs validate-repo".to_owned(),
        ".githooks/pre-commit must invoke `g3rs validate-repo` to enforce repo-level invariants \
         (hook shape, tool presence, repo-wide topology, marker-pair completeness)."
            .to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `check_dispatches_per_unit_validate_staged` function.
fn check_dispatches_per_unit_validate_staged(
    input: &crate::inputs::RustHookCommandInput<'_>,
    results: &mut Vec<CompatResult>,
) {
    // Walk source lines, find a `g3rs validate --path <var> --staged` call inside a `while`
    // or `for` loop body.
    let lines: Vec<&str> = input
        .parsed
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect();
    let mut depth: usize = 0;
    let mut found = false;
    for line in &lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth += 1;
            continue;
        }
        if trimmed.starts_with("done") {
            depth = depth.saturating_sub(1);
            continue;
        }
        if depth > 0 && line_is_g3rs_validate_path_staged(trimmed) {
            found = true;
            break;
        }
    }
    if found {
        results.push(
            CompatResult::from_parts(
                DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID.to_owned(),
                G3Severity::Warn,
                "hook dispatches per-unit g3rs validate --staged".to_owned(),
                ".githooks/pre-commit invokes `g3rs validate --path <unit> --staged` from a discovery loop."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }
    results.push(CompatResult::from_parts(
        DISPATCHES_PER_UNIT_VALIDATE_STAGED_ID.to_owned(),
        G3Severity::Error,
        "hook does not dispatch per-unit g3rs validate --staged".to_owned(),
        ".githooks/pre-commit must invoke `g3rs validate --path <unit> --staged` from a discovery loop over staged files."
            .to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `line_is_g3rs_validate_path_staged` function.
fn line_is_g3rs_validate_path_staged(line: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(line);
    let mut i = 0;
    while i + 1 < words.len() {
        if words[i] == "g3rs" && words[i + 1] == "validate" {
            // Must contain --staged and --path among following args.
            let tail = &words[i + 2..];
            let has_staged = tail.iter().any(|w| w == "--staged");
            let has_path = tail
                .iter()
                .any(|w| w == "--path" || w.starts_with("--path="));
            return has_staged && has_path;
        }
        i += 1;
    }
    false
}

/// `check_dedups_owning_units` function.
fn check_dedups_owning_units(
    input: &crate::inputs::RustHookCommandInput<'_>,
    results: &mut Vec<CompatResult>,
) {
    let body = source_text(input.parsed);
    let dedup_present = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        line.contains("sort -u")
            || line.contains("sort --unique")
            || line.contains("uniq")
            || line.contains("awk '!seen")
    });
    if dedup_present {
        results.push(
            CompatResult::from_parts(
                DEDUPS_OWNING_UNITS_ID.to_owned(),
                G3Severity::Warn,
                "hook dedups owning units".to_owned(),
                ".githooks/pre-commit dedups owning units before invoking `g3rs validate`."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }
    results.push(CompatResult::from_parts(
        DEDUPS_OWNING_UNITS_ID.to_owned(),
        G3Severity::Error,
        "hook does not dedup owning units".to_owned(),
        ".githooks/pre-commit must dedup discovered owning Rust units (e.g. `sort -u`, `uniq`, or equivalent) before invoking `g3rs validate`."
            .to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `check_skips_when_no_owning_unit` function.
fn check_skips_when_no_owning_unit(
    input: &crate::inputs::RustHookCommandInput<'_>,
    results: &mut Vec<CompatResult>,
) {
    let body = source_text(input.parsed);
    // Lenient policy: when no owning unit found the hook must continue silently. Detect the
    // presence of any `[ -z "$X" ]` / `[ -n "$X" ]` test that gates the validate dispatch
    // OR a `continue` inside the discovery loop.
    let lenient = body.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        trimmed.starts_with("continue")
            || trimmed.contains("[ -z ")
            || trimmed.contains("[ -n ")
            || trimmed.contains("[[ -z ")
            || trimmed.contains("[[ -n ")
    });
    if lenient {
        results.push(
            CompatResult::from_parts(
                SKIPS_WHEN_NO_OWNING_UNIT_ID.to_owned(),
                G3Severity::Warn,
                "hook skips silently when no owning unit".to_owned(),
                ".githooks/pre-commit gates the per-unit Rust validate on having discovered an owning adopted unit."
                    .to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }
    results.push(CompatResult::from_parts(
        SKIPS_WHEN_NO_OWNING_UNIT_ID.to_owned(),
        G3Severity::Error,
        "hook does not skip when no owning unit".to_owned(),
        ".githooks/pre-commit must silently skip Rust-relevant staged files that have no owning adopted unit (lenient policy)."
            .to_owned(),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

/// `check_no_toolchain_invocation` function.
fn check_no_toolchain_invocation(
    input: &crate::inputs::RustHookCommandInput<'_>,
    results: &mut Vec<CompatResult>,
) {
    // Cargo invocation outside of allowed contexts (e.g. inside command-substitution
    // for environment metadata). The new contract: hook must not run `cargo` directly.
    // All cargo work is inside `g3rs validate --path X --staged`.
    let offending = first_offending_cargo_call(input.parsed);
    if let Some((line_no, command_text)) = offending {
        results.push(CompatResult::from_parts(
            NO_TOOLCHAIN_INVOCATION_ID.to_owned(),
            G3Severity::Error,
            "hook invokes cargo directly".to_owned(),
            format!(
                "`.githooks/pre-commit` invokes `{command_text}` directly. The hook must delegate all cargo work to `g3rs validate --path <unit> --staged`; no direct `cargo` invocations are allowed."
            ),
            Some(input.rel_path.to_owned()),
            Some(line_no),
            false,
        ));
        return;
    }
    results.push(
        CompatResult::from_parts(
            NO_TOOLCHAIN_INVOCATION_ID.to_owned(),
            G3Severity::Warn,
            "hook does not invoke cargo directly".to_owned(),
            ".githooks/pre-commit does not invoke cargo directly.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory(),
    );
}

/// `first_offending_cargo_call` function.
fn first_offending_cargo_call(
    parsed: &hook_shell_parser::types::ParsedShellScript,
) -> Option<(usize, String)> {
    for line in &parsed.executable_lines {
        if line.command_name == "cargo" {
            return Some((line.line_no, line.command_text.clone()));
        }
    }
    None
}

/// `source_text` function.
fn source_text(parsed: &hook_shell_parser::types::ParsedShellScript) -> String {
    parsed
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// `check_required_contracts_across_selected_surface` function.
fn check_required_contracts_across_selected_surface(
    inputs: &[G3RsHooksSourceChecksInput],
    results: &mut Vec<G3CheckResult>,
) {
    let Some(pre_commit) = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::PreCommit)
    else {
        return;
    };
    let mut content = script_content(pre_commit);
    if pre_commit_dispatches_modular_scripts(pre_commit) {
        for input in inputs.iter().filter(|input| {
            input.kind == G3RsHookScriptKind::Modular
                && input.rel_path.starts_with(".githooks/pre-commit.d/")
        }) {
            content.push_str(script_content(input).as_str());
        }
    }
    let parsed = hook_shell_parser::parse_script(&content);
    let input = crate::inputs::RustHookCommandInput {
        rel_path: pre_commit.rel_path.as_str(),
        parsed: &parsed,
        is_workspace_project: pre_commit.is_workspace_project,
        requirements: &pre_commit.requirements,
    };
    let mut contract_results = Vec::new();
    crate::required_contract_command_present::rule::check_with_validate_dispatch(
        &input,
        &mut contract_results,
    );
    results.extend(crate::compat::finish(contract_results));
}

/// `script_content` function.
fn script_content(input: &G3RsHooksSourceChecksInput) -> String {
    let mut content = String::new();
    for line in &input.parsed.source_lines {
        content.push_str(line.raw.as_str());
        content.push('\n');
    }
    content
}

/// `pre_commit_dispatches_modular_scripts` function.
fn pre_commit_dispatches_modular_scripts(input: &G3RsHooksSourceChecksInput) -> bool {
    input.parsed.executable_lines.iter().any(|line| {
        line.is_dispatcher_syntax && dispatcher_invokes_modular_directory(&line.command_text)
    })
}

/// `dispatcher_invokes_modular_directory` function.
fn dispatcher_invokes_modular_directory(command_text: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(command_text);
    let Some(command) = words.first().map(String::as_str) else {
        return false;
    };
    match command {
        "run-parts" => words
            .iter()
            .skip(1)
            .any(|word| word.trim_end_matches('/') == ".githooks/pre-commit.d"),
        "." | "source" => words
            .iter()
            .skip(1)
            .any(|word| word == ".githooks/pre-commit.d"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;

// Suppress dead-code warnings for the legacy ResolvedCommand type alias.
/// `_unused` function.
#[allow(dead_code)] // reason: parking-only reference to keep the legacy ResolvedCommand alias alive while callers migrate to the new command-query surface.
const fn _unused(_: &ResolvedCommand) {}
