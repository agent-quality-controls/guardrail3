mod facts;
mod hook_shared_01_pre_commit_exists;
mod hook_shared_02_hooks_path_configured;
mod hook_shared_03_modular_directory_inventory;
mod hook_shared_04_dispatcher_pattern;
mod hook_shared_05_pre_commit_executable;
mod hook_shared_06_script_stats_inventory;
mod hook_shared_07_modular_scripts_inventory;
mod hook_shared_08_pre_commit_file_size_inventory;
mod hook_shared_09_local_override_inventory;
mod hook_shared_10_shell_error_handling;
mod hook_shared_11_valid_shebang;
mod hook_shared_12_modular_scripts_executable;
mod hook_shared_13_no_unconditional_exit_zero;
mod hook_shared_14_no_bypass_instructions;
mod hook_shared_15_merge_conflict_step_present;
mod hook_shared_16_file_size_step_present;
mod hook_shared_17_execution_trust;
mod hook_shared_18_executable_command_context_only;
mod hook_shared_19_real_dispatcher_syntax_only;
mod hook_shared_20_concrete_lockfile_command;
mod hook_shared_21_no_fail_open_wrappers;
mod inputs;
pub mod hook_shell;

use crate::hook_shell::parse_script;
use guardrail3_domain_report::CheckResult;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::FileSystem;
use guardrail3_outbound_traits::ToolChecker;

use self::facts::{HookScriptFacts, collect};
use self::inputs::{DispatcherSyntaxInput, ExecutableCommandContextInput, FailOpenWrapperInput};

pub fn check(
    fs: &dyn FileSystem,
    root: &std::path::Path,
    tree: &ProjectTree,
    _tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let facts = collect(fs, root, tree);
    let mut results = Vec::new();

    hook_shared_01_pre_commit_exists::check(facts.pre_commit.as_ref(), &mut results);
    hook_shared_02_hooks_path_configured::check(facts.hooks_path.as_deref(), &mut results);
    hook_shared_03_modular_directory_inventory::check(facts.has_modular_dir, &mut results);
    hook_shared_17_execution_trust::check(&facts.trust_risks, &mut results);
    hook_shared_07_modular_scripts_inventory::check(&facts.modular_scripts, &mut results);
    hook_shared_09_local_override_inventory::check(&facts.local_override_scripts, &mut results);
    hook_shared_12_modular_scripts_executable::check(&facts.modular_executable, &mut results);

    if let Some(pre_commit) = facts.pre_commit.as_ref() {
        let parsed = parse_script(&pre_commit.content);
        hook_shared_05_pre_commit_executable::check(
            &pre_commit.rel_path,
            facts.pre_commit_executable,
            &mut results,
        );
        hook_shared_04_dispatcher_pattern::check(
            &DispatcherSyntaxInput {
                rel_path: &pre_commit.rel_path,
                has_modular_dir: facts.has_modular_dir,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_06_script_stats_inventory::check(
            &pre_commit.rel_path,
            &pre_commit.content,
            &mut results,
        );
        hook_shared_08_pre_commit_file_size_inventory::check(
            &pre_commit.rel_path,
            &pre_commit.content,
            &mut results,
        );
        hook_shared_10_shell_error_handling::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_18_executable_command_context_only::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_11_valid_shebang::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_13_no_unconditional_exit_zero::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_14_no_bypass_instructions::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_15_merge_conflict_step_present::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_16_file_size_step_present::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_20_concrete_lockfile_command::check(
            &ExecutableCommandContextInput {
                rel_path: &pre_commit.rel_path,
                kind: pre_commit.kind,
                content: &pre_commit.content,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_19_real_dispatcher_syntax_only::check(
            &DispatcherSyntaxInput {
                rel_path: &pre_commit.rel_path,
                has_modular_dir: facts.has_modular_dir,
                parsed: &parsed,
            },
            &mut results,
        );
        hook_shared_21_no_fail_open_wrappers::check(
            &FailOpenWrapperInput {
                rel_path: &pre_commit.rel_path,
                executable_lines: &parsed.executable_lines,
            },
            &mut results,
        );
    }

    for script in &facts.modular_scripts {
        run_script_rules(script, &mut results);
    }

    results
}

fn run_script_rules(script: &HookScriptFacts, results: &mut Vec<CheckResult>) {
    let parsed = parse_script(&script.content);
    hook_shared_10_shell_error_handling::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_18_executable_command_context_only::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_11_valid_shebang::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_13_no_unconditional_exit_zero::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_14_no_bypass_instructions::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_15_merge_conflict_step_present::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_16_file_size_step_present::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_20_concrete_lockfile_command::check(
        &ExecutableCommandContextInput {
            rel_path: &script.rel_path,
            kind: script.kind,
            content: &script.content,
            parsed: &parsed,
        },
        results,
    );
    hook_shared_21_no_fail_open_wrappers::check(
        &FailOpenWrapperInput {
            rel_path: &script.rel_path,
            executable_lines: &parsed.executable_lines,
        },
        results,
    );
}
