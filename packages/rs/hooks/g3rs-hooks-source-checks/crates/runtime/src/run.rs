use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;
use hook_shell_parser::parse_script;

pub fn check(input: &G3RsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_single(input, true)
}

pub fn check_all(inputs: &[G3RsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = inputs
        .iter()
        .flat_map(|input| check_single(input, false))
        .collect::<Vec<_>>();

    check_required_contracts_across_selected_surface(inputs, &mut results);

    results
}

fn check_single(
    input: &G3RsHooksSourceChecksInput,
    include_required_contracts: bool,
) -> Vec<G3CheckResult> {
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
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
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::real_dispatcher_syntax_only::check(&dispatcher_input, &mut results);

        crate::fmt_step_present::check(&rust_input, &mut results);
        crate::clippy_step_present::check(&rust_input, &mut results);
        crate::cargo_deny_step_present::check(&rust_input, &mut results);
        crate::test_step_present::check(&rust_input, &mut results);
        crate::cargo_machete_step_present::check(&rust_input, &mut results);
        crate::duplication_tool_is_cargo_dupes::check(&rust_input, &mut results);
        crate::guardrail_validate_staged_present::check(&rust_input, &mut results);
        crate::clippy_denies_warnings::check(&rust_input, &mut results);
        crate::test_uses_workspace::check(&rust_input, &mut results);
        crate::gitleaks_step_present::check(&rust_input, &mut results);
        crate::cargo_dupes_step_present::check(&rust_input, &mut results);
        crate::cargo_dupes_excludes::check(&rust_input, &mut results);
        crate::config_changes_trigger_validation::check(&rust_input, &mut results);
        crate::contract_trigger_coverage::rule::check(&rust_input, &mut results);
        crate::shared_target_dir_present::check(&rust_input, &mut results);
        if include_required_contracts {
            crate::required_contract_command_present::rule::check(&rust_input, &mut results);
        }
    }

    crate::shell_safety::shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::valid_shebang::check(&executable_input, &mut results);
    crate::shell_safety::no_unconditional_exit_zero::check(&executable_input, &mut results);
    crate::shell_safety::no_bypass_instructions::check(&executable_input, &mut results);
    crate::workflow::merge_conflict_step_present::check(&executable_input, &mut results);
    crate::workflow::file_size_step_present::check(&executable_input, &mut results);
    crate::shell_safety::executable_command_context_only::check(&executable_input, &mut results);
    crate::shell_safety::concrete_lockfile_command::check(&executable_input, &mut results);
    crate::shell_safety::no_fail_open_wrappers::check(&fail_open_input, &mut results);
    crate::contract_critical_command_not_fail_open::rule::check(&fail_open_input, &mut results);

    crate::compat::finish(results)
}

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
    let parsed = parse_script(&content);
    let input = crate::inputs::RustHookCommandInput {
        rel_path: pre_commit.rel_path.as_str(),
        parsed: &parsed,
        is_workspace_project: pre_commit.is_workspace_project,
        requirements: &pre_commit.requirements,
    };
    let mut contract_results = Vec::new();
    crate::required_contract_command_present::rule::check(&input, &mut contract_results);
    results.extend(crate::compat::finish(contract_results));
}

fn script_content(input: &G3RsHooksSourceChecksInput) -> String {
    let mut content = String::new();
    for line in &input.parsed.source_lines {
        content.push_str(line.raw.as_str());
        content.push('\n');
    }
    content
}

fn pre_commit_dispatches_modular_scripts(input: &G3RsHooksSourceChecksInput) -> bool {
    input.parsed.executable_lines.iter().any(|line| {
        line.is_dispatcher_syntax && dispatcher_invokes_modular_directory(&line.command_text)
    })
}

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
