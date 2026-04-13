use g3rs_hooks_source_checks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
    };
    let rust_input = crate::inputs::RustHookCommandInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        is_workspace_project: input.is_workspace_project,
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
    };
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::hook_shared_04_dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::hook_shared_19_real_dispatcher_syntax_only::check(
            &dispatcher_input,
            &mut results,
        );

        crate::hook_rs_01_fmt_step_present::check(&rust_input, &mut results);
        crate::hook_rs_02_clippy_step_present::check(&rust_input, &mut results);
        crate::hook_rs_03_cargo_deny_step_present::check(&rust_input, &mut results);
        crate::hook_rs_04_test_step_present::check(&rust_input, &mut results);
        crate::hook_rs_05_cargo_machete_step_present::check(&rust_input, &mut results);
        crate::hook_rs_07_duplication_tool_is_cargo_dupes::check(&rust_input, &mut results);
        crate::hook_rs_08_guardrail_validate_staged_present::check(&rust_input, &mut results);
        crate::hook_rs_09_clippy_denies_warnings::check(&rust_input, &mut results);
        crate::hook_rs_10_test_uses_workspace::check(&rust_input, &mut results);
        crate::hook_rs_11_gitleaks_step_present::check(&rust_input, &mut results);
        crate::hook_rs_12_cargo_dupes_step_present::check(&rust_input, &mut results);
        crate::hook_rs_13_cargo_dupes_excludes::check(&rust_input, &mut results);
        crate::hook_rs_16_config_changes_trigger_validation::check(
            &rust_input,
            &mut results,
        );
    }

    crate::shell_safety::hook_shared_10_shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_11_valid_shebang::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_13_no_unconditional_exit_zero::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_14_no_bypass_instructions::check(
        &executable_input,
        &mut results,
    );
    crate::workflow::hook_shared_15_merge_conflict_step_present::check(
        &executable_input,
        &mut results,
    );
    crate::workflow::hook_shared_16_file_size_step_present::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_18_executable_command_context_only::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_20_concrete_lockfile_command::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_21_no_fail_open_wrappers::check(
        &fail_open_input,
        &mut results,
    );

    crate::compat::finish(results)
}
